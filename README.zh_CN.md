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

Qubit Measure 是围绕 [`uom`](https://crates.io/crates/uom) 的持久化边界
wrapper。`uom` 很适合做类型安全的量纲分析，但一个 `uom` quantity 本身不是完整的
持久化契约：创建后，数值会归一到该 quantity 的基准单位，用户或来源数据原本选择的
单位不会继续留在值里。

很多 API、数据库、电子表格、表单和审计边界需要精确保留以下事实：

```json
{
  "value": "50.0",
  "unit": "cm"
}
```

Qubit Measure 把这个边界表示显式建模为 `Decimal + typed unit`，同时把所有物理
单位换算都交给 `uom`。

## 设计目标

- **以 `uom` 为换算真相**：单位换算通过 `uom` 完成，本 crate 不维护第二份比例表。
- **数值和单位一起持久化**：序列化数据保留用户选择的单位，而不是只保留基准单位值。
- **quantity 类型显式**：`measurement::Length` 和 `unit::Length` 在 Rust 类型层面表达 quantity family。
- **边界值使用 decimal**：持久化值使用 `rust_decimal`，避免文本边界上的二进制浮点往返误差。
- **区分物理 measurement 和业务计数**：件、张、箱、批、包装数量不属于这套物理单位系统。
- **保持公开 API 小而清楚**：本 crate 提供类型化 wrapper 和 unit family；计算代码仍可直接使用 `uom`。

## 特性

### 类型化持久化 measurement

- **泛型 wrapper**：`Measurement<U>` 保存 `Decimal` 数值和类型化 unit family 成员。
- **quantity 别名**：`measurement::Length`、`measurement::Mass`、`measurement::Pressure` 等别名提供更方便的字段类型。
- **unit family**：`unit::Length`、`unit::Mass`、`unit::Pressure` 等枚举暴露稳定的序列化符号。
- **类型安全解析**：解析长度 measurement 时只会解析长度单位；`kg` 这类质量单位会在长度上下文中被拒绝。

### `uom` 桥接

- **不可失败的 `to_uom()`**：持久化 measurement 可以转换成类型化 `uom` quantity 用于计算。
- **可失败的 `from_uom()`**：`uom` quantity 可以按指定目标单位持久化；转回 `Decimal` 时会报告精度错误。
- **可失败的 `convert_to()`**：同一 quantity family 内的单位换算通过 `uom` 完成。

### 稳定序列化

- **Serde 支持**：值序列化为 `{ "value": "...", "unit": "..." }`。
- **字符串 decimal**：decimal 使用 `rust_decimal::serde::str`，保留稳定的十进制文本值。
- **稳定单位符号**：单位序列化为 `cm`、`kg`、`kPa`、`mW`、`cm/s` 等紧凑符号。
- **输入别名**：解析时接受 `um`、`m2`、`m^3`、`mmHg`、`mph`、`year`
  等别名；序列化时保持 `µm`、`m²`、`m³`、`mm Hg`、`mi/h`、`a` 等规范符号。

### 聚焦的公开 API

- **`Measurement<U>`**：泛型持久化 measurement wrapper。
- **`Unit`**：每个支持的 unit family 实现的 trait。
- **`measurement::*`**：`measurement::Length` 这类 quantity-specific 别名。
- **`unit::*`**：`unit::Length` 这类 quantity-specific unit family。
- **`MeasurementError`**：解析、未知单位和 decimal 转换的类型化错误。

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
qubit-measure = "0.1"
```

## 快速开始

### 创建类型化 measurement

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
assert_eq!(thickness.quantity_name(), "length");
```

### 序列化持久化值

```rust
use qubit_measure::{
    measurement,
    unit,
};
use rust_decimal::Decimal;
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let length = measurement::Length::new(Decimal::new(500, 1), unit::Length::Centimeter);
    let value = serde_json::to_value(length)?;

    assert_eq!(value, json!({
        "value": "50.0",
        "unit": "cm"
    }));
    Ok(())
}
```

### 解析类型化 measurement

```rust
use qubit_measure::{
    measurement,
    unit,
};
use rust_decimal::Decimal;
use std::str::FromStr;

fn main() -> Result<(), qubit_measure::MeasurementError> {
    let length = measurement::Length::from_str("12.5 cm")?;

    assert_eq!(length.value, Decimal::new(125, 1));
    assert_eq!(length.unit, unit::Length::Centimeter);
    Ok(())
}
```

### 通过 `uom` 换算单位

```rust
use qubit_measure::{
    measurement,
    unit,
};
use rust_decimal::Decimal;

fn main() -> Result<(), qubit_measure::MeasurementError> {
    let grams = measurement::Mass::new(Decimal::new(1, 1), unit::Mass::Gram);
    let kilograms = grams.convert_to(unit::Mass::Kilogram)?;

    assert_eq!(kilograms.value, Decimal::new(1, 4));
    assert_eq!(kilograms.unit, unit::Mass::Kilogram);
    Ok(())
}
```

### 把值传入 `uom`

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
```

### 按目标单位持久化 `uom` quantity

```rust
use qubit_measure::{
    measurement,
    unit,
};
use uom::si::f64::Length;
use uom::si::length::meter;

fn main() -> Result<(), qubit_measure::MeasurementError> {
    let length = Length::new::<meter>(0.5);
    let persisted = measurement::Length::from_uom(length, unit::Length::Centimeter)?;

    assert_eq!(persisted.to_string(), "50 cm");
    Ok(())
}
```

### 使用生产常用单位

```rust
use qubit_measure::{
    measurement,
    unit,
};
use rust_decimal::Decimal;
use uom::si::pressure::pascal;
use uom::si::velocity::meter_per_second;

let pressure = measurement::Pressure::new(Decimal::new(2500, 0), unit::Pressure::Millipascal);
let speed = measurement::Velocity::new(Decimal::new(100, 0), unit::Velocity::CentimeterPerSecond);

assert_eq!(pressure.to_uom().get::<pascal>(), 2.5);
assert_eq!(speed.to_uom().get::<meter_per_second>(), 1.0);
```

## API 参考

### 核心类型

| 类型 | 描述 |
|------|------|
| `Measurement<U>` | 泛型持久化值和类型化单位 |
| `Unit` | 支持的 unit family 实现的 trait |
| `MeasurementError` | 解析、未知单位和 decimal 转换错误 |
| `measurement::*` | `measurement::Length` 这类类型别名 |
| `unit::*` | `unit::Length` 这类单位枚举 |

### `Measurement<U>` 操作

| 方法 | 描述 | 错误行为 |
|------|------|----------|
| `new(value, unit)` | 创建持久化 measurement | 不失败 |
| `quantity_name()` | 返回小写 quantity 名称 | 不失败 |
| `to_uom()` | 转换成类型化 `uom` quantity | 不失败 |
| `from_uom(quantity, unit)` | 用指定单位持久化 `uom` quantity | 结果 `f64` 不能表示成 `Decimal` 时失败 |
| `convert_to(target)` | 换算到同 family 的另一个单位 | 同 `from_uom()` |
| `to_string()` | 格式化为 `<value> <unit>` | 不失败 |
| `FromStr` | 解析 `<decimal><unit>` 或 `<decimal> <unit>` | decimal 无效或单位不属于该 typed family 时失败 |

### `Unit` 操作

| 方法 | 描述 |
|------|------|
| `all()` | 返回该 family 支持的所有单位 |
| `symbol()` | 返回规范序列化符号 |
| `to_uom(value)` | 用当前单位中的 decimal 值创建 `uom` quantity |
| `value_from_uom(quantity)` | 从 `uom` quantity 中按当前单位取出 decimal 值 |

## 已支持的 Quantity Family

| Measurement 别名 | Unit family | 示例单位 |
|------------------|-------------|----------|
| `measurement::Length` | `unit::Length` | `µm`、`mm`、`cm`、`m`、`km`、`in`、`ft`、`mi` |
| `measurement::Area` | `unit::Area` | `mm²`、`cm²`、`m²`、`km²`、`ha`、`ac`、`ft²` |
| `measurement::Volume` | `unit::Volume` | `mm³`、`cm³`、`m³`、`µL`、`mL`、`L`、`gal` |
| `measurement::Mass` | `unit::Mass` | `µg`、`mg`、`g`、`kg`、`t`、`oz`、`lb` |
| `measurement::Time` | `unit::Time` | `ns`、`µs`、`ms`、`s`、`min`、`h`、`d`、`a` |
| `measurement::Pressure` | `unit::Pressure` | `nPa`、`µPa`、`mPa`、`Pa`、`hPa`、`kPa`、`MPa`、`bar`、`psi` |
| `measurement::Energy` | `unit::Energy` | `J`、`kJ`、`MJ`、`W · h`、`kW · h`、`eV`、`cal`、`Btu` |
| `measurement::Power` | `unit::Power` | `nW`、`µW`、`mW`、`W`、`kW`、`MW`、`hp` |
| `measurement::Velocity` | `unit::Velocity` | `µm/s`、`mm/s`、`cm/s`、`m/s`、`km/h`、`ft/s`、`kn` |
| `measurement::Frequency` | `unit::Frequency` | `Hz`、`kHz`、`MHz`、`GHz` |
| `measurement::MassDensity` | `unit::MassDensity` | `kg/m³`、`g/m³`、`g/cm³`、`lb/ft³`、`lb/gal` |
| `measurement::Temperature` | `unit::Temperature` | `K`、`°C`、`°F`、`°R` |
| `measurement::TemperatureInterval` | `unit::TemperatureInterval` | `K`、`°C`、`°F`、`°R` |

## 持久化策略

### 应该持久化什么

当用户选择的单位本身有业务含义时，使用 Qubit Measure 持久化：

- 数据库记录；
- JSON API；
- 电子表格导入导出；
- UI 表单值；
- 规则配置；
- 审计记录；
- 来源数据对账。

### 不应该作为物理 measurement 持久化什么

除非概念能映射到固定的 `uom` 物理 quantity，否则业务计数和日历概念应放在本 crate 外部：

- 件、张、箱、卷、批、批号、包装数量；
- 日历月份或业务周期；
- 领域专用包装规格。

例如，`month` 不是固定物理时长，因此不是 `unit::Time` 变体。它应由日历或业务周期模型表达。

### 与 `uom` 的关系

计算密集的内部代码直接使用 `uom`。当值跨越运行时边界，且需要保留用户选择或来源数据中的单位时，使用 Qubit Measure。
本 crate 是 `uom` 的 wrapper，而不是替代性的量纲系统。

## 精度与换算说明

| 方向 | 数值表示 | 说明 |
|------|----------|------|
| 持久化值 | `rust_decimal::Decimal` | 适合稳定文本持久化 |
| `to_uom()` | `f64` `uom` quantity | 不失败，因为 `Decimal` 有限且处在 `f64` 指数范围内 |
| `from_uom()` | `f64` 到 `Decimal` | NaN、无穷大或无法表示为 `Decimal` 的值会失败 |
| `convert_to()` | `Decimal -> uom -> Decimal` | 使用 `uom` 作为换算真相源 |

## 测试与代码覆盖率

本项目通过集成测试和本地 CI 脚本覆盖核心行为。

### 运行测试

```bash
# 运行所有测试
cargo test --all-features

# 运行集成测试入口
cargo test --test mod

# 运行风格检查
./style-check.sh

# 运行覆盖率
./coverage.sh

# 运行完整本地 CI
./ci-check.sh
```

### 覆盖率指标

`./coverage.sh` 会在 `target/llvm-cov/` 下生成覆盖率报告。README 顶部的覆盖率徽章链接到 GitHub Pages 覆盖率报告。

## 依赖项

运行时依赖保持聚焦：

- `uom` 提供类型安全的 SI quantity 系统和单位换算。
- `rust_decimal` 保存稳定的 decimal 边界值。
- `serde` 负责序列化 measurement 和单位符号。
- `thiserror` 定义公开错误类型。

## 许可证

Copyright (c) 2026 Haixing Hu.

根据 Apache 许可证 2.0 版（"许可证"）授权；
除非遵守许可证，否则您不得使用此文件。
您可以在以下位置获取许可证副本：

    http://www.apache.org/licenses/LICENSE-2.0

除非适用法律要求或书面同意，否则根据许可证分发的软件
按"原样"分发，不附带任何明示或暗示的担保或条件。
有关许可证下的特定语言管理权限和限制，请参阅许可证。

完整的许可证文本请参阅 [LICENSE](LICENSE)。

## 贡献

欢迎贡献。提交 Pull Request 前请运行本地检查：

```bash
./align-ci.sh
./ci-check.sh
```

### 开发指南

- 保持 `uom` 作为换算真相源。
- 新增物理 quantity family 时，按类型化 `Measurement<U>` 别名和 `unit::*` family 添加。
- unit enum 是非穷尽的。下游 `match` 应保留通配分支；本 crate 后续扩展 unit 时，
  只需要在对应 `unit::*` 宏调用中追加规范符号和解析别名。
- 不要把业务计数单位混入物理 measurement family。
- 为符号、解析、serde 和 `uom` 桥接行为补充聚焦的集成测试。

## 作者

Haixing Hu

## 相关项目

- [`uom`](https://crates.io/crates/uom)：类型安全、零成本的量纲分析库。
- [`rust_decimal`](https://crates.io/crates/rust_decimal)：用于稳定边界值的 decimal 算术库。
