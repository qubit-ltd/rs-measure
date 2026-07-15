# rs-measure 正确性、可选 uom 与 Rust 风格加固设计

日期：2026-07-15

状态：待书面审核

## 1. 背景

rs-measure 0.3 已经把持久化换算从 f64 迁移到 Decimal，并把精确 Unit
与近似 UomUnit 分开。此次加固处理评审中确认的两个正确性缺陷、真正可选的
uom 构建依赖、外部单位族的不变量校验、缺失的计量语义说明，以及整个 crate
内已识别的 Rust 组织、Rustdoc、方法顺序和内联属性问题。

当前确认的问题如下：

1. 三个 International Table BTU 变体映射到了 uom 的通用 BTU 单位。
2. ConversionFactor 公开允许未约分比例，但转换过程可能因未约分的中间值产生
   虚假溢出或舍入。
3. uom 是普通依赖，因此所谓可选桥接只存在于 trait 边界，不存在于构建边界。
4. 宏生成和手工 Unit 实现缺少统一的不变量检查。
5. 别名冲突优先级、外部实现责任及部分计量常量的具体语义没有完整写入文档。
6. 集中式 measurement 类型别名、私有 MeasurementWire、合并测试、Rustdoc、
   固有方法顺序和 inline 属性不完全符合当前 Rust 规范。

本设计不处理 rs-datatype、rs-serde 或 rs-retry 的 duration 集成；用户已明确要求
忽略真实下游采用问题。

## 2. 目标

1. 用先失败、后修复的回归测试证明并修复三个 BTU 映射。
2. 保证数学上等价的未约分因子不会因 2/2 一类表示产生虚假溢出。
3. 让默认构建完全不依赖 uom，并通过显式 uom feature 启用近似桥接。
4. 对宏生成单位执行编译期元数据检查，对手工 Unit 实现提供公开测试断言。
5. 在 Rustdoc、英文 README 和中文 README 中写清稳定公共契约和已知计量语义。
6. 使 rs-measure 源码和测试符合项目 Rust 组织、文档、方法顺序与内联规范。
7. 保留已有 Decimal API、unit 枚举和 measurement、unit 公共模块路径。

## 3. 非目标

- 不新增运行时单位注册表。
- 不封闭 Unit trait，也不强制外部单位通过宏定义。
- 不把 uom 用于持久化 Decimal 换算。
- 不重做所有计量常量的独立标准审计。
- 不引入 rs-datatype 或其他 sibling crate 依赖。
- 不改变现有 JSON 三字段格式或严格、宽松解析入口。
- 不因整理文件而重命名现有公共类型、枚举变体或方法。

## 4. 方案选择

采用分层校验方案：

- define_unit_family 宏生成完整枚举和 all 列表，并在 const 上下文检查静态元数据。
- 公开 assert_unit_family_valid 泛型测试辅助函数，检查手工实现可观察的不变量。
- 文档明确说明 Rust 无反射能力，辅助函数无法证明手工 enum 的所有变体都已列入
  all；这一点仍由实现者负责。

不采用仅宏检查方案，因为它不能保护公开允许的手工 Unit 实现。也不采用运行时注册表
或 validated builder，因为它们会扩大 API、引入运行时状态并改变当前编译期扩展模型。

## 5. BTU uom 映射修复

### 5.1 回归测试

先在 uom 桥接测试中增加直接对 SI 基准单位的断言，而不是继续使用同一错误映射往返：

- Energy::BritishThermalUnitInternationalTable 的 1 Btu (IT) 必须得到
  1055.056 joule。
- HeatCapacity::BritishThermalUnitInternationalTablePerDegreeFahrenheit 的
  1 单位必须得到 1899.1008 joule per kelvin。
- SpecificHeatCapacity::BritishThermalUnitInternationalTablePerPoundDegreeFahrenheit
  的 1 单位必须得到 4186.800307941667 joule per kilogram kelvin，并使用与
  uom 0.38 IT 定义相匹配的容差。

测试先在现有实现上运行并确认因系数不匹配而失败。失败必须是数值断言失败，而不是导入、
编译或测试配置错误。

### 5.2 最小修复

分别把映射改为 uom 的 btu_it、btu_it_per_degree_fahrenheit 和
btu_it_per_pound_degree_fahrenheit。保留 rs-measure 自身的 Decimal 定义、枚举名、
规范符号和别名不变。

