# rs-measure 后续正确性与测试契约加固设计

日期：2026-07-16

状态：已确认

## 1. 背景

rs-measure 已完成 Decimal 精确换算、默认关闭的 uom 桥接、单位元数据检查、
BTU/mmHg 修复和 Rust 文件组织加固。当前复审仍确认了一个实现不变量缺口、三个
公共契约测试缺口，以及 Serde、测试路径和 inline 分类问题。

本轮采用“契约优先、保持 API 兼容”的方案：修复内建因子的约分存储，增强解析与
外部宏的往返保证，以独立 SI 基准验证全部 uom 映射，并按项目规范补齐源码到测试的
镜像关系。

## 2. 目标

1. 所有内建 ConversionFactor 与公共构造器生成的因子都保存约分后的项。
2. 宏生成单位的规范符号和别名不得含首尾空白；空格分隔的 Measurement 文本可使用
   以 `.`、`+` 或 `-` 开头的单位符号，同时继续拒绝歧义的紧凑数字文本。
3. 每个内建 uom 映射都与 Decimal 定义和明确的 uom 基准单位独立一致。
4. 在真正的下游 crate feature 命名空间中验证默认构建和 uom 构建的导出宏行为。
5. 锁定 Measurement JSON 的必需字段、字符串 Decimal、未知单位和额外字段契约。
6. 为每个具体源码文件提供规范路径下的外部测试文件，纯别名只做最小契约测试。
7. 按 Rust inline 分类规则修正生成代码属性。

## 3. 非目标

- 不改变已有公共类型、枚举变体、方法名、模块路径或 JSON 三字段格式。
- 不把 uom 引入默认依赖，也不让 uom 参与持久化 Decimal 换算。
- 不新增运行时单位注册表或反射式全局单位目录。
- 不重新审计全部计量常量的外部标准来源；本轮只验证 Decimal 与现有 uom 映射一致。
- 不为纯类型别名复制转换、解析或 Serde 的完整行为测试。
- 不修改共享 `.rs-ci` 实现；项目特有的下游 fixture 检查放在 rs-measure 自有包装层。

## 4. 内建 ConversionFactor 约分

### 4.1 根因

ConversionFactor::new 会约分 Decimal 分子和分母，但 consts.rs 的 definition! 直接构造
私有字段。Revolution、两个角速度单位和 Spat 因此保存了未约分项，与公开 Rustdoc 和
转换算法“源、目标因子内部已规范化”的前提冲突。

### 4.2 设计

ConversionFactor 增加一个仅 crate 内可见的 const 整数比例构造器。该构造器：

1. 断言分子、分母为正数；
2. 在 const 上下文计算 i128 最大公约数；
3. 约分后验证两个值都能放入 Decimal 的 96 位有效范围；
4. 构造 scale 为零的 Decimal 项并返回 ConversionFactor。

definition! 只能调用该构造器，不再直接写 ConversionFactor 字段。运行时 new 继续支持
任意正 Decimal，并与 const 构造器共享最大公约数语义。

### 4.3 测试

先增加失败测试，证明内建 Revolution 因子等于从其数学等价值构造的规范因子，并验证
Decimal::MAX 在内建因子与等价外部定义之间保持恒等。随后更新 4 个 golden 比例为约分
形式，并增加遍历所有内建 family 的约分不变量断言。

## 5. 单位元数据与 Measurement 解析往返

### 5.1 固定契约

- 规范符号和别名仍可包含 Unicode 计量字符。
- 规范符号和别名必须等于自身 trim 后的文本，即不得带首尾 Unicode 空白。
- Unit::parse_strict 和 Unit::parse_lenient 仍忽略调用方输入的首尾空白。
- Measurement 的空格分隔形式 `<decimal> <unit>` 可承载任意合法非空单位符号，包括
  以 `.`、`+` 或 `-` 开头的符号。
- Measurement 的紧凑形式 `<decimal><unit>` 继续拒绝以 `.`、`+` 或 `-` 开始的后缀，
  避免把 `1+2`、`1.2.3` 等非法数字误解释为单位。

### 5.2 实现

const 元数据校验增加“已 trim”判断，宏生成 family 在编译期拒绝违规字面量。为避免
依赖不明确的 const 标准库能力，校验函数按 Unicode White_Space 属性显式识别边界字符：
U+0009..U+000D、U+0020、U+0085、U+00A0、U+1680、U+2000..U+200A、U+2028、
U+2029、U+202F、U+205F 和 U+3000。手工 Unit 的 assert_unit_family_valid 使用运行时
trim 检查同一可观察契约，并给出明确失败。

split_measurement_parts 在 trim 单位后缀前记录数值与单位之间是否存在空白。只有没有
分隔空白的紧凑形式才应用保留前缀拒绝规则。这样 Display 始终产生的空格分隔文本可被
FromStr 和 Serde 稳定读回。

### 5.3 测试

- define_unit_family! Rustdoc 增加首尾空白符号和别名的 compile_fail 示例。
- 外部 fixture family 增加以 `+` 开头的规范符号，验证 Unit、Measurement Display、
  FromStr 和 Serde 往返。
- 保留现有紧凑非法数字测试，证明解析边界没有放宽。

## 6. uom 独立映射验证

### 6.1 测试模型

