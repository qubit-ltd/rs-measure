# Qubit Measure

[![Rust CI](https://github.com/qubit-ltd/rs-measure/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-measure/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-measure/coverage-badge.json)](https://qubit-ltd.github.io/rs-measure/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-measure.svg?color=blue)](https://crates.io/crates/qubit-measure)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![English Document](https://img.shields.io/badge/Document-English-blue.svg)](README.md)

为 Rust 提供可持久化的类型化 measurement：使用纯 Decimal 单位换算、显式单位，
并提供可选的近似 `uom` 桥接。

## 1. 安装与快速开始

默认构建只包含精确 Decimal 核心，不编译 `uom`：

```toml
[dependencies]
qubit-measure = "0.4"
rust_decimal = "1.39"
```

需要近似 `f64` 桥接时显式启用：

```toml
[dependencies]
qubit-measure = { version = "0.4", features = ["uom"] }
rust_decimal = "1.39"
```

```rust
use qubit_measure::{measurement, unit};
use rust_decimal::Decimal;

let length = measurement::Length::new(Decimal::new(500, 1), unit::Length::Centimeter);
let meters = length.convert_to(unit::Length::Meter)?;
assert_eq!(meters.value, Decimal::new(5, 1));
# Ok::<(), qubit_measure::MeasurementError>(())
```

`Measurement<U>` 保存一个 `Decimal` 和一个类型化单位。`measurement::*` 中的别名与
`unit::*` 中的枚举覆盖 56 个物理 quantity family。

## 2. 三字段 JSON 契约

Serde 使用带 quantity 校验的 wire format：

```json
{
  "quantity": "length",
  "value": "50.0",
  "unit": "cm"
}
```

三个字段都必须存在。`quantity` 是稳定的 `snake_case` 标识，`value` 是 Decimal 字符串，
`unit` 序列化和反序列化时都只接受规范符号；别名会被拒绝。如果 quantity 与目标 Rust
类型不匹配，反序列化也会失败。额外字段会被忽略，允许未来添加元数据。

## 3. Decimal 精度与舍入

`convert_to` 不会把持久化值、系数、偏移或中间结果转换成 `f64`。单位系数使用经过校验的
Decimal 有理数，因此 `5 / 9`、精确 SI 前缀及精确英美制定义不会在声明时被舍入。所有
内建换算系数和偏移统一放在 crate 内部的 `consts.rs`，并按 quantity 分组。精确及派生定义
遵循 [BIPM SI Brochure](https://www.bipm.org/en/publications/si-brochure)、
[NIST 换算资料](https://www.nist.gov/pml/owm/metric-si/unit-conversion)、
[NIST Handbook 44](https://www.nist.gov/pml/owm/nist-handbook-44-current-edition) 和
[2022 CODATA](https://physics.nist.gov/cuu/Constants/)。无理数只能使用有限近似：pi 采用
[NIST DLMF 3.12](https://dlmf.nist.gov/3.12) 的 23 位小数，有限 tau 严格等于它的两倍，
平方度采用 28 位小数。每个内建 quantity family 使用的版本化来源、数值策略和覆盖范围
记录在机器可读的[单位定义来源清单](docs/unit-definition-provenance.tsv)中。

```rust
use qubit_measure::{
    ConversionOptions, measurement, unit,
};
use rust_decimal::{Decimal, RoundingStrategy};

let value = measurement::Length::new(Decimal::ONE, unit::Length::Meter);
let options = ConversionOptions::fixed_scale(
    4,
    RoundingStrategy::MidpointNearestEven,
)?;
let feet = value.convert_to_with_options(unit::Length::Foot, options)?;
assert_eq!(feet.value.to_string(), "3.2808");
# Ok::<(), qubit_measure::MeasurementError>(())
```

`ConversionOptions::maximum_precision()` 不指定固定输出 scale。换算过程先进行精确有理数
计算；若结果不能被 Decimal 精确表示，则在 mantissa 可容纳的最高 scale 上按 nearest-even
舍入，并规范化末尾零。`rounding() == None` 只表示没有选择 fixed-scale 策略，不表示表示
边界上绝不会舍入。`fixed_scale(0..=28, strategy)` 则按指定策略舍入并保留恰好指定的小数
位数。结果超出 Decimal 范围时返回 `ValueOutOfRange`；数值可表示但不能保留指定 scale 时
返回 `OutputScaleUnrepresentable`。当源和目标定义相同时，maximum-precision 换算是 no-op，
会保留原始 Decimal 的 scale 和末尾零。

## 4. 确定性默认配置

`convert_to` 始终使用不可变的 `ConversionOptions::DEFAULT`：不指定固定 scale，在表示边界
按 nearest-even 舍入；除上述同定义 no-op 外，输出会被规范化。crate 不包含进程级可变
换算状态。需要固定输出 scale 和舍入策略时，应显式调用 `convert_to_with_options`。

## 5. 严格与宽松解析

`Unit::parse_strict` 只接受规范符号；遇到已知别名时返回 `NonCanonicalUnit` 并给出规范替代。
`FromStr`、`Measurement::from_str` 和默认 Serde 反序列化都采用这一严格契约。只有显式调用
`Unit::parse_lenient` 或 `Measurement::parse_lenient` 时才接受已声明别名。规范符号与别名
必须互不相交。

```rust
use qubit_measure::{Unit, unit};

assert_eq!(unit::Time::parse_lenient("year")?, unit::Time::CommonYear365);
assert!(unit::Time::parse_strict("year").is_err());
assert_eq!(unit::Time::parse_strict("a (365 d)")?, unit::Time::CommonYear365);
assert!("1 year".parse::<qubit_measure::measurement::Time>().is_err());
assert_eq!(
    qubit_measure::measurement::Time::parse_lenient("1 year")?.unit,
    unit::Time::CommonYear365,
);
# Ok::<(), qubit_measure::MeasurementError>(())
```

`Time::Minute` 也接受宽松别名 `m`，但显示、严格解析和 Serde 始终保留规范符号
`min`。紧凑 measurement 会按已知单位后缀匹配；如果存在多个合法的数值/单位切分，
返回 `AmbiguousMeasurement`，不会静默选择其中一种解释。
以 `.`、`+` 或 `-` 开头的单位符号或别名必须与 Decimal 数值使用空白分隔；
其紧凑形式会作为有歧义的数值边界被拒绝（例如应写成 `1.25 +cu`）。

### 精确 `std::time::Duration` 适配

`Measurement<Time>` 通过标准 `From` 和 `TryFrom` 实现与
`std::time::Duration` 双向转换。转换精确到纳秒；负值、亚纳秒值和越界值会直接报错，
不会隐式舍入。

## 6. 歧义单位别名

有歧义的概念使用带限定词的枚举名和规范符号。常见输入字符串只在宽松解析中保留。

| Quantity | 显式变体 | 规范符号 | 宽松别名 |
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

热容和比热容中的 calorie、Btu 变体使用相同的 thermochemical 和 International Table 限定。
`CommonYear365` 精确等于 31,536,000 秒，是固定时长而不是日历模型。

`MillimeterOfMercury` 采用精确的 Torr 等价值 `101325 / 760 Pa`
（`20265 / 152 Pa`）。其规范符号为 `mm Hg`，宽松解析也接受 `mmHg`。该值不同于
部分换算表采用的 conventional `133.3224 Pa` 舍入值。若应用需要该 conventional
值，应定义外部单位，不能假定当前变体采用该值。可选 `uom` 桥接通过 Pascal 基单位
应用这一精确定义，而不采用 `uom` 的 conventional millimeter-of-mercury 系数。参见
[NIST SP 811 Chapter 5](https://www.nist.gov/pml/special-publication-811/nist-guide-si-chapter-5-units-outside-si)
和 [Appendix B.9](https://www.nist.gov/pml/special-publication-811/nist-guide-si-appendix-b-conversion-factors/nist-guide-si-appendix-b9)。

## 7. 外部单位族

`Unit`、`ConversionFactor` 和 `UnitDefinition` 均为公开 API。导出的宏支持编译期扩展，
不引入运行时注册表，也不强制要求 `uom` 映射。

```rust
use qubit_measure::{Unit, define_unit_family};

define_unit_family! {
    pub enum CustomLength for "custom_length" {
        Base => { symbol: "cu"; coefficient: 1; }
        Half => {
            symbol: "hcu";
            coefficient: 1 / 2;
            aliases: ["half-cu"];
        }
    }
}

assert_eq!(CustomLength::parse_lenient("half-cu")?, CustomLength::Half);
# Ok::<(), qubit_measure::MeasurementError>(())
```

宏会生成规范显示、严格和宽松解析、Serde、枚举遍历及精确定义。可选 `uom` 桥接通过
`impl_uom_unit!` 在消费者自己的 feature 配置下单独添加。外部代码也可以手工实现
`Unit`。Measurement Serde 直接使用 `Unit` 的符号和解析契约，因此手工单位无需另外实现
`Serialize` 或 `Deserialize`。

每个单位族都遵守以下元数据契约：

- `quantity` 是非空 ASCII `snake_case`，以小写字母开头，且没有开头、结尾或连续下划线；
- 规范符号非空且互不重复，并且不得包含开头或结尾的 Unicode 空白字符；
- 别名非空且互不重复，并且不得包含开头或结尾的 Unicode 空白字符；
- 别名不得等于单位族中的任何规范符号；
- 宏生成的单位族在编译期接受检查；
- 手工 `Unit` 实现应在测试中调用 `assert_unit_family_valid`；
- stable Rust 无法证明手工枚举的 `all()` 没有遗漏任何变体。

## 8. 近似 `uom` 桥接

该桥接只在显式启用默认关闭的 `uom` Cargo feature 后存在。未启用时，`UomUnit`、
`try_to_uom_approx`、`to_uom_approx` 和 `from_uom_approx` 均不在 API 中。启用后，
映射到 `uom` 的 family 会实现 `UomUnit`。`_approx` 后缀是有意设计：这些适配器会跨越
`Decimal <-> f64`，因此可能损失精度。适配器会先按 `qubit-measure` 的精确定义得到
SI 基单位值，所以 quantity 的物理基准值与精确 Decimal 核心一致；但之后若通过
`uom` 自带的非基准单位 getter 读取，当两个库对同名单位的定义不同时，显示数值仍按
`uom` 自身的系数计算。持久化单位换算 `convert_to` 不使用该桥接。

对于外部手写的单位定义，应优先使用 `try_to_uom_approx`，以便返回 definition 错误；
为保持兼容，无错误返回值的 `to_uom_approx` 仍保留原有 panic 行为。

```rust
use qubit_measure::{measurement, unit};
use rust_decimal::Decimal;
use uom::si::length::meter;

let value = measurement::Length::new(Decimal::new(50, 0), unit::Length::Centimeter);
assert_eq!(value.to_uom_approx().get::<meter>(), 0.5);
```

适合二进制浮点语义的量纲计算仍可使用 `uom`，然后在持久化边界显式适配结果。

## 9. 从 0.3 迁移到 0.4

| 0.3 | 0.4 |
| --- | --- |
| `qubit_measure::{Decimal, RoundingStrategy}` | 直接从 `rust_decimal` 导入这两个类型 |
| `FromStr` 和默认 Serde 接受别名 | 只接受规范符号；别名使用显式 `parse_lenient` |
| 别名可以与规范符号冲突 | 规范符号与别名必须互不相交 |
| 下游宏通过本 crate 的重导出解析依赖 | 下游直接声明 `serde`，并在需要时声明 `rust_decimal` |

本版本有意移除依赖项重导出，并收紧解析与单位族元数据契约。下游 crate 必须更新导入、
依赖声明，以及持久化数据中的别名拼写。

## 测试

```bash
# 使用默认 feature 集运行测试
cargo test

# 使用项目声明的全部 feature 运行测试
cargo test --all-features

# 运行项目 CI 检查
./ci-check.sh

# 检查代码覆盖率
./coverage.sh
```

## 许可证

Copyright (c) 2025 - 2026. Haixing Hu. All rights reserved.

本项目基于 Apache License 2.0 授权。完整许可证文本请参阅
[LICENSE](LICENSE)。

## 贡献

欢迎贡献。请遵循 Rust API 指南，及时更新公共 API 文档与测试，并在提交
Pull Request 前运行 `./align-ci.sh`格式化代码，运行`./ci-check.sh`对齐CI要求。

## 作者

**Haixing Hu** - *Qubit Co. Ltd.*

仓库地址：[https://github.com/qubit-ltd/rs-measure](https://github.com/qubit-ltd/rs-measure)