修复后先运行三个聚焦回归测试，再运行完整 uom 桥接测试，确认直接基准和全量往返同时
成立。

## 6. Decimal 比例规范化与交叉约分

### 6.1 根因

当前 ConversionFactor 保存正 Decimal 分子和分母，但不规范化。转换时先组合源分子与
目标分母、源分母与目标分子，随后尝试乘除。对于 Decimal::MAX 和 2/2，数学结果为原值，
但 MAX 乘 2 会溢出；改为先除 2 又会因中间的二分之一无法表示而舍入。

因此问题不只是运算顺序，而是等价比例没有在进入危险算术前消去公共因子。

### 6.2 失败测试

先增加以下回归行为：

- ConversionFactor::new(2, 2) 与 identity 定义之间转换 Decimal::MAX，结果必须精确
  等于 Decimal::MAX。
- 4/6 和 2/3 构造出的 ConversionFactor 必须具有相同的规范表示。
- 源、目标因子存在可交叉消去项时，在组合乘法前消去公共因子，并保留期望结果。
- 原有 5/9、offset、循环小数和显式 scale 行为不得改变。

第一个测试必须先在现有实现上以 ArithmeticOverflow 失败。

### 6.3 因子规范化

ConversionFactor::new 在正数校验后规范化两个 Decimal：

1. 读取正 Decimal 的 mantissa 和 scale。
2. 用 mantissa 的最大公约数同时约分。
3. 从两边 scale 中减去共同的最小 scale。
4. 用约分后的 mantissa 和剩余 scale 重建 Decimal。

该过程不需要把一边乘以十的高次幂，因此不会为了获得规范表示而引入新的中间溢出。
例如 2/2 变为 1/1，4/6 变为 2/3，0.4/0.1 也变为 4/1；1/0.1 可以保持为
1/0.1，因为它已经没有可同时消去的 mantissa 或 scale。

ConversionFactor 的 Rustdoc 从未约分比例改为构造时规范化的正 Decimal 比例，并说明
PartialEq 比较规范表示。

### 6.4 转换期交叉约分

源到目标的组合比例为：

source.numerator × target.denominator
除以
source.denominator × target.numerator。

在执行两次 checked_mul 前，分别约分 source.numerator 与 target.numerator，以及
target.denominator 与 source.denominator。源因子内部和目标因子内部已经由构造函数
规范化，因此这两次交叉约分覆盖剩余的公共项。

约分后再组合分子分母并调用 apply_ratio。若约分后的组合项仍超出 Decimal 表示范围，
保留按两个规范比例依次应用的受检 fallback。所有失败继续返回
MeasurementError::ArithmeticOverflow，不引入 panic 或 f64。

## 7. 默认关闭的 uom feature

### 7.1 Cargo 契约

Cargo.toml 增加：

- default 为空。
- uom feature 只启用 dep:uom。
- uom 依赖标记 optional，现有 default-features 与 f64、si、std 配置保持不变。

默认构建提供 Decimal、Measurement、Unit、ConversionFactor、UnitDefinition、所有内置
单位 family 和精确转换。默认构建不导出 UomUnit，也不提供 to_uom_approx 与
from_uom_approx。

启用 uom feature 后恢复以上近似桥接 API。此变化无需兼容旧的默认启用行为。

### 7.2 宏条件实现

不能在导出宏的展开结果中直接使用 cfg(feature = "uom")，因为该条件会在下游 crate
的 feature 命名空间求值。实现改为由 qubit-measure 自身按 feature 定义两个同名隐藏
辅助宏：

- feature 开启时，辅助宏生成 UomUnit impl。
- feature 关闭时，辅助宏接受并丢弃 uom 映射 token。

define_unit_family 始终生成精确 Unit 实现，再调用该隐藏辅助宏。内置单位文件中的 uom
imports 使用 crate 自身的 cfg 条件；feature 关闭时，传给宏但未展开的类型 token 不会
参与名称解析。

### 7.3 测试与文档

所有 uom imports 和桥接测试移入 feature-gated 的 uom_unit_tests 模块。默认测试验证
纯 Decimal API；all-features 测试验证桥接和三个 BTU 回归。

英文和中文 README 同步给出两种依赖写法：

- 默认精确核心，不启用 feature。
- 使用 features = ["uom"] 启用近似桥接。

README 和 UomUnit Rustdoc 明确说明该 feature 默认关闭、桥接经过 f64、API 只在 feature
开启时存在。示例应在默认 rustdoc 和 all-features rustdoc 中都能编译。

