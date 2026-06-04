# Qubit Measure

[![Rust CI](https://github.com/qubit-ltd/rs-measure/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-measure/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-measure/coverage-badge.json)](https://qubit-ltd.github.io/rs-measure/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-measure.svg?color=blue)](https://crates.io/crates/qubit-measure)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![中文文档](https://img.shields.io/badge/文档-中文版-blue.svg)](README.zh_CN.md)

Persistent typed measurement values with explicit units and `uom` adapters for Rust.

## Overview

Qubit Measure is a persistence-oriented wrapper around [`uom`](https://crates.io/crates/uom).
`uom` is the right foundation for type-safe dimensional analysis, but a `uom`
quantity is not a complete persistence contract: after construction, the value is
stored in the quantity's normalized base unit and the original user-facing unit
is no longer part of the value.

Many API, database, spreadsheet, form, and audit boundaries need to preserve this
fact exactly:

```json
{
  "value": "50.0",
  "unit": "cm"
}
```

Qubit Measure keeps that boundary representation explicit as `Decimal + typed
unit`, while delegating all physical conversion to `uom`.

## Design Goals

- **Use `uom` as the source of truth**: conversions go through `uom`, not a
  second conversion table maintained by this crate.
- **Persist value and unit together**: serialized data keeps the selected unit
  instead of only a base-unit value.
- **Keep quantity types explicit**: `measurement::Length` and `unit::Length`
  encode the quantity family at the Rust type level.
- **Use decimal values at boundaries**: persisted values use `rust_decimal` to
  avoid accidental binary floating-point text round-trips.
- **Separate physical measurement from business counting**: pieces, sheets,
  boxes, batches, and package counts are intentionally outside this physical
  unit system.
- **Keep the public API small**: the crate provides typed wrappers and unit
  families; calculation code can continue to use `uom` directly.

## Features

### Typed persisted measurements

- **Generic wrapper**: `Measurement<U>` stores a `Decimal` value and a typed unit
  family member.
- **Quantity aliases**: `measurement::Length`, `measurement::Mass`,
  `measurement::Pressure`, and other aliases provide ergonomic field types.
- **Unit families**: `unit::Length`, `unit::Mass`, `unit::Pressure`, and other
  enums expose stable serialized symbols.
- **Type-safe parsing**: parsing a length measurement can only resolve length
  units; a mass unit such as `kg` is rejected in a length context.

### `uom` bridge

- **Infallible `to_uom()`**: persisted measurements can be converted into typed
  `uom` quantities for calculation.
- **Fallible `from_uom()`**: `uom` quantities can be persisted with a requested
  storage unit; conversion back to `Decimal` reports precision errors.
- **Fallible `convert_to()`**: measurements convert between units of the same
  quantity family through `uom`.

### Stable serialization

- **Serde support**: values serialize as `{ "value": "...", "unit": "..." }`.
- **String decimals**: decimals use `rust_decimal::serde::str` to preserve the
  textual decimal value.
- **Stable unit symbols**: units serialize to compact symbols such as `cm`,
  `kg`, `kPa`, `mW`, and `cm/s`.
- **Input aliases**: parsers accept aliases such as `um`, `m2`, `m^3`,
  `mmHg`, `mph`, and `year`; serialization keeps canonical symbols such as
  `µm`, `m²`, `m³`, `mm Hg`, `mi/h`, and `a`.

### Focused public API

- **`Measurement<U>`**: generic persisted measurement wrapper.
- **`Unit`**: trait implemented by each supported unit family.
- **`measurement::*`**: quantity-specific aliases for persisted values.
- **`unit::*`**: quantity-specific unit families and stable symbols.
- **`MeasurementError`**: typed errors for parsing, unknown units, and decimal
  conversion failures.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
qubit-measure = "0.2"
```

## Quick Start

### Creating a typed measurement

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

### Serializing a persisted value

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

### Parsing a typed measurement

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

### Converting units through `uom`

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

### Passing values into `uom`

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

### Persisting a `uom` quantity with a target unit

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

### Using production units

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

## API Reference

### Core types

| Type | Description |
|------|-------------|
| `Measurement<U>` | Generic persisted value plus typed unit |
| `Unit` | Trait implemented by supported unit families |
| `MeasurementError` | Error type for parsing, unknown units, and decimal conversion |
| `measurement::*` | Type aliases such as `measurement::Length` |
| `unit::*` | Unit enums such as `unit::Length` |

### `Measurement<U>` operations

| Method | Description | Error behavior |
|--------|-------------|----------------|
| `new(value, unit)` | Create a persisted measurement | Infallible |
| `quantity_name()` | Return the lower-case quantity name | Infallible |
| `to_uom()` | Convert into the typed `uom` quantity | Infallible |
| `from_uom(quantity, unit)` | Persist a `uom` quantity in the requested unit | Fails if the resulting `f64` cannot be represented as `Decimal` |
| `convert_to(target)` | Convert to another unit in the same family | Same as `from_uom()` |
| `to_string()` | Format as `<value> <unit>` | Infallible |
| `FromStr` | Parse `<decimal><unit>` or `<decimal> <unit>` | Fails on invalid decimals or units outside the typed family |

### `Unit` operations

| Method | Description |
|--------|-------------|
| `all()` | Return all supported units in the family |
| `symbol()` | Return the canonical serialized symbol |
| `to_uom(value)` | Build a `uom` quantity from a decimal value in this unit |
| `value_from_uom(quantity)` | Extract a decimal value from a `uom` quantity in this unit |

## Supported Quantity Families

Voltage is modeled by the SI electric potential quantity: use
`unit::ElectricPotential`; `measurement::Voltage` is an ergonomic alias of
`measurement::ElectricPotential`.

| Measurement alias | Unit family | Examples |
|-------------------|-------------|----------|
| `measurement::Length` | `unit::Length` | `µm`, `mm`, `cm`, `m`, `km`, `in`, `ft`, `mi` |
| `measurement::Area` | `unit::Area` | `mm²`, `cm²`, `m²`, `km²`, `ha`, `ac`, `ft²` |
| `measurement::Volume` | `unit::Volume` | `mm³`, `cm³`, `m³`, `µL`, `mL`, `L`, `gal` |
| `measurement::Mass` | `unit::Mass` | `µg`, `mg`, `g`, `kg`, `t`, `oz`, `lb` |
| `measurement::Time` | `unit::Time` | `ns`, `µs`, `ms`, `s`, `min`, `h`, `d`, `a` |
| `measurement::Pressure` | `unit::Pressure` | `nPa`, `µPa`, `mPa`, `Pa`, `hPa`, `kPa`, `MPa`, `bar`, `psi` |
| `measurement::Energy` | `unit::Energy` | `J`, `kJ`, `MJ`, `W · h`, `kW · h`, `eV`, `cal`, `Btu` |
| `measurement::Power` | `unit::Power` | `nW`, `µW`, `mW`, `W`, `kW`, `MW`, `hp` |
| `measurement::Velocity` | `unit::Velocity` | `µm/s`, `mm/s`, `cm/s`, `m/s`, `km/h`, `ft/s`, `kn` |
| `measurement::Frequency` | `unit::Frequency` | `Hz`, `kHz`, `MHz`, `GHz` |
| `measurement::MassDensity` | `unit::MassDensity` | `kg/m³`, `g/m³`, `g/cm³`, `lb/ft³`, `lb/gal` |
| `measurement::Temperature` | `unit::Temperature` | `K`, `°C`, `°F`, `°R` |
| `measurement::TemperatureInterval` | `unit::TemperatureInterval` | `K`, `°C`, `°F`, `°R` |
| `measurement::ElectricCurrent` | `unit::ElectricCurrent` | `pA`, `nA`, `µA`, `mA`, `A`, `kA`, `MA` |
| `measurement::ElectricPotential` / `measurement::Voltage` | `unit::ElectricPotential` | `nV`, `µV`, `mV`, `V`, `kV`, `MV` |
| `measurement::ElectricCharge` | `unit::ElectricCharge` | `µC`, `mC`, `C`, `kC`, `A · h`, `mA · h` |
| `measurement::Capacitance` | `unit::Capacitance` | `pF`, `nF`, `µF`, `mF`, `F` |
| `measurement::ElectricalResistance` | `unit::ElectricalResistance` | `µΩ`, `mΩ`, `Ω`, `kΩ`, `MΩ`, `GΩ` |
| `measurement::ElectricalConductance` | `unit::ElectricalConductance` | `µS`, `mS`, `S` |
| `measurement::Inductance` | `unit::Inductance` | `nH`, `µH`, `mH`, `H` |
| `measurement::Force` | `unit::Force` | `mN`, `N`, `kN`, `MN`, `gf`, `kgf`, `lbf` |
| `measurement::Acceleration` | `unit::Acceleration` | `mm/s²`, `m/s²`, `ft/s²`, `g₀` |
| `measurement::Torque` | `unit::Torque` | `mN · m`, `N · m`, `kN · m`, `lbf · ft`, `lbf · in` |
| `measurement::Angle` | `unit::Angle` | `rad`, `°`, `r`, `′`, `″` |
| `measurement::AngularVelocity` | `unit::AngularVelocity` | `rad/s`, `°/s`, `rps`, `rpm` |
| `measurement::VolumeRate` | `unit::VolumeRate` | `m³/s`, `m³/h`, `mL/s`, `L/s`, `L/min`, `gal/min` |
| `measurement::MassRate` | `unit::MassRate` | `mg/s`, `g/s`, `kg/s`, `kg/h`, `t/h`, `lb/h` |
| `measurement::DynamicViscosity` | `unit::DynamicViscosity` | `µPa · s`, `mPa · s`, `Pa · s`, `P`, `cP` |
| `measurement::KinematicViscosity` | `unit::KinematicViscosity` | `mm²/s`, `m²/s`, `St`, `cSt` |
| `measurement::AmountOfSubstance` | `unit::AmountOfSubstance` | `µmol`, `mmol`, `mol`, `kmol`, `particle` |
| `measurement::MolarConcentration` | `unit::MolarConcentration` | `nmol/L`, `µmol/L`, `mmol/L`, `mol/L`, `M`, `mol/m³` |
| `measurement::MassConcentration` | `unit::MassConcentration` | `µg/L`, `mg/L`, `g/L`, `kg/m³`, `mg/dL`, `g/dL` |
| `measurement::CatalyticActivity` | `unit::CatalyticActivity` | `µkat`, `mkat`, `kat`, `U`, `mU` |
| `measurement::Radioactivity` | `unit::Radioactivity` | `Bq`, `kBq`, `MBq`, `Ci`, `mCi`, `µCi`, `dpm` |
| `measurement::ElectricField` | `unit::ElectricField` | `V/m`, `V/cm`, `V/mm`, `V/µm`, `kV/mm`, `MV/m` |
| `measurement::ElectricCurrentDensity` | `unit::ElectricCurrentDensity` | `A/m²`, `A/cm²`, `A/mm²` |
| `measurement::ElectricalConductivity` | `unit::ElectricalConductivity` | `S/m`, `S/cm` |
| `measurement::ElectricalResistivity` | `unit::ElectricalResistivity` | `mΩ · m`, `Ω · m`, `Ω · cm`, `Ω · mm²/m` |
| `measurement::MagneticFluxDensity` | `unit::MagneticFluxDensity` | `nT`, `µT`, `mT`, `T`, `G` |
| `measurement::MagneticFlux` | `unit::MagneticFlux` | `µWb`, `mWb`, `Wb`, `Mx` |
| `measurement::MagneticFieldStrength` | `unit::MagneticFieldStrength` | `A/m`, `A/cm`, `Oe` |
| `measurement::HeatCapacity` | `unit::HeatCapacity` | `J/K`, `kJ/K`, `J/°C`, `cal/K`, `Btu/°F` |
| `measurement::SpecificHeatCapacity` | `unit::SpecificHeatCapacity` | `J/(kg · K)`, `kJ/(kg · K)`, `J/(g · °C)`, `cal/(g · K)`, `Btu/(lb · °F)` |
| `measurement::ThermalConductivity` | `unit::ThermalConductivity` | `mW/(m · K)`, `W/(m · K)`, `kW/(m · K)`, `W/(m · °C)` |
| `measurement::ThermalResistance` | `unit::ThermalResistance` | `K/mW`, `K/W`, `K/kW` |
| `measurement::HeatFluxDensity` | `unit::HeatFluxDensity` | `mW/m²`, `W/m²`, `kW/m²`, `W/cm²` |
| `measurement::SurfaceTension` | `unit::SurfaceTension` | `mN/m`, `N/m`, `dyn/cm`, `J/m²` |
| `measurement::LuminousIntensity` | `unit::LuminousIntensity` | `mcd`, `cd`, `kcd` |
| `measurement::Illuminance` | `unit::Illuminance` | `lx`, `klx`, `fc` |
| `measurement::Luminance` | `unit::Luminance` | `cd/m²`, `cd/cm²`, `cd/ft²`, `fl`, `sb` |
| `measurement::SolidAngle` | `unit::SolidAngle` | `sr`, `sp`, `°²` |
| `measurement::Molality` | `unit::Molality` | `mol/kg` |
| `measurement::MolarMass` | `unit::MolarMass` | `mg/mol`, `g/mol`, `kg/mol` |
| `measurement::MolarVolume` | `unit::MolarVolume` | `cm³/mol`, `dm³/mol`, `m³/mol` |
| `measurement::CatalyticActivityConcentration` | `unit::CatalyticActivityConcentration` | `kat/m³`, `U/L`, `mU/mL` |
| `measurement::SpecificRadioactivity` | `unit::SpecificRadioactivity` | `Bq/kg`, `Ci/kg`, `dpm/kg` |

## Persistence Strategy

### What should be persisted

Persist Qubit Measure values where the selected unit has business meaning:

- database rows;
- JSON APIs;
- spreadsheet import/export;
- UI form values;
- rule configuration;
- audit trails;
- source-data reconciliation.

### What should not be persisted as physical measurement

Keep business counts and calendar concepts outside this crate unless they map to
a fixed `uom` physical quantity:

- pieces, sheets, boxes, rolls, batches, lots, packages;
- calendar months or business periods;
- domain-specific package specifications.

For example, `month` is not a fixed physical duration and is therefore not a
`unit::Time` variant. It belongs in a calendar or business-period model.

### Relationship with `uom`

Use `uom` directly inside calculation-heavy code. Use Qubit Measure at runtime
boundaries where a user-selected or source-selected unit must remain explicit.
The crate is intentionally a wrapper around `uom`, not a replacement dimension
system.

## Precision and Conversion Notes

| Direction | Numeric representation | Notes |
|-----------|------------------------|-------|
| Persisted value | `rust_decimal::Decimal` | Good for stable textual persistence |
| `to_uom()` | `f64` `uom` quantity | Infallible because `Decimal` is finite and within the `f64` exponent range |
| `from_uom()` | `f64` to `Decimal` | Fallible for NaN, infinity, or values not representable by `Decimal` |
| `convert_to()` | `Decimal -> uom -> Decimal` | Uses `uom` as the conversion source of truth |

## Testing & Code Coverage

This project keeps behavior covered through integration tests and local CI
scripts.

### Running tests

```bash
# Run all tests
cargo test --all-features

# Run the integration test entry point
cargo test --test mod

# Run style checks
./style-check.sh

# Run coverage
./coverage.sh

# Run the full local CI pipeline
./ci-check.sh
```

### Coverage metrics

Coverage reports are generated under `target/llvm-cov/` by `./coverage.sh`.
The published coverage badge links to the GitHub Pages coverage report.

## Dependencies

Runtime dependencies are intentionally focused:

- `uom` provides the type-safe SI quantity system and unit conversion.
- `rust_decimal` stores stable decimal boundary values.
- `serde` serializes persisted measurements and unit symbols.
- `thiserror` defines the public error type.

## License

Copyright (c) 2026 Haixing Hu.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

See [LICENSE](LICENSE) for the full license text.

## Contributing

Contributions are welcome. Please run the local checks before opening a pull
request:

```bash
./align-ci.sh
./ci-check.sh
```

### Development Guidelines

- Keep `uom` as the conversion source of truth.
- Add new physical quantity families as typed `Measurement<U>` aliases and
  `unit::*` families.
- Unit enums are non-exhaustive. Downstream code should include a wildcard
  match arm, and this crate can add units by extending the relevant `unit::*`
  macro invocation with a canonical symbol plus parse aliases.
- Do not mix business count units into physical measurement families.
- Add focused integration tests for symbols, parsing, serde, and `uom` bridge
  behavior.

## Author

Haixing Hu

## Related Projects

- [`uom`](https://crates.io/crates/uom): type-safe zero-cost dimensional analysis.
- [`rust_decimal`](https://crates.io/crates/rust_decimal): decimal arithmetic for stable boundary values.
