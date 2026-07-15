# rs-measure Decimal 精确换算与可扩展单位设计

日期：2026-07-15  
状态：已确认

## 1. 背景

`rs-measure` 当前用 `rust_decimal::Decimal` 保存持久化数值，但单位换算经过
`Decimal -> f64 -> uom -> f64 -> Decimal`。因此，即使输入和目标单位之间存在精确的
十进制关系，换算也可能先丢失 Decimal 的有效数字。

`uom 0.38` 的 quantity 类型虽然对数值类型泛型化，但其存储类型和单位换算实现由固定
feature 列表生成，不包含 `rust_decimal::Decimal`。`BigRational` 也不能直接解决问题，
因为 `uom` 的非浮点单位系数仍先作为 `f64` 表达式求值，再通过 `from_f64` 转换。

本设计把持久化单位换算和量纲计算明确分层：`rs-measure` 自己负责 Decimal 换算，
`uom/f64` 仅作为显式标注为近似的计算桥接层。

参考资料：

- [uom 支持的存储类型](https://docs.rs/uom/latest/uom/macro.storage_types.html)
- [uom 单位换算宏实现](https://docs.rs/uom/latest/src/uom/unit.rs.html)
- [rust_decimal Decimal 表示范围](https://docs.rs/rust_decimal/latest/rust_decimal/struct.Decimal.html)

## 2. 目标

1. `Measurement::convert_to()` 的输入、换算因子、中间值和输出都不经过 `f64`。
2. 用精确的十进制有理数表达单位系数，支持 `5/9` 等非终止十进制比例。
3. 允许调用方逐次指定小数位数和舍入策略，并提供可在运行时修改的进程级默认配置。
4. 在 JSON 中加入并校验稳定的 `quantity` 字段。
5. 将含糊的单位定义改成显式定义，同时保留可控的宽松解析入口。
6. 允许下游 crate 定义新的 quantity family 和单位，而不依赖 crate 私有宏。
7. 用独立基准数据补足精度、语义、Serde、扩展性和并发配置测试。
8. 同步完善英文和简体中文 README，并提供 0.2 到 0.3 的迁移说明。

## 3. 非目标

- 不重新实现完整的 `uom` 量纲代数、数学函数或派生量类型系统。
- 不让 `uom` 直接以 `rust_decimal::Decimal` 为存储后端。
- 不保证循环小数、无理数或实验测得常数的数学无限精度。
- 不引入运行时单位注册表；外部扩展是编译期扩展。
- 不把日历年、月等日历概念建模为固定物理时长。
- 不兼容 0.2 的 JSON wire format、trait 形状或歧义单位枚举名。

## 4. 总体架构

核心类型按职责拆分：

| 组件 | 职责 |
| --- | --- |
| `Measurement<U>` | 保存 `Decimal` 数值和具体单位，负责换算、文本解析及 Serde |
| `Unit` | 描述 quantity、规范符号、别名和 Decimal 换算定义；不依赖 `uom` |
| `UomUnit` | 可选的 `uom/f64` 近似桥接；只有可映射的单位 family 才实现 |
| `ConversionFactor` | 以 Decimal 分子和分母表示正的换算比例 |
| `UnitDefinition` | 组合换算比例和加法偏移量 |
| `ConversionOptions` | 描述最终输出 scale 和舍入策略 |
| 全局配置 | 用 `parking_lot::Mutex<ConversionOptions>` 保存进程级默认值 |
| `define_unit_family!` | 为内置和外部单位生成枚举、解析、Serde 及 Decimal 定义 |

建议将配置、换算模型、核心 trait 和 `uom` 桥接放在独立模块中，避免继续扩大当前
`units.rs` 宏文件的职责。现有 quantity-specific unit 文件仍保持一类 quantity 一个文件。

## 5. Decimal 换算模型

### 5.1 单位定义

每个单位相对于本 quantity 基准单位的定义为：

```text
coefficient = numerator / denominator
base = (value + offset) * coefficient
```

`numerator`、`denominator` 和 `offset` 都使用 `Decimal`。分子与分母分开保存，不能预先
执行 Decimal 除法；因此 Fahrenheit 的 `5/9` 等比例不会在单位定义阶段被截断。

`ConversionFactor` 和 `UnitDefinition` 使用经过校验的构造入口。系数必须为正，分母不能
为零。内置宏在生成阶段保证定义有效；手工实现的外部单位若返回非法定义，换算返回
`InvalidUnitDefinition`，而不是 panic。

`UnitDefinition::convert_value_to(value, target, options)` 公开底层 Decimal 换算能力，
便于外部扩展在不构造 `Measurement` 的情况下验证和复用其单位定义；
`Measurement::convert_to_with_options()` 委托给该方法。

有限十进制定义按其十进制值精确保存。角度、物理常数等本身不能以有限 Decimal 精确
表示的单位，采用文档明确列出的权威十进制定义；这类定义是确定的有限精度常量，不宣称
等于数学上的无限精度值。

### 5.2 换算公式

从源单位换到目标单位时，先合并两个比例：

```text
result =
    (value + source.offset)
    * source.numerator
    * target.denominator
    / source.denominator
    / target.numerator
    - target.offset
```

实现应在不改变结果语义的前提下约分和调整运算顺序，尽量只进行一次不可整除除法并降低
中间溢出风险。所有算术使用 `Decimal::checked_*`。若中间值超出 Decimal 范围，返回
`ArithmeticOverflow`。

对于相同源、目标单位：

- `scale = None` 时原样保留输入 Decimal，包括其 scale；
- `scale = Some(n)` 时仍执行最终舍入并把结果设置为 `n` 位小数。

### 5.3 精度与舍入语义

`ConversionOptions` 的语义为：

```rust
pub struct ConversionOptions {
    scale: Option<u32>,
    rounding: RoundingStrategy,
}
```

- `scale = None`：不额外降低结果精度，保留 Decimal 在本次运算中可表示的最高精度。
- `scale = Some(n)`：最终结果按指定策略舍入，并保留恰好 `n` 位小数。
- `n` 的合法范围是 `0..=Decimal::MAX_SCALE`，当前即 `0..=28`。
- 初始默认值是 `scale = None`、`RoundingStrategy::MidpointNearestEven`。
- 对循环小数，“最高精度”仍表示 Decimal 能承载的有限结果，而不是数学无限精度。
- `rounding` 用于最终的显式 scale 舍入；`scale = None` 时不额外执行降精度舍入。
- 若数值幅度使 Decimal 无法同时保存结果和恰好 `n` 位小数，返回
  `ArithmeticOverflow`，不得静默减少请求的 scale。

字段保持私有，通过构造函数或 builder 创建，确保无效 scale 不会进入全局配置。
crate 根模块重新导出 `Decimal` 和 `RoundingStrategy`，调用方无需依赖内部模块路径。

## 6. 进程级默认配置

公开 API：

```rust
pub fn default_conversion_options() -> ConversionOptions;

pub fn set_default_conversion_options(
    options: ConversionOptions,
) -> ConversionOptions;
```

全局值由 `parking_lot::Mutex<ConversionOptions>` 保护。setter 原子替换配置并返回旧值，
便于调用方在需要时恢复。

`Measurement::convert_to()` 在换算开始时锁定、复制并立即释放默认配置；后续计算只使用
这份快照。并发 setter 因此只能使一次换算看到完整的旧配置或完整的新配置。

`Measurement::convert_to_with_options()` 始终使用显式配置，不读取全局状态。测试和对结果
可重复性要求高的业务代码应优先使用显式配置。

## 7. 公共 API

### 7.1 核心 Unit

`Unit` 不再携带 `uom` quantity 关联类型。它负责：

- 稳定的 `QUANTITY` 机器标识；
- 返回所有单位变体；
- 返回唯一规范符号和宽松解析别名；
- 通过 `definition()` 返回 `Result<UnitDefinition, MeasurementError>`；
- 提供 `parse_strict()` 和 `parse_lenient()`。

`Display` 始终输出规范符号。`FromStr` 委托给 `parse_lenient()`，保持面向人类输入时的易用性。
内置宏生成的 `definition()` 总是成功；手工扩展可以把非法定义报告为
`InvalidUnitDefinition`，核心换算统一传播该错误。

### 7.2 Measurement 换算

公开两个换算入口：

```rust
measurement.convert_to(target)
measurement.convert_to_with_options(target, options)
```

前者使用进程默认配置，后者使用显式配置。两者都只调用 Decimal 换算层。

### 7.3 uom 近似桥接

`UomUnit: Unit` 提供 `uom` quantity 关联类型和桥接实现。`Measurement<U>` 仅在
`U: UomUnit` 时提供：

```rust
measurement.to_uom_approx()
Measurement::from_uom_approx(quantity, unit)
```

名称中的 `approx` 是公共契约的一部分，用来明确 `Decimal <-> f64` 可能丢失精度。
从 `uom` 返回 NaN、无穷大或无法转成 Decimal 的值时，返回 `DecimalConversion`。

## 8. 外部扩展

公开 `Unit`、`ConversionFactor`、`UnitDefinition` 和 `define_unit_family!`。外部 crate 可以
使用宏定义完整的 quantity family，也可以手工实现 `Unit`。

宏的声明模型为：

```rust
define_unit_family! {
    pub enum CustomLength for "custom_length" {
        Base => {
            symbol: "cu";
            coefficient: 1;
        }
        Half => {
            symbol: "hcu";
            coefficient: 1 / 2;
            aliases: ["half-cu"];
        }
    }
}
```

`coefficient` 接受一个 Decimal 字面量，或两个 Decimal 字面量组成的比例；可选 `offset`
接受 Decimal 字面量。宏生成：

- 单位枚举和 `Unit` 实现；
- `Display`、`FromStr`、Serialize、Deserialize；
- 严格与宽松解析表；
- `all()`、规范符号和 Decimal 单位定义。

外部 family 不必实现 `UomUnit`。如需映射到自有或 `uom` quantity，可手工实现可选 trait。
宏展开只引用 `$crate` 下的公开或文档隐藏重导出，确保下游无需直接依赖宏的实现依赖。
不提供运行时全局注册表。

## 9. 持久化与解析契约

### 9.1 JSON

0.3 的 JSON 固定包含三个字段：

```json
{
  "quantity": "length",
  "value": "50.0",
  "unit": "cm"
}
```

- `quantity` 是稳定的 `snake_case` 机器标识，如 `electric_potential`、
  `temperature_interval`。
- `value` 继续使用 Decimal 字符串表示。
- `unit` 序列化时始终使用规范符号。
- 三个字段都必须存在；重复字段拒绝。
- 额外字段按 Serde 默认行为忽略，为将来的字段扩展保留空间。
- 反序列化必须严格校验 `quantity == U::QUANTITY`；quantity 不提供宽松别名。
- unit 的默认 Deserialize 使用宽松解析，以接受已文档化的常见输入。

### 9.2 严格与宽松解析

- `parse_strict()` 只接受规范符号。
- 严格解析遇到已知别名时返回 `NonCanonicalUnit`，并携带建议的规范符号。
- `parse_lenient()` 接受规范符号及已声明别名。
- `FromStr`、`Measurement::from_str()` 和默认 Serde Deserialize 使用宽松解析。
- `Measurement::parse_strict()` 为高完整性导入和校验流程提供严格入口。
- 无论输入使用何种别名，`Display` 和 Serialize 都输出规范符号。

## 10. 歧义单位的显式定义

所有歧义单位都改用带定义限定词的枚举名和规范符号；常见短写只作为宽松别名。

| 范畴 | 0.3 显式单位 | 规范符号 | 主要宽松别名 |
| --- | --- | --- | --- |
| 时间 | `CommonYear365` | `a (365 d)` | `a`, `yr`, `year` |
| 能量 | `ThermochemicalCalorie` | `cal (th)` | `cal` |
| 能量 | `ThermochemicalKilocalorie` | `kcal (th)` | `kcal` |
| 能量 | `BritishThermalUnitInternationalTable` | `Btu (IT)` | `Btu`, `BTU` |
| 功率 | `MechanicalHorsepower` | `hp (mechanical)` | `hp` |
| 体积 | `UsFluidOunce` | `fl oz (US)` | `fl oz` |
| 体积 | `UsCustomaryCup` | `cup (US customary)` | `cup` |
| 体积 | `UsLiquidPint` | `pt (US liq)` | `liq pt` |
| 体积 | `UsLiquidQuart` | `qt (US liq)` | `liq qt` |
| 体积 | `UsLiquidGallon` | `gal (US)` | `gal` |
| 体积流量 | `UsGallonPerMinute` | `gal (US)/min` | `gal/min` |
| 质量密度 | `PoundPerUsGallon` | `lb/gal (US)` | `lb/gal` |

热容和比热容中的 calorie、Btu 单位同步改成 thermochemical calorie 和 Btu (IT) 的显式
变体及规范符号。宽松别名映射必须在中英文 README 中逐项列出。

`CommonYear365` 恒等于 31,536,000 秒，只是固定物理时长；日历年不属于本 crate。
US liquid volume 使用显式 US 定义。单位系数优先依据 SI、NIST 等权威定义计算，不直接复制
已经过 `f64` 舍入的 `uom` 数值。

## 11. 错误模型

保留现有错误并补充以下语义：

| 错误 | 条件 |
| --- | --- |
| `QuantityMismatch` | JSON quantity 与目标泛型 `U::QUANTITY` 不一致 |
| `InvalidScale` | 请求的 scale 超过 Decimal 上限 |
| `ArithmeticOverflow` | checked Decimal 运算发生上溢、下溢、范围失败，或无法保留请求的精确 scale |
| `InvalidUnitDefinition` | 外部单位定义含零分母、非正系数等非法数据 |
| `NonCanonicalUnit` | 严格解析遇到已知别名；错误包含规范符号建议 |
| `UnknownUnit` | 严格和宽松模式都无法识别输入 |
| `DecimalConversion` | 仅用于近似 `uom` 桥接的非有限值或转换失败 |

全局配置使用 `parking_lot::Mutex`，不存在标准库 mutex 的中毒错误分支。

## 12. 测试策略

实现遵循 TDD，先写失败测试，再补最小实现。主要测试层次如下。

### 12.1 Decimal 换算

- 有限十进制精确换算，例如 `m <-> cm`。
- 有理比例换算，例如 `5/9` 温标比例。
- 带 offset 的绝对温度换算。
- 循环小数在 `scale = None` 和各类显式 scale 下的结果。
- `MidpointNearestEven` 及其他公开舍入策略。
- 同单位默认保真和显式 scale 舍入。
- 最大、最小 Decimal 附近的范围错误。

### 12.2 单位定义独立基准

- 为每个内置单位提供独立于生产宏参数的基准系数与偏移量断言。
- 基准数据引用 SI、NIST 或对应规范，不把同一套 `uom` 映射来回转换当作正确性证明。
- 全量遍历 `Unit::all()`，确保每个变体都被基准覆盖。
- 对 US/Imperial、IT/thermochemical、固定年等易混定义设置专门的 golden tests。

### 12.3 配置与并发

- setter 返回旧配置并可恢复。
- 显式配置优先进程默认配置。
- 并发读取和修改只能观察到完整快照。
- 除专门的全局配置测试外，其余精度测试使用显式 options，避免并行测试依赖全局状态。
- 修改全局配置的测试集中执行并通过恢复守卫还原初始值。

### 12.4 解析与 Serde

- 三字段 JSON round trip。
- quantity 缺失、重复和不匹配。
- 严格解析只接受规范符号。
- 宽松解析覆盖所有文档化别名。
- alias 输入经 Display/Serialize 后规范化。
- `NonCanonicalUnit` 包含正确建议。

### 12.5 外部扩展和 uom

- 在集成测试中从 crate 外部视角调用 `define_unit_family!`。
- 测试手工 `Unit` 实现。
- `uom/f64` 桥接单独归类为近似测试，并使用显式容差。
- README 示例纳入 doctest 或等价编译检查。

## 13. 文档

`README.md` 和 `README.zh_CN.md` 保持相同章节结构，至少包含：

- 三字段 JSON 契约；
- Decimal 有限精度和循环小数边界；
- 进程默认配置及逐次覆盖示例；
- 严格与宽松解析示例；
- 所有歧义别名的确定映射表；
- 外部自定义 quantity family 示例；
- `to_uom_approx()` / `from_uom_approx()` 精度警告；
- 0.2 到 0.3 的迁移表。

安装和示例统一使用 crate 根模块重导出的 `Decimal`、`RoundingStrategy` 和配置类型，避免
示例依赖未声明的直接依赖或内部模块路径。

## 14. 发布与迁移

版本提升到 `0.3.0`。不提供旧 JSON 自动兼容，也不保留旧 trait 形状或旧歧义枚举变体。

迁移重点：

1. JSON 增加必需的 `quantity`。
2. `convert_to()` 从 `uom/f64` 换成 Decimal 换算。
3. `to_uom()` / `from_uom()` 改名为带 `approx` 的接口。
4. `Unit` 与可选 `UomUnit` 分离。
5. 歧义单位枚举改名；宽松解析仍接受常见短写。
6. quantity 标识统一为 `snake_case`。

新增直接依赖 `parking_lot`。如宏实现使用 `rust_decimal::dec!`，应通过现有
`rust_decimal` 依赖的 `macros` feature 提供，不要求下游额外声明宏依赖。

## 15. 验收标准

1. `convert_to()` 的实现路径中不存在 Decimal 与 `f64` 的互转。
2. 所有内置单位都有 Decimal 定义和独立基准测试。
3. 默认与显式舍入配置行为符合本文档，并且并发配置读取是完整快照。
4. JSON 始终写出 quantity、Decimal 字符串和规范单位符号。
5. 严格与宽松解析行为及全部歧义映射有测试和双语文档。
6. 下游 crate 能仅依赖公开 API 定义并使用新的单位 family。
7. `uom` 桥接只通过带 `approx` 的 API 暴露。
8. 单元测试、集成测试、doctest、rustdoc、格式化、Clippy 和项目既有检查全部通过。
