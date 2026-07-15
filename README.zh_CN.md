# Qubit Measure

[![Rust CI](https://github.com/qubit-ltd/rs-measure/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-measure/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/qubit-measure.svg?color=blue)](https://crates.io/crates/qubit-measure)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![English Document](https://img.shields.io/badge/Document-English-blue.svg)](README.md)

为 Rust 提供可持久化的类型化 measurement：使用纯 Decimal 单位换算、显式单位，
并提供可选的近似 `uom` 桥接。

## 1. 安装与快速开始

```toml
[dependencies]
qubit-measure = "0.3"
```

```rust
use qubit_measure::{Decimal, measurement, unit};

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
`unit` 序列化时始终使用规范符号。如果 quantity 与目标 Rust 类型不匹配，反序列化会失败。
额外字段会被忽略，允许未来添加元数据。

## 3. Decimal 精度与舍入

`convert_to` 不会把持久化值、系数、偏移或中间结果转换成 `f64`。单位系数使用经过校验的
Decimal 有理数，因此 `5 / 9`、精确 SI 前缀及精确英美制定义不会在声明时被舍入。

```rust
use qubit_measure::{
    ConversionOptions, Decimal, RoundingStrategy, measurement, unit,
};

let value = measurement::Length::new(Decimal::ONE, unit::Length::Meter);
let options = ConversionOptions::fixed_scale(
    4,
    RoundingStrategy::MidpointNearestEven,
)?;
let feet = value.convert_to_with_options(unit::Length::Foot, options)?;
assert_eq!(feet.value.to_string(), "3.2808");
# Ok::<(), qubit_measure::MeasurementError>(())
```

`ConversionOptions::maximum_precision(strategy)` 不额外降低最终 scale。
`fixed_scale(0..=28, strategy)` 会按指定策略舍入并保留恰好指定的小数位数。Decimal 仍只有
有限的 96 位 mantissa：循环小数、无理常数和超出范围的结果不可能获得数学无限精度。
算术溢出或无法保留指定 scale 时返回 `MeasurementError`。

## 4. 进程级默认配置

`convert_to` 会快照读取由 `parking_lot::Mutex` 保护的进程级默认值。初始值是最大精度与
`MidpointNearestEven`。需要可重复结果的业务代码和测试通常应传入显式配置。

```rust
use qubit_measure::{
    ConversionOptions, RoundingStrategy, default_conversion_options,
    set_default_conversion_options,
};

let original = default_conversion_options();
let replacement = ConversionOptions::fixed_scale(
    6,
    RoundingStrategy::MidpointNearestEven,
)?;
set_default_conversion_options(replacement);
// 执行有意使用进程默认配置的换算。
set_default_conversion_options(original);
# Ok::<(), qubit_measure::MeasurementError>(())
```

setter 会原子替换完整配置并返回旧值，方便调用方恢复。

## 5. 严格与宽松解析

`Unit::parse_strict` 只接受规范符号；遇到已知别名时返回 `NonCanonicalUnit` 并给出规范替代。
`Unit::parse_lenient`、`FromStr`、`Measurement::from_str` 和默认 Serde 反序列化接受已声明别名。
`Measurement::parse_strict` 为完整 measurement 提供严格解析。

```rust
use qubit_measure::{Unit, unit};

assert_eq!(unit::Time::parse_lenient("year")?, unit::Time::CommonYear365);
assert!(unit::Time::parse_strict("year").is_err());
assert_eq!(unit::Time::parse_strict("a (365 d)")?, unit::Time::CommonYear365);
# Ok::<(), qubit_measure::MeasurementError>(())
```

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

宏会生成规范显示、严格和宽松解析、Serde、枚举遍历及精确定义。外部代码也可以手工实现
`Unit`。

## 8. 近似 `uom` 桥接

映射到 `uom` 的 family 会实现 `UomUnit`，并提供 `to_uom_approx` / `from_uom_approx`。
`_approx` 后缀是有意设计：这些适配器会跨越 `Decimal <-> f64`，因此可能损失精度。
持久化单位换算 `convert_to` 不使用该桥接。

```rust
use qubit_measure::{Decimal, measurement, unit};
use uom::si::length::meter;

let value = measurement::Length::new(Decimal::new(50, 0), unit::Length::Centimeter);
assert_eq!(value.to_uom_approx().get::<meter>(), 0.5);
```

适合二进制浮点语义的量纲计算仍可使用 `uom`，然后在持久化边界显式适配结果。

## 9. 从 0.2 迁移到 0.3

| 0.2 | 0.3 |
| --- | --- |
| JSON `{value, unit}` | JSON `{quantity, value, unit}` |
| `convert_to` 经过 `uom/f64` | 使用精确系数的纯 Decimal 换算 |
| `to_uom` / `from_uom` | `to_uom_approx` / `from_uom_approx` |
| `Year`、`Gallon`、`Horsepower` 等歧义变体 | 上表列出的带限定词变体 |
| 歧义短写是规范符号 | 限定后的规范符号；短写仅为宽松别名 |
| `Unit` 包含 `uom` 方法 | 精确 `Unit` 加可选 `UomUnit` |
| 单位族仅 crate 内部可定义 | 公开 `define_unit_family!` 并允许手工实现 |

本版本有意破坏 0.2 wire format 和相关 Rust API。

## License

使用 Apache License 2.0，详见 [LICENSE](LICENSE)。
