# Qubit Measure

[![Rust CI](https://github.com/qubit-ltd/rs-measure/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-measure/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-measure/coverage-badge.json)](https://qubit-ltd.github.io/rs-measure/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-measure.svg?color=blue)](https://crates.io/crates/qubit-measure)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![English Document](https://img.shields.io/badge/Document-English-blue.svg)](README.md)

为 Rust 提供可持久化的类型化 measurement：显式保存数值和单位，并和
`uom` 的 quantity 类型互相转换。

## 概述

`uom` 很适合做类型安全的量纲分析。它以 `Length`、`Mass`、`Time` 这类
quantity 为中心，只在边界通过 `Length::new::<centimeter>(50.0)` 和
`length.get::<meter>()` 这样的调用使用具体单位。

这个模型适合计算，但不是完整的持久化契约。`uom` quantity 创建后会把值归一到
quantity 的基准单位，而很多系统需要保留用户输入时的事实：

```json
{
  "value": "50.0",
  "unit": "cm"
}
```

Qubit Measure 因此是 `uom` 的薄 wrapper，不是另一套单位系统。

## 设计目标

- **以 `uom` 为换算真相**：单位换算通过 `uom` 完成，本 crate 不维护第二份比例表。
- **quantity 类型显式**：使用 `measurement::Length` 这类持久化别名和
  `unit::Length` 这类 unit family，不再使用一个未类型化的全局单位枚举。
- **保留输入单位**：把 decimal 数值和用户选择的单位一起保存，适合 API、数据库、
  表单、电子表格和审计记录。
- **不混入计数单位**：件、张、箱、批属于业务数量或包装概念，不和 `uom` 物理
  quantity 混在同一套 API 里。

## 特性

### 类型化持久化 measurement

```rust
use qubit_measure::{
    Unit,
    measurement,
    unit,
};
use rust_decimal::Decimal;

let thickness = measurement::Length::new(Decimal::new(500, 1), unit::Length::Centimeter);

assert_eq!(thickness.value.to_string(), "50.0");
assert_eq!(thickness.unit.symbol(), "cm");
```

`measurement::Length` 是 `Measurement<unit::Length>` 的类型别名。所有支持的
`uom` quantity family 都复用同一个泛型 wrapper。

### Serde 持久化

```json
{
  "value": "50.0",
  "unit": "cm"
}
```

quantity 通常由字段类型决定。例如字段类型是 `measurement::Length` 时，只能反序列化
长度单位。

### 通过 `uom` 换算

```rust
use qubit_measure::{
    measurement,
    unit,
};
use rust_decimal::Decimal;

let grams = measurement::Mass::new(Decimal::new(1, 1), unit::Mass::Gram);
let kilograms = grams.convert_to(unit::Mass::Kilogram)?;

assert_eq!(kilograms.value, Decimal::new(1, 4));
# Ok::<(), qubit_measure::MeasurementError>(())
```

### `uom` 适配

```rust
use qubit_measure::{
    measurement,
    unit,
};
use rust_decimal::Decimal;
use uom::si::length::meter;

let persisted = measurement::Length::new(Decimal::new(50, 0), unit::Length::Centimeter);
let length = persisted.to_uom();

assert_eq!(length.get::<meter>(), 0.5);
# Ok::<(), qubit_measure::MeasurementError>(())
```

也可以从 `uom` quantity 创建指定目标单位的持久化值：

```rust
use qubit_measure::{
    measurement,
    unit,
};
use uom::si::f64::Length;
use uom::si::length::meter;

let length = Length::new::<meter>(0.5);
let persisted = measurement::Length::from_uom(length, unit::Length::Centimeter)?;

assert_eq!(persisted.to_string(), "50 cm");
# Ok::<(), qubit_measure::MeasurementError>(())
```

## 已支持的 Quantity Family

- `measurement::Length` / `unit::Length`
- `measurement::Area` / `unit::Area`
- `measurement::Volume` / `unit::Volume`
- `measurement::Mass` / `unit::Mass`
- `measurement::Time` / `unit::Time`
- `measurement::Pressure` / `unit::Pressure`
- `measurement::Energy` / `unit::Energy`
- `measurement::Power` / `unit::Power`
- `measurement::Velocity` / `unit::Velocity`
- `measurement::Frequency` / `unit::Frequency`
- `measurement::MassDensity` / `unit::MassDensity`
- `measurement::Temperature` / `unit::Temperature`
- `measurement::TemperatureInterval` / `unit::TemperatureInterval`

API 设计为后续追加其他 `uom` quantity family 时不需要改变泛型
`Measurement<U>` wrapper。

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
qubit-measure = "0.1"
```

## 与 `uom` 的关系

计算代码内部直接使用 `uom`。当一个值跨越运行时边界、需要显式保留单位时，使用
Qubit Measure：

- 数据库记录；
- JSON API；
- 电子表格导入导出；
- UI 表单值；
- 规则配置；
- 审计记录；
- 来源数据对账。

## 许可证

本项目使用 Apache License 2.0。详见 [LICENSE](LICENSE)。