## 8. Unit 不变量与生成期检查

### 8.1 固定策略

quantity 标识必须是非空 ASCII snake_case：

- 第一个字符必须是 a 到 z。
- 后续字符只能是 a 到 z、0 到 9 或单个下划线。
- 不允许前导、尾随或连续下划线。

单位元数据遵循：

- family 至少包含一个单位。
- 每个规范符号非空，并在 family 内唯一。
- 每个别名非空，并在所有变体的别名集合中唯一。
- 允许别名等于另一个变体的规范符号。
- 当输入同时是规范符号和别名时，规范符号拥有者优先。

这一策略必须同时写入 Unit、define_unit_family、parse_lenient Rustdoc，以及英文、中文
README 的外部扩展章节。

### 8.2 宏生成期检查

define_unit_family 展开一个匿名 const 检查块，把 quantity、规范符号切片和扁平别名切片
传给文档隐藏的 const 校验函数。校验函数只使用 const 可用的字节比较、循环和静态 panic
消息，兼容 crate 声明的 Rust 1.94。

宏自动生成 all，因此宏定义 family 的变体完整性由展开结构保证。重复规范符号、重复
别名、空文本或非法 quantity 会在使用宏的 crate 编译时失败。

define_unit_family Rustdoc 增加 compile_fail 示例，分别覆盖重复规范符号与重复别名；
正常示例覆盖允许的 alias-to-canonical 冲突。

### 8.3 手工 Unit 测试辅助函数

crate 根和 unit 模块导出：

pub fn assert_unit_family_valid<U: Unit>()

该函数标记 track_caller，并检查：

- all 非空且不包含重复的单位值。
- quantity、规范符号和别名符合静态策略。
- 每个 all 项的 definition 成功。
- parse_strict 对规范符号返回原单位。
- parse_lenient 对规范符号返回原单位。
- 普通别名返回声明它的单位。
- alias-to-canonical 冲突返回规范符号拥有者。

函数用于测试，因此发现错误时 panic，并在信息中包含 quantity、符号或索引上下文。
Rustdoc包含 Panics 和 Examples，并明确说明它只能检查 all 中列出的值；手工 enum 是否
遗漏变体无法通过稳定 Rust 反射能力证明。

内置所有 family 和现有手工外部 Unit 都调用该辅助函数。负面辅助 fixture 覆盖重复 all、
重复规范符号和重复别名。

## 9. Rustdoc 与计量语义

### 9.1 API 文档

对所有本次涉及的类型、别名、字段、函数和方法进行逐项审计：

- 有参数的方法补 Arguments。
- 有非 unit 返回值的方法补 Returns。
- 返回 Result 的方法补完整 Errors。
- 测试断言辅助函数补 Panics。
- getter、薄包装也保留简洁但完整的 Rustdoc。
- 私有 split、parse、ratio 及 const 校验辅助函数说明输入、返回状态和边界。

宏生成的 enum variant 继续从单位 family 文件携带 Rustdoc。此前含糊的 year、
calorie、BTU、horsepower、US volume、volume rate、mass density、heat capacity 和
specific heat capacity 变体改为写出精确定义限定词和规范符号。

### 9.2 mmHg 语义

现有 MillimeterOfMercury 数值 20265/152 Pa 等于 101325/760 Pa，即精确 Torr 等价值。
本次不重命名公共变体，也不改变数值。Pressure variant Rustdoc 和双语 README 的计量
说明必须明确：

- 当前库采用精确 Torr 等价值。
- 该值不同于部分标准表使用的 conventional 133.3224 Pa mmHg。
- 调用方需要 conventional mmHg 时必须定义外部 unit，而不能假定当前变体采用该值。

文档附上 NIST SP 811 Chapter 5 和 Appendix B.9 的来源链接。其余常量不宣称已完成全面
NIST 或 BIPM 审计。

## 10. Rust 文件组织与测试布局

### 10.1 类型和别名

私有 MeasurementWire 从 measure/measurement.rs 拆到
measure/internal/measurement_wire.rs，由无业务类型的 measure/internal.rs 声明和受限
重导出。Measurement 的公共路径不变。

measurement.rs 保留为公共聚合模块，但每个公开 measurement 类型别名放到
measurement 目录下各自的 snake_case 文件，再由 measurement.rs 重导出。Voltage 保留
为 ElectricPotential 的人体工学别名并拥有独立文件。外部路径仍是
qubit_measure::measurement::Length 等。

