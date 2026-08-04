# Qubit Measure

[![Rust CI](https://github.com/qubit-ltd/rs-measure/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-measure/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-measure/coverage-badge.json)](https://qubit-ltd.github.io/rs-measure/coverage/)
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
qubit-measure = "0.4"
rust_decimal = "1.39"
```

Enable the approximate `f64` bridge explicitly:

```toml
[dependencies]
qubit-measure = { version = "0.4", features = ["uom"] }
rust_decimal = "1.39"
uom = { version = "0.38", default-features = false, features = ["f64", "si", "std"] }
```

```rust
use qubit_measure::{measurement, unit};
use rust_decimal::Decimal;

let length = measurement::Length::new(Decimal::new(500, 1), unit::Length::Centimeter);
let meters = length.convert_to(unit::Length::Meter)?;
assert_eq!(meters.value, Decimal::new(5, 1));
# Ok::<(), qubit_measure::MeasurementError>(())
```

`Measurement<U>` stores a `Decimal` and a typed unit. The aliases in
`measurement::*` and enums in `unit::*` cover 57 quantity families.

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
`value` is a Decimal string, and `unit` is always serialized with and
deserialized from its canonical symbol. Deserialization rejects aliases and a
quantity that does not match the requested Rust type. Extra fields are ignored
for forward-compatible metadata additions. Each decoded string field is limited
to 1,048,576 UTF-8 bytes after Serde has constructed it. This bounds accepted
field text and subsequent parsing work; it is not a transport-payload or
pre-allocation limit. Configure those limits at the transport or deserializer
boundary.

For configuration fields that intentionally use compact text, opt in with
`#[serde(with = "qubit_measure::measurement_text")]`. The adapter serializes
canonical strings such as `"2 MiB"` and uses strict parsing when decoding; it
does not change the default three-field representation. Optional fields use
`#[serde(default, with = "qubit_measure::measurement_text::option")]`; present
values remain canonical strings, while absent values use `null`.

## 3. Decimal precision and rounding

`convert_to` never converts the persisted value, factors, offsets, or intermediate
results through `f64`. Unit coefficients are validated Decimal ratios, so definitions
such as `5 / 9`, exact SI prefixes, and exact customary units are not rounded when
declared. Built-in factors and offsets live in one crate-internal `consts.rs`, grouped
by quantity. Exact and derived definitions follow the
[BIPM SI Brochure](https://www.bipm.org/en/publications/si-brochure),
[NIST conversion references](https://www.nist.gov/pml/owm/metric-si/unit-conversion),
[NIST Handbook 44](https://www.nist.gov/pml/owm/nist-handbook-44-current-edition),
and [2022 CODATA](https://physics.nist.gov/cuu/Constants/). Irrational values are
finite approximations: pi uses 23 decimal places from
[NIST DLMF section 3.12](https://dlmf.nist.gov/3.12), the finite tau is exactly twice
that value, and square degree uses 28 decimal places. The versioned source set,
numeric policy, and coverage scope for every built-in quantity family are recorded
in the machine-readable [unit-definition provenance manifest](doc/unit-definition-provenance.tsv).

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

`ConversionOptions::maximum_precision()` does not request a fixed output scale.
Conversion arithmetic first remains an exact rational calculation. If the result is
not exactly representable, the crate rounds to nearest-even at the greatest Decimal
scale whose mantissa fits, then normalizes trailing zeroes. `rounding() == None` means
that no fixed-scale strategy was selected; it does not mean that representation-boundary
rounding is impossible. `fixed_scale(0..=28, strategy)` instead rounds and retains
exactly the requested number of decimal places. Values outside Decimal's range return
`ValueOutOfRange`; a value that cannot retain the requested scale returns
`OutputScaleUnrepresentable`. When the source and target definitions are
mathematically equivalent, maximum-precision conversion is a no-op and preserves
the original Decimal scale and trailing zeroes.

Derived `PartialEq` and `Eq` compare the stored Decimal and unit fields, so
`1 m != 100 cm` structurally. Use `equivalent_to` for exact physical equality
or `try_cmp_exact` for exact cross-unit ordering. Both methods stay in rational
arithmetic and never apply the Decimal output-rounding policy.

## 4. Deterministic defaults

`convert_to` always uses the immutable `ConversionOptions::DEFAULT`: no requested
fixed scale, nearest-even at the representation boundary, and normalized output except
for the equal-definition no-op described above.
There is no process-wide mutable conversion state. Code that needs a fixed output
scale and rounding strategy uses `convert_to_with_options` explicitly.

## 5. Strict and lenient parsing

`Unit::parse_strict` accepts canonical symbols only. A recognized alias produces
`NonCanonicalUnit` with the canonical replacement. `FromStr`,
`Measurement::from_str`, and default Serde deserialization use this strict contract.
Call `Unit::parse_lenient` or `Measurement::parse_lenient` explicitly when documented
aliases should be accepted. Canonical symbols and aliases must be disjoint.

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

`Time::Minute` also accepts the lenient alias `m`, while display, strict parsing,
and Serde keep the canonical `min`. Compact measurements are matched against
known unit suffixes; input with multiple valid numeric/unit splits returns
`AmbiguousMeasurement` instead of silently choosing one interpretation.
Unit symbols or aliases beginning with `.`, `+`, or `-` must be separated from
the Decimal value by whitespace; their compact forms are rejected as ambiguous
numeric boundaries (for example, use `1.25 +cu`).

Measurement values accept ordinary and scientific Decimal text. Exactness is
decided from the final value, so inputs such as `1.0e-28 m` are accepted, while
values requiring rounding return `UnrepresentableMeasurementValue`. The parser
preserves as much input scale as Decimal can represent. Malformed text returns
`InvalidMeasurementSyntax`.

Default parsing and `FromStr` reject measurement text above 1,048,576 UTF-8
bytes. Use explicit options to choose a smaller or larger limit:

```rust
use qubit_measure::{MeasurementParseOptions, measurement};

let options = MeasurementParseOptions::default().with_max_text_bytes(64);
let length = measurement::Length::parse_strict_with_options("1.00e0 m", &options)?;
assert_eq!(length.value.scale(), 2);
# Ok::<(), qubit_measure::MeasurementError>(())
```

Oversized input returns `MeasurementTextLimitExceeded` before Decimal or unit
scanning begins.

### Exact `std::time::Duration` adapters

`Measurement<Time>` converts to and from `std::time::Duration` through standard
`From` and `TryFrom` implementations. Conversion is exact to one nanosecond and
rejects negative, subnanosecond, and out-of-range measurements without rounding.

This adapter is not a replacement for the workspace Duration codecs. `Measurement<Time>`
uses the three-field measurement record `{"quantity","value","unit"}` and accepts only
the canonical unit symbol during Serde deserialization. Use `qubit-datatype` and
`qubit-serde` for their established non-negative `Duration` formats: compact exact unit
text or the explicitly lossy whole-millisecond representations used by downstream crates.

### Exact information-size adapters

`Information` supports bit and byte, decimal `kB` through `TB`, and binary
`KiB` through `TiB`. Definitions follow IEC 80000-13:2025 and use byte as the
exact base unit. `u64::try_from` and `usize::try_from` return exact whole-byte
counts; negative, fractional-byte, and out-of-range values are rejected without
rounding. Conversely, `From<u64>` constructs a byte-based information
measurement without loss.

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
assuming this variant uses it. The optional `uom` bridge applies this exact
definition through the Pascal base value instead of using `uom`'s conventional
millimeter-of-mercury coefficient. See [NIST SP 811 Chapter 5](https://www.nist.gov/pml/special-publication-811/nist-guide-si-chapter-5-units-outside-si)
and [Appendix B.9](https://www.nist.gov/pml/special-publication-811/nist-guide-si-appendix-b-conversion-factors/nist-guide-si-appendix-b9).

## 7. External unit families

`Unit`, `ConversionFactor`, and `UnitDefinition` are public. The exported macro
defines compile-time families without a runtime registry or a mandatory `uom` mapping.
The expansion resolves `rust_decimal` and `serde` in the consumer crate, so
external families declare both dependencies directly:

```toml
[dependencies]
qubit-measure = "0.4"
rust_decimal = "1.39"
serde = "1.0"
```

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
and exact definitions. An optional `uom` bridge is added separately with
`impl_uom_unit!`, under the consumer's own feature configuration. External code
may also implement `Unit` manually. Measurement
Serde uses the `Unit` symbol and parsing contract directly, so a manual unit does not
need separate `Serialize` or `Deserialize` implementations.

Every unit family follows this metadata contract:

- `quantity` is non-empty ASCII `snake_case`, begins with a lowercase letter,
  and has no leading, trailing, or repeated underscores;
- canonical symbols are non-empty, unique, and contain no leading or trailing
  Unicode whitespace;
- aliases are non-empty, unique among aliases, and contain no leading or
  trailing Unicode whitespace;
- aliases do not match any canonical symbol in the family;
- macro-generated families are checked at compilation;
- manual `Unit` implementations should call `assert_unit_family_valid` in tests;
- stable Rust cannot prove that a manual enum omitted no variant from `all()`.

## 8. Approximate `uom` bridge

This bridge is available only with the default-off `uom` Cargo feature. Without
that feature, `UomUnit`, `try_to_uom_approx`, `to_uom_approx`, and
`from_uom_approx` are absent.
Families mapped to `uom` implement `UomUnit` when enabled. The `_approx` suffix
is intentional: each adapter first applies the `qubit-measure` exact definition
to obtain the SI base value, then crosses `Decimal <-> f64` and may lose
precision. This keeps the quantity's physical base value aligned with the exact
Decimal core. A later getter for a non-base `uom` unit still follows `uom`'s own
coefficient, so its displayed number may differ when the two libraries define
that named unit differently. Persisted unit conversion through `convert_to`
does not use this bridge.

External `UomUnit` implementations must implement `try_to_uom_approx`, which
returns the definition error. The default `to_uom_approx` convenience wrapper
panics whenever that fallible method returns an error.

Consumers that name `uom` types declare `uom` directly:

```toml
[dependencies]
qubit-measure = { version = "0.4", features = ["uom"] }
rust_decimal = "1.39"
uom = { version = "0.38", default-features = false, features = ["f64", "si", "std"] }
```

```rust
use qubit_measure::{measurement, unit};
use rust_decimal::Decimal;
use uom::si::length::meter;

let value = measurement::Length::new(Decimal::new(50, 0), unit::Length::Centimeter);
assert_eq!(value.to_uom_approx().get::<meter>(), 0.5);
```

Use `uom` for dimensional calculations where binary floating-point behavior is
appropriate, then explicitly adapt the result at the persistence boundary.

## Testing

```bash
# Run tests with the default feature set
cargo test

# Run tests with all declared features
cargo test --all-features

# Project CI checks
./ci-check.sh

# Check code coverage
./coverage.sh
```

## License

Copyright (c) 2025 - 2026. Haixing Hu. All rights reserved.

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for the
full license text.

## Contributing

Contributions are welcome. Please follow the Rust API guidelines, keep public
API documentation and tests current, and run `./align-ci.sh` to format code and
`./ci-check.sh` to satisfy CI requirements before submitting a pull request.

## Author

**Haixing Hu** - *Qubit Co. Ltd.*

Repository: [https://github.com/qubit-ltd/rs-measure](https://github.com/qubit-ltd/rs-measure)