现有“同一单位转出再转回”只验证映射自洽。新测试对每个具体 family 展开测试宏，并从
UnitDefinition::base() 找出 rs-measure 基准 unit。uom Quantity 的公开 value 字段存储
SI 基准值，因此测试不需要复制生产映射表或逐 family 指定 uom unit 类型。

对 family 中每个变体执行两条独立路径：

1. Decimal 路径把 `1 source unit` 精确转换到 rs-measure 基准 unit；uom 路径把同一输入
   转成 Quantity 后直接读取 SI 基准 value，两者按统一相对容差比较。
2. 先通过零值基准 Measurement 获得同维度 Quantity，再把其公开 value 替换为独立的
   exact_base_value，转换到原变体后必须近似等于 Decimal 的 1。

该模型能同时发现源变体映射错误和基准变体映射错误，不依赖同一错误映射的对称往返。
现有 BTU 和 mmHg 直接 oracle 保留，作为关键计量语义的可读回归测试。

### 6.2 下游 feature fixture

新增两个不加入主 workspace、也不打入发布包的 fixture crate：

- 默认 fixture：依赖 qubit-measure 默认 feature，自身不依赖 uom；宏调用携带不会被
  名称解析的 uom token，证明默认分支能丢弃桥接元数据。
- uom fixture：依赖以 `features = ["uom"]` 启用 qubit-measure，但 fixture 自身不声明
  名为 `uom` 的 feature；它定义带 uom 映射的外部 family，并用 trait bound 证明
  UomUnit 实现存在。

fixture 放在根目录 `fixtures/`，Cargo.toml 的 include 白名单不会发布它们。项目自有
align-ci.sh 负责格式化 fixture，ci-check.sh 在进入共享 rs-ci 流程前用 Rust 1.94 检查
两个 manifest。共享 `.rs-ci` 内容保持不变。

为证明 fixture 确实能捕获回归，测试阶段临时把隐藏宏改为在展开结果上使用下游
`cfg(feature = "uom")`，确认 uom fixture 失败后立即恢复，再确认通过；该临时变更不进入
最终差异。

## 7. Measurement Serde 边界

在 Measurement 对应的镜像测试文件中增加表驱动断言：

- 缺少 quantity、value 或 unit 时分别失败；
- value 为 JSON number 而非 string 时失败；
- 未知 unit 返回包含 quantity 上下文的错误；
- 额外字段被忽略并成功反序列化；
- quantity 不匹配和别名规范化的现有行为保持不变。

不增加 deny_unknown_fields，也不改变 MeasurementWire 字段或序列化输出。

## 8. 源码与测试镜像

### 8.1 目录策略

严格按 `src/<path>/<stem>.rs` 对应 `tests/<path>/<stem>_tests.rs`：

- 新增 tests/measurement/ 及其 mod.rs，为 57 个持久化类型别名各建一个测试文件；
- 新增根级 lib_tests.rs、consts_tests.rs、measurement_tests.rs、private_tests.rs 和
  unit_tests.rs；
- 为 measure 下现有 internal、units/internal 和聚合源码补齐镜像测试文件；
- 已位于正确路径的 tests/measure/ 和 tests/measure/units/ 测试保持原位；
- 将当前路径错误但内容可复用的测试移动到规范路径，避免重复执行。

### 8.2 纯别名测试

每个 measurement 别名文件只验证：

1. `measurement::<Alias>` 公共路径可用；
2. 它可以接收对应 `unit::<Family>` 并构造 Measurement；
3. quantity_name 与 family 契约一致。

共享断言放入 tests/measurement/support/；各别名文件保留独立测试入口和明确类型标注。
不复制精确换算、解析或 Serde 测试。

## 9. inline 分类

- 宏生成的 FromStr::from_str 是纯转发，使用 #[inline(always)]。
- value_from_uom_approx 包含按全部变体展开的 match，使用 #[inline]，不再强制内联。
- 不调整其他已符合当前 inline 决策表的函数。

## 10. TDD 与验证顺序

每项行为修复遵循 RED-GREEN-REFACTOR：先写聚焦测试并确认因目标缺口失败，再做最小
实现并运行同一测试。对已经正确但缺少保护的 feature fixture，使用临时回归变异确认
测试会失败；对纯目录映射和配置类变更，使用样式检查和编译检查验证。

全部改动完成后严格执行：

1. ./align-ci.sh
2. ./ci-check.sh
3. 仅当 CI 报告覆盖率低于阈值时执行 ./coverage.sh json

align-ci 可能修改文件，运行后必须重新检查差异。任何失败都只修复本轮范围内原因并从
相应步骤重跑。未经用户另行授权，不执行 git add、git commit 或 git push。

## 11. 兼容性与风险

- 公共 API 和模块路径保持不变；内建 factor getter 的 4 个返回文本会变为数学等价的
  约分形式，这是不变量修复带来的可观察变化。
- 首尾空白元数据从“宏可编译但无法稳定往返”变为编译期错误，属于收紧无效输入。
- 空格分隔 Measurement 对特殊前缀符号的接受范围扩大，但紧凑数字语法保持严格。
- uom 全量 oracle 直接读取 Quantity 的 SI 基准 value，不复制生产映射表；测试宏仍按
  具体 family 展开，以便编译器解析关联 Quantity 的具体字段。
- 新增镜像测试文件数量较多，但每个文件职责单一，且不引入新的生产抽象。
