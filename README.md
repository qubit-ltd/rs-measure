# Qubit Measure

[![Rust CI](https://github.com/qubit-ltd/rs-measure/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-measure/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/qubit-measure.svg?color=blue)](https://crates.io/crates/qubit-measure)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![中文文档](https://img.shields.io/badge/文档-中文版-blue.svg)](README.zh_CN.md)

Persistent typed measurements with Decimal-only unit conversion, explicit units,
and an optional approximate `uom` bridge.

## 1. Installation and quick start

The exact Decimal core is the default and does not compile `uom`:

```toml
[dependencies]
qubit-measure = "0.3"
```

Enable the approximate `f64` bridge explicitly:

```toml
[dependencies]
qubit-measure = { version = "0.3", features = ["uom"] }
```

```rust
use qubit_measure::{Decimal, measurement, unit};

let length = measurement::Length::new(Decimal::new(500, 1), unit::Length::Centimeter);
let meters = length.convert_to(unit::Length::Meter)?;
assert_eq!(meters.value, Decimal::new(5, 1));
# Ok::<(), qubit_measure::MeasurementError>(())
```

`Measurement<U>` stores a `Decimal` and a typed unit. The aliases in
`measurement::*` and enums in `unit::*` cover 56 physical quantity families.

## 2. Three-field JSON contract

Serde uses a quantity-aware wire format:

```json
{
  "quantity": "length",
  "value": "50.0",
  "unit": "cm"
}
```

All three fields are required. `quantity` is a stable `snake_case` identifier,
`value` is a Decimal string, and `unit` is always serialized with its canonical
symbol. Deserialization rejects a quantity that does not match the requested Rust
type. Extra fields are ignored for forward-compatible metadata additions.

## 3. Decimal precision and rounding

`convert_to` never converts the persisted value, factors, offsets, or intermediate
results through `f64`. Unit coefficients are validated Decimal ratios, so definitions
such as `5 / 9`, exact SI prefixes, and exact customary units are not rounded when
declared. Built-in factors and offsets live in one crate-internal `consts.rs`, grouped
by quantity. Mathematical values use Rust standard-library constants when available;
for example, angle conversions derive from `std::f64::consts::PI` and
`std::f64::consts::TAU`, with compile-time checks keeping their finite Decimal
representations coherent.

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

`ConversionOptions::maximum_precision(strategy)` adds no final scale reduction.
`fixed_scale(0..=28, strategy)` rounds and retains exactly the requested number of
decimal places. Decimal still has a finite 96-bit mantissa: repeating fractions,
irrational constants, and results outside its range cannot have infinite precision.
Arithmetic and unrepresentable scale requests return `MeasurementError`.

## 4. Deterministic defaults

`convert_to` always uses the immutable `ConversionOptions::DEFAULT`: maximum
precision with `MidpointNearestEven`. There is no process-wide mutable conversion
state. Code that needs a fixed output scale or another rounding strategy uses
`convert_to_with_options` explicitly.

## 5. Strict and lenient parsing

`Unit::parse_strict` accepts canonical symbols only. A recognized alias produces
`NonCanonicalUnit` with the canonical replacement. `Unit::parse_lenient`, `FromStr`,
`Measurement::from_str`, and default Serde deserialization accept documented aliases.
Canonical symbols always take precedence when they collide with another unit's alias.
`Measurement::parse_strict` provides strict parsing for complete values.

```rust
use qubit_measure::{Unit, unit};

assert_eq!(unit::Time::parse_lenient("year")?, unit::Time::CommonYear365);
assert!(unit::Time::parse_strict("year").is_err());
assert_eq!(unit::Time::parse_strict("a (365 d)")?, unit::Time::CommonYear365);
# Ok::<(), qubit_measure::MeasurementError>(())
```

## 6. Ambiguous unit aliases

Ambiguous concepts use qualified enum names and canonical symbols. Common input
strings remain available only in lenient parsing.

| Quantity | Explicit variant | Canonical symbol | Lenient aliases |
| --- | --- | --- | --- |
| Time | `CommonYear365` | `a (365 d)` | `a`, `yr`, `year` |
| Energy | `ThermochemicalCalorie` | `cal (th)` | `cal` |
| Energy | `ThermochemicalKilocalorie` | `kcal (th)` | `kcal` |
| Energy | `BritishThermalUnitInternationalTable` | `Btu (IT)` | `Btu`, `BTU` |
| Power | `MechanicalHorsepower` | `hp (mechanical)` | `hp` |
| Volume | `UsFluidOunce` | `fl oz (US)` | `fl oz` |
| Volume | `UsCustomaryCup` | `cup (US customary)` | `cup` |
| Volume | `UsLiquidPint` | `pt (US liq)` | `liq pt` |
| Volume | `UsLiquidQuart` | `qt (US liq)` | `liq qt` |
| Volume | `UsLiquidGallon` | `gal (US)` | `gal` |
| Volume rate | `UsGallonPerMinute` | `gal (US)/min` | `gal/min` |
| Mass density | `PoundPerUsGallon` | `lb/gal (US)` | `lb/gal` |

Calorie and Btu variants in heat-capacity families use the same thermochemical
and International Table qualifiers. `CommonYear365` is exactly 31,536,000 seconds;
it is a fixed duration, not a calendar model.

`MillimeterOfMercury` uses the exact Torr-equivalent definition
`101325 / 760 Pa` (`20265 / 152 Pa`). Its canonical symbol is `mm Hg`, and
`mmHg` is accepted as a lenient alias. This differs from the conventional
rounded `133.3224 Pa` value used by some conversion tables. Applications that
require that conventional value must define an external unit instead of
assuming this variant uses it. The optional `uom` bridge uses a private
Torr-equivalent marker instead of `uom`'s conventional millimeter-of-mercury
coefficient, so the bridge preserves the same semantic. See [NIST SP 811 Chapter 5](https://www.nist.gov/pml/special-publication-811/nist-guide-si-chapter-5-units-outside-si)
and [Appendix B.9](https://www.nist.gov/pml/special-publication-811/nist-guide-si-appendix-b-conversion-factors/nist-guide-si-appendix-b9).

## 7. External unit families

`Unit`, `ConversionFactor`, and `UnitDefinition` are public. The exported macro
defines compile-time families without a runtime registry or a mandatory `uom` mapping.

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

The macro generates canonical display, strict and lenient parsing, Serde, enumeration,
and exact definitions. External code may also implement `Unit` manually. Measurement
Serde uses the `Unit` symbol and parsing contract directly, so a manual unit does not
need separate `Serialize` or `Deserialize` implementations.

Every unit family follows this metadata contract:

- `quantity` is non-empty ASCII `snake_case`, begins with a lowercase letter,
  and has no leading, trailing, or repeated underscores;
- canonical symbols are non-empty, unique, and contain no leading or trailing
  Unicode whitespace;
- aliases are non-empty, unique among aliases, and contain no leading or
  trailing Unicode whitespace;
- an alias may equal another variant's canonical symbol;
- canonical symbols are checked first and therefore win;
- macro-generated families are checked at compilation;
- manual `Unit` implementations should call `assert_unit_family_valid` in tests;
- stable Rust cannot prove that a manual enum omitted no variant from `all()`.

## 8. Approximate `uom` bridge

This bridge is available only with the default-off `uom` Cargo feature. Without
that feature, `UomUnit`, `to_uom_approx`, and `from_uom_approx` are absent.
Families mapped to `uom` implement `UomUnit` when enabled. The `_approx` suffix
is intentional: these adapters cross `Decimal <-> f64` and may lose precision.
Persisted unit conversion through `convert_to` does not use this bridge.

```rust
use qubit_measure::{Decimal, measurement, unit};
use uom::si::length::meter;

let value = measurement::Length::new(Decimal::new(50, 0), unit::Length::Centimeter);
assert_eq!(value.to_uom_approx().get::<meter>(), 0.5);
```

Use `uom` for dimensional calculations where binary floating-point behavior is
appropriate, then explicitly adapt the result at the persistence boundary.

## 9. Migration from 0.2 to 0.3

| 0.2 | 0.3 |
| --- | --- |
| JSON `{value, unit}` | JSON `{quantity, value, unit}` |
| `convert_to` routed through `uom/f64` | Decimal-only exact-factor conversion |
| `to_uom` / `from_uom` | `to_uom_approx` / `from_uom_approx` |
| Ambiguous variants such as `Year`, `Gallon`, `Horsepower` | Qualified variants listed above |
| Canonical ambiguous short symbols | Qualified canonical symbols; short forms are lenient aliases |
| `Unit` included `uom` methods | Exact `Unit` plus optional `UomUnit` |
| Unit families were crate-internal | Public `define_unit_family!` and manual implementations |

This release intentionally breaks the 0.2 wire format and affected Rust APIs.

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE).