unit.rs 保留为单位聚合和重导出模块；UomUnit 的重导出受 feature 控制。它不新增重复
类型定义。

### 10.2 测试

uom 桥接测试从 measurement_tests 拆到与 uom_unit 源模块对应的
uom_unit_tests。单位 family 的定义、解析和 golden 数据测试按 family 拆入
tests/measure/units 目录，并通过聚合模块注册；共享断言留在无类型的测试 support 模块。

大型原测试文件缩减为其对应核心源模块的行为测试或聚合入口，不保留与其他源文件职责
混杂的测试。拆分只改变测试组织，不改变断言数据。

新增测试 fixture 中的每个 struct、trait 和 enum 独占文件。所有新增 Rust 文件复制项目
标准版权头。

### 10.3 方法顺序与内联

每个固有 impl 按以下顺序整理：

1. 所有构造器和解析工厂。
2. 构造器内部按 pub、受限、private。
3. 其余方法按 pub、受限、private。
4. 同一可见性组内保持 getter 在相关 setter 或派生操作之前。

Measurement::parse_strict 移到普通 getter 和转换方法之前；
from_uom_approx 在受 UomUnit 约束的 impl 中放在 to_uom_approx 之前。

inline 按函数体分类：

- getter、setter、纯转发和极薄包装使用 inline(always)。
- 其他短小、无循环、低分支函数使用 inline。
- 循环、解析器、验证器和复杂转换函数不添加 inline。

这项审计覆盖 src 下所有手写函数、方法和宏生成方法；不修改依赖、vendored 或 rs-ci
子模块代码。

## 11. TDD 实施顺序

1. 写三个 BTU SI 基准失败测试并确认失败。
2. 修复三个映射并确认聚焦测试转绿。
3. 写 Decimal::MAX 与 2/2 失败测试并确认失败。
4. 实现因子规范化和交叉约分并确认转换测试转绿。
5. 写 Unit 元数据校验测试和 compile_fail Rustdoc，再实现 const 与运行时校验。
6. 增加 uom feature 配置和条件边界，分别验证默认构建与 all-features 构建。
7. 同步双语 README、Rustdoc 和计量语义。
8. 在现有行为测试保护下完成文件组织、方法顺序和 inline 整理。
9. 运行项目规定的最终验证序列。

每个 bug 的生产修复前都必须看到对应测试因预期行为差异而失败。纯 Cargo feature 配置
通过依赖树和默认、all-features 构建的前后命令验证，不伪造无法表达真实契约的单元测试。

## 12. 验证

实施期间运行聚焦 cargo test 命令，保存两个 bug 的 red 和 green 结果。最终从
rs-measure 仓库根目录严格按以下顺序运行：

1. ./align-ci.sh
2. ./ci-check.sh
3. 仅当 ci-check 明确报告覆盖率低于阈值时运行 ./coverage.sh json

align-ci 可能修改文件，因此运行后必须重新检查 diff。ci-check 应覆盖默认 feature、
all-features、Clippy、rustdoc、doctest、格式化及项目自定义风格检查；若实际脚本没有
覆盖某一 feature 组合，则使用仓库已有 cargo-feature-check.sh 所规定的命令补充，不猜测
自定义参数。

## 13. 验收标准

1. 三个 BTU IT 桥接使用 uom 对应的 IT 单位，并有直接 SI 基准回归测试。
2. Decimal::MAX 经 2/2 转换精确返回原值，不出现虚假 ArithmeticOverflow。
3. ConversionFactor 文档与实现都采用规范表示，组合比例先交叉约分。
4. 默认 Cargo 构建不包含 uom；启用 uom feature 后所有近似 API 和测试可用。
5. 宏对 quantity、规范符号和别名执行编译期检查。
6. 手工 Unit 实现可通过公开测试辅助函数验证可观察不变量。
7. alias-to-canonical 冲突策略在 Rustdoc 和双语 README 中一致。
8. mmHg 当前采用精确 Torr 等价值的语义在 Rustdoc 和双语 README 中明确。
9. 所有公共路径保持兼容，除默认关闭 uom 导致的明确 feature opt-in 变化。
10. 源码类型组织、测试映射、文档、方法顺序和 inline 属性通过项目检查。
11. 最终验证命令及真实退出状态被完整记录，未运行的覆盖率命令不宣称已运行。
