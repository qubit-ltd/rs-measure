# Qubit Measure

[![Rust CI](https://github.com/qubit-ltd/rs-measure/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-measure/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-measure/coverage-badge.json)](https://qubit-ltd.github.io/rs-measure/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-measure.svg?color=blue)](https://crates.io/crates/qubit-measure)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Chinese Document](https://img.shields.io/badge/Document-Chinese-blue.svg)](README.zh_CN.md)

Persistent typed measurement values with explicit units and `uom` adapters for Rust.

## Overview

`uom` is excellent for type-safe dimensional analysis. It works with quantities
such as `Length`, `Mass`, and `Time`, and only uses concrete units at API
boundaries through calls such as `Length::new::<centimeter>(50.0)` and
`length.get::<meter>()`.

That is the right model for calculation, but it is not a complete persistence
contract. Once a `uom` quantity is created, the value is normalized to the
quantity's base unit. This crate preserves the boundary fact:

```json
{
  "value": "50.0",
  "unit": "cm"
}
```

Qubit Measure is therefore a thin wrapper around `uom`, not a replacement unit
system.

## Design Goals

- **Use `uom` as the source of truth**: conversions go through `uom`, not a
  second conversion table maintained by this crate.
- **Keep quantity types explicit**: use aliases such as `measurement::Length`
  and unit families such as `unit::Length` instead of a single untyped unit enum.
- **Preserve input units**: store the decimal value and the selected unit
  together for APIs, databases, forms, spreadsheets, and audit records.
- **Keep count units out of physical measurement**: pieces, sheets, boxes, and
  batches are business or counting concepts and are intentionally not mixed
  with `uom` physical quantities.

## Features

### Typed persisted measurements

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

`measurement::Length` is an alias for `Measurement<unit::Length>`. The same
generic wrapper is used for every supported `uom` quantity family.

### Serde persistence

```json
{
  "value": "50.0",
  "unit": "cm"
}
```

The quantity is usually known from the field type. For example, a JSON field
typed as `measurement::Length` can only deserialize length units.

### Conversion through `uom`

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

### `uom` adapters

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

Create persisted values from `uom` quantities with the target storage unit:

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

## Supported Quantity Families

- `measurement::Length` with `unit::Length`
- `measurement::Area` with `unit::Area`
- `measurement::Volume` with `unit::Volume`
- `measurement::Mass` with `unit::Mass`
- `measurement::Time` with `unit::Time`
- `measurement::Pressure` with `unit::Pressure`
- `measurement::Energy` with `unit::Energy`
- `measurement::Power` with `unit::Power`
- `measurement::Velocity` with `unit::Velocity`
- `measurement::Frequency` with `unit::Frequency`
- `measurement::MassDensity` with `unit::MassDensity`
- `measurement::Temperature` with `unit::Temperature`
- `measurement::TemperatureInterval` with `unit::TemperatureInterval`

The API is intentionally shaped so additional `uom` quantity families can be
added without changing the generic `Measurement<U>` wrapper.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
qubit-measure = "0.1"
```

## Relationship with `uom`

Use `uom` directly inside calculation code. Use Qubit Measure at runtime
boundaries where the selected unit must remain explicit:

- database rows;
- JSON APIs;
- spreadsheet import/export;
- UI form values;
- rule configuration;
- audit trails;
- source-data reconciliation.

## License

This project is licensed under the Apache License 2.0. See [LICENSE](LICENSE)
for details.
