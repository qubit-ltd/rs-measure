# rs-measure Decimal Conversion Implementation Plan

> **Historical plan:** The implemented design was subsequently revised. The
> current crate has no mutable process-wide conversion default or `parking_lot`
> dependency; `convert_to` always uses `ConversionOptions::DEFAULT`. Built-in
> factors now live in `src/consts.rs`, Measurement owns its string unit wire
> contract, and canonical symbols take precedence over aliases. Treat current
> source and README files as authoritative.

> **For agentic workers:** REQUIRED SUB-SKILL: use `superpowers:executing-plans` to implement this plan task-by-task. Do not dispatch subagents unless the user explicitly authorizes delegation.

**Goal:** Release `qubit-measure` 0.3.0 with Decimal-only persisted unit conversion, configurable rounding, quantity-aware JSON, explicit ambiguous-unit semantics, and public compile-time unit extension.

**Architecture:** Keep `Measurement<U>` as the persisted value, split exact `Unit` metadata from optional `UomUnit` approximation, and route `convert_to` through validated Decimal rational factors. A `parking_lot::Mutex<ConversionOptions>` supplies process defaults while explicit options remain deterministic and independent of global state.

**Tech Stack:** Rust 2024, rust-version 1.94, `rust_decimal` 1.39+, `parking_lot`, Serde, thiserror, uom 0.38, project CI scripts.

## Global Constraints

- Follow `docs/superpowers/specs/2026-07-15-rs-measure-decimal-conversion-design.md` exactly.
- `Measurement::convert_to` must not convert any value or factor through `f64`.
- `uom/f64` is available only through APIs whose names end in `_approx`.
- Process defaults use `parking_lot::Mutex<ConversionOptions>` and are snapshotted once per conversion.
- Default options are maximum Decimal precision and `RoundingStrategy::MidpointNearestEven`.
- JSON requires `quantity`, string `value`, and canonical `unit`; quantity matching is strict.
- `FromStr` and default Deserialize are lenient; explicit strict APIs accept canonical symbols only.
- External extension is compile-time only; no runtime registry.
- Breaking changes are allowed; the release version is `0.3.0`.
- Do not run `git add`, `git commit`, or `git push` without a separate explicit user request. Each task ends with a review checkpoint instead of a commit.

## File Structure

Create focused core modules:

- `src/measure/conversion_options.rs`: options type and process-global default accessors.
- `src/measure/unit_definition.rs`: validated Decimal rational factor and offset types.
- `src/measure/decimal_conversion.rs`: Decimal-only conversion and final-scale logic.
- `src/measure/uom_unit.rs`: optional approximate uom bridge trait.
- `tests/measure/conversion_options_tests.rs`: option validation and global-state behavior.
- `tests/measure/decimal_conversion_tests.rs`: exact arithmetic and rounding behavior.
- `tests/measure/external_unit_tests.rs`: downstream-style public macro and manual trait implementations.
- `tests/measure/unit_definition_tests.rs`: independently declared golden unit definitions.

Modify existing boundaries rather than adding unrelated abstractions:

- `src/measure/unit.rs`: exact unit metadata and strict/lenient parsing.
- `src/measure/units.rs`: exported unit macro, transitional legacy macro, and approximate conversion helpers.
- `src/measure/measurement.rs`: exact conversion, custom Serde, strict text parsing, approximate adapters.
- `src/measure/measurement_error.rs`: new precise error variants.
- `src/measure/mod.rs`, `src/lib.rs`, `src/unit.rs`: public exports.
- All files under `src/measure/units/`: exact factors, offsets, aliases, and optional uom mappings.
- Existing test modules: migration of API names and assertions.
- `README.md`, `README.zh_CN.md`, `examples/basic_usage.rs`, `Cargo.toml`: 0.3 public contract.

---

### Task 1: Conversion Options, Global Defaults, and Error Surface

**Files:**
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`
- Create: `src/measure/conversion_options.rs`
- Modify: `src/measure/measurement_error.rs`
- Modify: `src/measure/mod.rs`
- Modify: `src/lib.rs`
- Create: `tests/measure/conversion_options_tests.rs`
- Modify: `tests/measure/mod.rs`
- Modify: `tests/measure/measurement_error_tests.rs`

**Interfaces:**
- Produces: `ConversionOptions::new`, `ConversionOptions::maximum_precision`, `ConversionOptions::fixed_scale`, `scale`, `rounding`, `default_conversion_options`, `set_default_conversion_options`.
- Produces errors: `InvalidScale`, `ArithmeticOverflow`, `InvalidUnitDefinition`, `NonCanonicalUnit`, `QuantityMismatch`.
- Later tasks consume these exact public names.

- [ ] **Step 1: Add failing option and error tests**

Add the module declaration and tests with these behaviors:

```rust
use qubit_measure::{
    ConversionOptions,
    MeasurementError,
    RoundingStrategy,
    default_conversion_options,
    set_default_conversion_options,
};

#[test]
fn test_conversion_options_reject_scale_above_decimal_limit() {
    assert_eq!(
        ConversionOptions::fixed_scale(
            29,
            RoundingStrategy::MidpointNearestEven,
        ),
        Err(MeasurementError::InvalidScale { scale: 29, max: 28 }),
    );
}

#[test]
fn test_global_conversion_options_replace_and_restore() {
    let original = default_conversion_options();
    let replacement = ConversionOptions::fixed_scale(
        6,
        RoundingStrategy::MidpointAwayFromZero,
    )
    .expect("scale should be valid");

    assert_eq!(set_default_conversion_options(replacement), original);
    assert_eq!(default_conversion_options(), replacement);
    assert_eq!(set_default_conversion_options(original), replacement);
}
```

Extend `measurement_error_tests.rs` with equality and display assertions for every new variant.

- [ ] **Step 2: Run the focused tests and verify the intended failure**

Run:

```bash
cargo test --test mod measure::conversion_options_tests -- --nocapture
```

Expected: compile failure because `ConversionOptions`, `RoundingStrategy`, and the global functions are not exported.

- [ ] **Step 3: Add dependencies and implement the option model**

Update dependencies:

```toml
rust_decimal = { version = "1.39", features = ["macros", "serde-with-str"] }
parking_lot = "0.12"
```

Implement this public shape in `conversion_options.rs`:

```rust
use crate::measure::MeasurementError;
use parking_lot::Mutex;
use rust_decimal::{Decimal, RoundingStrategy};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConversionOptions {
    scale: Option<u32>,
    rounding: RoundingStrategy,
}

impl ConversionOptions {
    pub const DEFAULT: Self = Self {
        scale: None,
        rounding: RoundingStrategy::MidpointNearestEven,
    };

    pub fn new(
        scale: Option<u32>,
        rounding: RoundingStrategy,
    ) -> Result<Self, MeasurementError>;

    #[must_use]
    pub const fn maximum_precision(rounding: RoundingStrategy) -> Self;

    pub fn fixed_scale(
        scale: u32,
        rounding: RoundingStrategy,
    ) -> Result<Self, MeasurementError>;

    #[must_use]
    pub const fn scale(self) -> Option<u32>;

    #[must_use]
    pub const fn rounding(self) -> RoundingStrategy;
}

impl Default for ConversionOptions {
    fn default() -> Self {
        Self::DEFAULT
    }
}

static DEFAULT_CONVERSION_OPTIONS: Mutex<ConversionOptions> =
    Mutex::new(ConversionOptions::DEFAULT);

#[must_use]
pub fn default_conversion_options() -> ConversionOptions {
    *DEFAULT_CONVERSION_OPTIONS.lock()
}

pub fn set_default_conversion_options(
    options: ConversionOptions,
) -> ConversionOptions {
    std::mem::replace(&mut *DEFAULT_CONVERSION_OPTIONS.lock(), options)
}
```

`new` must reject a scale larger than `Decimal::MAX_SCALE`. Add fully documented error variants with these fields:

```rust
InvalidScale { scale: u32, max: u32 }
ArithmeticOverflow { operation: &'static str }
InvalidUnitDefinition { reason: String }
NonCanonicalUnit { quantity: String, unit: String, canonical: String }
QuantityMismatch { expected: String, actual: String }
```

Re-export `Decimal`, `RoundingStrategy`, `ConversionOptions`, and the two global functions from `src/lib.rs`.

- [ ] **Step 4: Run focused tests and documentation checks**

Run:

```bash
cargo test --test mod measure::conversion_options_tests
cargo test --test mod measure::measurement_error_tests
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
```

Expected: all selected tests pass; rustdoc completes without warnings.

- [ ] **Step 5: Review checkpoint**

Inspect the diff for only dependency, error, option, export, and focused-test changes. Confirm the original global configuration is restored even when an assertion would fail by using a small private restoration guard if necessary.

---

### Task 2: Decimal Rational Factors and Conversion Engine

**Files:**
- Create: `src/measure/unit_definition.rs`
- Create: `src/measure/decimal_conversion.rs`
- Modify: `src/measure/mod.rs`
- Modify: `src/lib.rs`
- Create: `tests/measure/decimal_conversion_tests.rs`
- Modify: `tests/measure/mod.rs`

**Interfaces:**
- Consumes: `ConversionOptions`, `MeasurementError` from Task 1.
- Produces: `ConversionFactor::new`, `ConversionFactor::from_integer`, accessors, `UnitDefinition::new`, `UnitDefinition::base`, and public `UnitDefinition::convert_value_to`.

- [ ] **Step 1: Write failing exact arithmetic tests**

Cover finite, rational, offset, rounding, identical-unit, and range behavior:

```rust
#[test]
fn test_decimal_conversion_keeps_five_ninths_as_a_ratio() {
    let fahrenheit = UnitDefinition::new(
        ConversionFactor::new(dec!(5), dec!(9)).unwrap(),
        dec!(459.67),
    );
    let kelvin = UnitDefinition::base();
    let options = ConversionOptions::maximum_precision(
        RoundingStrategy::MidpointNearestEven,
    );

    assert_eq!(
        fahrenheit.convert_value_to(dec!(32), kelvin, options).unwrap(),
        dec!(273.15),
    );
}

#[test]
fn test_decimal_conversion_applies_requested_scale() {
    let meter = UnitDefinition::base();
    let foot = UnitDefinition::new(
        ConversionFactor::new(dec!(381), dec!(1250)).unwrap(),
        Decimal::ZERO,
    );
    let options = ConversionOptions::fixed_scale(
        4,
        RoundingStrategy::MidpointNearestEven,
    )
    .unwrap();

    let result = meter.convert_value_to(dec!(1), foot, options).unwrap();
    assert_eq!(result, dec!(3.2808));
    assert_eq!(result.scale(), 4);
}
```

Also test zero/negative factors, exact same-definition preservation with `scale = None`, same-definition rounding with `Some(n)`, and inability to attach a requested scale to `Decimal::MAX`.

- [ ] **Step 2: Run the test and verify missing-type failures**

Run:

```bash
cargo test --test mod measure::decimal_conversion_tests -- --nocapture
```

Expected: compile failure because the factor, definition, and conversion engine do not exist.

- [ ] **Step 3: Implement validated factor and definition types**

Implement this shape with private fields and documented accessors:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConversionFactor {
    numerator: Decimal,
    denominator: Decimal,
}

impl ConversionFactor {
    pub fn new(
        numerator: Decimal,
        denominator: Decimal,
    ) -> Result<Self, MeasurementError>;

    pub fn from_integer(value: Decimal) -> Result<Self, MeasurementError>;
    pub const fn numerator(self) -> Decimal;
    pub const fn denominator(self) -> Decimal;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitDefinition {
    factor: ConversionFactor,
    offset: Decimal,
}

impl UnitDefinition {
    pub const fn new(factor: ConversionFactor, offset: Decimal) -> Self;
    pub fn base() -> Self;
    pub const fn factor(self) -> ConversionFactor;
    pub const fn offset(self) -> Decimal;
}
```

Reject non-positive numerator or denominator with `InvalidUnitDefinition` and a concrete reason string.

- [ ] **Step 4: Implement Decimal-only conversion and scale enforcement**

Implement the internal engine:

```rust
pub(crate) fn convert_decimal(
    value: Decimal,
    source: UnitDefinition,
    target: UnitDefinition,
    options: ConversionOptions,
) -> Result<Decimal, MeasurementError>;
```

Expose it through the public, documented delegation method:

```rust
impl UnitDefinition {
    pub fn convert_value_to(
        self,
        value: Decimal,
        target: Self,
        options: ConversionOptions,
    ) -> Result<Decimal, MeasurementError> {
        convert_decimal(value, self, target, options)
    }
}
```

Use checked addition for source offset, try a combined source/target ratio first, and fall back to two ratio applications only if combining factor terms overflows. Apply each ratio with multiply-then-divide first to avoid premature division rounding; only retry divide-then-multiply when the first ordering overflows but the mathematically equivalent result may still fit. Finish with checked target-offset subtraction.

For `scale = Some(n)`, call `round_dp_with_strategy(n, options.rounding())`, then `set_scale(n)`. Map any failure to `ArithmeticOverflow { operation: "set output scale" }`; never silently lower the requested scale. For identical definitions and `scale = None`, return the input unchanged.

- [ ] **Step 5: Run focused tests**

Run:

```bash
cargo test --test mod measure::decimal_conversion_tests
cargo test --test mod measure::conversion_options_tests
```

Expected: all conversion and configuration tests pass.

- [ ] **Step 6: Review checkpoint**

Search the new engine for `f32`, `f64`, `to_f64`, and `from_f64`; expected result is no matches. Confirm all arithmetic failure paths return `MeasurementError` rather than panic.

---

### Task 3: Split Unit from UomUnit and Export the Extension Macro

**Files:**
- Modify: `src/measure/unit.rs`
- Create: `src/measure/uom_unit.rs`
- Modify: `src/measure/units.rs`
- Modify: `src/measure/mod.rs`
- Modify: `src/lib.rs`
- Modify: `src/unit.rs`
- Create: `tests/measure/external_unit_tests.rs`
- Modify: `tests/measure/mod.rs`
- Modify: `tests/measure/unit_tests.rs`

**Interfaces:**
- Consumes: `UnitDefinition`, `MeasurementError`, and Decimal public re-exports.
- Produces: exact `Unit`, optional `UomUnit`, public `define_unit_family!`, strict and lenient parsing.
- Transitional requirement: the old private `define_measurement_unit!` remains until Tasks 4–7 migrate every built-in family.

- [ ] **Step 1: Write failing external-extension and parsing tests**

Define a downstream-style family only through public API:

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

#[test]
fn test_external_family_supports_strict_and_lenient_parsing() {
    assert_eq!(CustomLength::parse_strict("hcu").unwrap(), CustomLength::Half);
    assert!(matches!(
        CustomLength::parse_strict("half-cu"),
        Err(MeasurementError::NonCanonicalUnit { .. }),
    ));
    assert_eq!("half-cu".parse(), Ok(CustomLength::Half));
    assert_eq!(CustomLength::Half.to_string(), "hcu");
}
```

Add a manual `Unit` implementation in the same integration test to prove the trait is not macro-sealed.

- [ ] **Step 2: Run the focused test and verify failure**

Run:

```bash
cargo test --test mod measure::external_unit_tests -- --nocapture
```

Expected: compile failure because the public macro and split traits do not exist.

- [ ] **Step 3: Define the final core traits**

Make `Unit` expose this contract:

```rust
pub trait Unit:
    Copy + Eq + fmt::Display + FromStr<Err = MeasurementError> + 'static
{
    const QUANTITY: &'static str;

    fn all() -> &'static [Self];
    fn symbol(self) -> &'static str;
    fn aliases(self) -> &'static [&'static str];
    fn definition(self) -> Result<UnitDefinition, MeasurementError>;

    fn parse_strict(input: &str) -> Result<Self, MeasurementError>;
    fn parse_lenient(input: &str) -> Result<Self, MeasurementError>;
}
```

The default parsing methods must trim input, search canonical symbols first, then aliases. Strict alias hits return `NonCanonicalUnit` with the matched unit's canonical symbol.

Define the optional bridge:

```rust
pub trait UomUnit: Unit {
    type Quantity: Copy;

    fn to_uom_approx(self, value: Decimal) -> Self::Quantity;
    fn value_from_uom_approx(
        self,
        quantity: Self::Quantity,
    ) -> Result<Decimal, MeasurementError>;
}
```

During migration, give `Unit::definition` a documented default that returns `InvalidUnitDefinition { reason: "unit family has not been migrated to Decimal definitions" }`. Task 7 removes this default after the last built-in migration.

- [ ] **Step 4: Implement the public macro and adapt the legacy macro**

Support this final macro grammar:

```rust
define_unit_family! {
    pub enum Name for "quantity_id" $(, uom = UomQuantity)? {
        Variant => {
            symbol: "canonical";
            coefficient: numerator $(/ denominator)?;
            $(offset: decimal_offset;)?
            $(aliases: ["alias", ...];)?
            $(uom: uom_unit_type;)?
        }
    }
}
```

Generate the enum, `Unit`, `Display`, `FromStr`, Serialize, and Deserialize. If `uom = ...` and per-variant `uom` mappings are present, also generate `UomUnit`. Use `$crate`-qualified hidden re-exports for `serde`, `rust_decimal`, and `uom` so downstream users do not need implementation dependencies in scope.

Modify the old `define_measurement_unit!` to generate `UomUnit` rather than putting uom methods on `Unit`; it may rely on the temporary default definition until each family migrates.

- [ ] **Step 5: Run parsing, external-extension, and existing unit tests**

Run:

```bash
cargo test --test mod
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
```

Expected: tests pass; existing built-ins still compile through the transitional macro; rustdoc has no missing-doc warnings.

- [ ] **Step 6: Review checkpoint**

Verify the public macro expands from an integration test without private-module access. Confirm `Unit` has no associated uom quantity and no `f64` methods.

---

### Task 4: Migrate Foundational Mechanical Families to Decimal Definitions

**Files:**
- Modify: `src/measure/units/{length,area,mass,temperature,temperature_interval,velocity,acceleration,force,pressure,frequency,angle,angular_velocity,torque,dynamic_viscosity,kinematic_viscosity,mass_rate}.rs`
- Create: `tests/measure/unit_definition_tests.rs`
- Modify: `tests/measure/mod.rs`

**Interfaces:**
- Consumes: final `define_unit_family!` grammar from Task 3.
- Produces: valid Decimal definitions for every variant in the listed 16 families and the reusable golden-case assertion helper.

- [ ] **Step 1: Add failing independent golden-definition tests**

Create a helper that compares `Unit::definition()` with independently parsed expected values and asserts exact coverage:

```rust
struct DefinitionCase<U> {
    unit: U,
    numerator: &'static str,
    denominator: &'static str,
    offset: &'static str,
}

fn assert_definition_cases<U>(cases: &[DefinitionCase<U>])
where
    U: Unit + Debug,
{
    assert_eq!(cases.len(), U::all().len());
    for case in cases {
        let definition = case.unit.definition().expect("definition should be valid");
        assert_eq!(definition.factor().numerator(), Decimal::from_str(case.numerator).unwrap());
        assert_eq!(definition.factor().denominator(), Decimal::from_str(case.denominator).unwrap());
        assert_eq!(definition.offset(), Decimal::from_str(case.offset).unwrap());
    }
}
```

Populate one case for every existing variant in the listed files. At minimum, independently assert these exact definitions:

```rust
Length::Meter       => 1 / 1, offset 0
Length::Centimeter  => 1 / 100, offset 0
Length::Inch        => 127 / 5000, offset 0
Length::Foot        => 381 / 1250, offset 0
Length::Mile        => 201168 / 125, offset 0
Temperature::Kelvin     => 1 / 1, offset 0
Temperature::Celsius    => 1 / 1, offset 273.15
Temperature::Fahrenheit => 5 / 9, offset 459.67
```

For exact SI prefixes and exact customary definitions, express integer ratios. For conventional finite-precision constants such as angle units, copy the published decimal text into the Decimal numerator without evaluating it as `f64`.

- [ ] **Step 2: Run the definition tests and verify migration errors**

Run:

```bash
cargo test --test mod measure::unit_definition_tests -- --nocapture
```

Expected: failures report the transitional `InvalidUnitDefinition` for unmigrated families.

- [ ] **Step 3: Convert each listed family to `define_unit_family!`**

For every variant, retain its canonical symbol, aliases, and uom marker while adding an exact factor and offset. Use this concrete pattern:

```rust
define_unit_family! {
    pub enum Length for "length", uom = UomLength {
        Meter => {
            symbol: "m";
            coefficient: 1;
            uom: meter;
        }
        Foot => {
            symbol: "ft";
            coefficient: 381 / 1250;
            uom: foot;
        }
    }
}
```

Do not leave any listed variant on the legacy macro. Preserve all currently accepted unambiguous aliases.

- [ ] **Step 4: Run the group tests**

Run:

```bash
cargo test --test mod measure::unit_definition_tests
cargo test --test mod measure::units_tests
```

Expected: all foundational golden cases and unit symbol tests pass.

- [ ] **Step 5: Review checkpoint**

Check that the golden case count equals `Unit::all().len()` for every migrated family and that no production definition calls uom to discover a coefficient.

---

### Task 5: Migrate Electrical and Electromagnetic Families

**Files:**
- Modify: `src/measure/units/{capacitance,electric_charge,electric_current,electric_current_density,electric_field,electric_potential,electrical_conductance,electrical_conductivity,electrical_resistance,electrical_resistivity,inductance,magnetic_field_strength,magnetic_flux,magnetic_flux_density}.rs`
- Modify: `tests/measure/unit_definition_tests.rs`

**Interfaces:**
- Consumes: macro and golden helper from Tasks 3–4.
- Produces: complete Decimal definitions for all variants in the listed 14 families.

- [ ] **Step 1: Add failing golden cases for every listed variant**

Append one `DefinitionCase` per `Unit::all()` variant. Express SI prefix conversions as exact powers of ten. Add explicit representative assertions:

```rust
ElectricCurrent::Ampere       => 1 / 1
ElectricCurrent::Milliampere  => 1 / 1000
ElectricCharge::Coulomb       => 1 / 1
ElectricPotential::Millivolt  => 1 / 1000
ElectricalResistance::Kiloohm => 1000 / 1
Capacitance::Microfarad       => 1 / 1_000_000
MagneticFluxDensity::Gauss    => 1 / 10_000
```

- [ ] **Step 2: Verify the tests fail for unmigrated definitions**

Run:

```bash
cargo test --test mod measure::unit_definition_tests -- --nocapture
```

Expected: only newly added electrical/electromagnetic cases fail with `InvalidUnitDefinition`.

- [ ] **Step 3: Migrate every listed family to `define_unit_family!`**

Use exact ratios for SI prefixes and exact decimal literals for conventional units. Preserve the existing `uom` marker on every variant so `UomUnit` remains available.

- [ ] **Step 4: Run group and bridge tests**

Run:

```bash
cargo test --test mod measure::unit_definition_tests
cargo test --test mod measure::units_tests
```

Expected: all migrated definitions and existing electrical unit behavior pass.

- [ ] **Step 5: Review checkpoint**

Compare the migrated file list to the task header and confirm all 14 families have exact case-count coverage.

---

### Task 6: Migrate Thermal, Chemical, Optical, and Nuclear Families

**Files:**
- Modify: `src/measure/units/{amount_of_substance,catalytic_activity,catalytic_activity_concentration,heat_flux_density,illuminance,luminance,luminous_intensity,mass_concentration,molality,molar_concentration,molar_mass,molar_volume,radioactivity,solid_angle,specific_radioactivity,surface_tension,thermal_conductivity,thermal_resistance}.rs`
- Modify: `tests/measure/unit_definition_tests.rs`

**Interfaces:**
- Consumes: macro and golden helper from Tasks 3–4.
- Produces: complete Decimal definitions for all variants in the listed 18 families.

- [ ] **Step 1: Add failing golden cases for every listed variant**

Append exact cases and assert representative definitions:

```rust
AmountOfSubstance::Mole          => 1 / 1
AmountOfSubstance::Millimole     => 1 / 1000
Radioactivity::Becquerel         => 1 / 1
Radioactivity::Curie             => 37_000_000_000 / 1
Illuminance::Kilolux             => 1000 / 1
MolarConcentration::MolePerLiter => 1000 / 1
```

For constants that are not exact SI definitions, retain the authoritative published decimal digits as a Decimal literal and cite the source in the test comment.

- [ ] **Step 2: Verify focused failure**

Run:

```bash
cargo test --test mod measure::unit_definition_tests -- --nocapture
```

Expected: newly added family cases fail until their files migrate.

- [ ] **Step 3: Migrate all 18 family files**

Replace the legacy macro with `define_unit_family!`, add exact factors and offsets, preserve canonical symbols and unambiguous aliases, and retain all uom markers.

- [ ] **Step 4: Run group tests**

Run:

```bash
cargo test --test mod measure::unit_definition_tests
cargo test --test mod measure::units_tests
```

Expected: all thermal, chemical, optical, and nuclear cases pass.

- [ ] **Step 5: Review checkpoint**

Confirm all listed files migrated and every `Unit::all()` variant has exactly one independent golden case.

---

### Task 7: Make Ambiguous Units Explicit and Finish Built-in Migration

**Files:**
- Modify: `src/measure/units/{time,energy,power,volume,volume_rate,mass_density,heat_capacity,specific_heat_capacity}.rs`
- Modify: `src/measure/units.rs`
- Modify: `src/measure/unit.rs`
- Modify: `tests/measure/unit_definition_tests.rs`
- Modify: `tests/measure/units_tests.rs`
- Modify: `tests/measure/unit_tests.rs`

**Interfaces:**
- Consumes: strict/lenient parsing and exact macro grammar.
- Produces: final explicit unit names, canonical symbols, documented lenient aliases, and no remaining legacy macro/default definition.

- [ ] **Step 1: Add failing rename, canonicalization, and golden tests**

Test the agreed mappings:

```rust
assert_eq!(Time::parse_lenient("year").unwrap(), Time::CommonYear365);
assert_eq!(Time::CommonYear365.symbol(), "a (365 d)");
assert!(matches!(
    Time::parse_strict("year"),
    Err(MeasurementError::NonCanonicalUnit { .. }),
));

assert_eq!(
    Energy::parse_lenient("Btu").unwrap(),
    Energy::BritishThermalUnitInternationalTable,
);
assert_eq!(
    Volume::parse_lenient("gal").unwrap(),
    Volume::UsLiquidGallon,
);
assert_eq!(
    Power::parse_lenient("hp").unwrap(),
    Power::MechanicalHorsepower,
);
```

Add golden factors for every variant in all eight files. Include the exact fixed-year value `31_536_000` seconds, Btu (IT) `1055.056` joules, thermochemical calorie `4.184` joules, exact US liquid gallon `0.003785411784` cubic meters, and mechanical horsepower derived from `550 ft·lbf/s` without using uom's rounded `f64` coefficient.

- [ ] **Step 2: Run focused tests and verify old variants fail expectations**

Run:

```bash
cargo test --test mod measure::unit_definition_tests -- --nocapture
cargo test --test mod measure::unit_tests -- --nocapture
cargo test --test mod measure::units_tests -- --nocapture
```

Expected: compile/test failures reference old ambiguous variant names and symbols.

- [ ] **Step 3: Rename and define every affected unit**

Apply the complete semantic mapping from the design spec, including:

```text
Year -> CommonYear365
Calorie -> ThermochemicalCalorie
Kilocalorie -> ThermochemicalKilocalorie
BritishThermalUnit -> BritishThermalUnitInternationalTable
Horsepower -> MechanicalHorsepower
FluidOunce -> UsFluidOunce
Cup -> UsCustomaryCup
PintLiquid -> UsLiquidPint
QuartLiquid -> UsLiquidQuart
Gallon -> UsLiquidGallon
GallonPerMinute -> UsGallonPerMinute
PoundPerGallon -> PoundPerUsGallon
```

Rename calorie/Btu variants in heat capacity and specific heat capacity to equally explicit thermochemical/IT names. Canonical symbols carry qualifiers; the old short strings remain only in `aliases`.

- [ ] **Step 4: Remove all migration scaffolding**

Run:

```bash
rg -n "define_measurement_unit|has not been migrated" src
```

Expected before removal: matches in the legacy macro/default implementation only. Delete the legacy macro arm and make `Unit::definition` required with no default. Re-run the command; expected: no matches.

- [ ] **Step 5: Run all unit definition and parsing tests**

Run:

```bash
cargo test --test mod
```

Expected: all 56 families and all 296 current variants have one valid definition and one golden case; strict/lenient parsing tests pass.

- [ ] **Step 6: Review checkpoint**

Confirm no ambiguous old enum variant remains, every old short symbol is either an intentional documented alias or removed, and no built-in family relies on the transitional definition error.

---

### Task 8: Route Measurement Through Decimal and Add Quantity-aware Serde

**Files:**
- Modify: `src/measure/measurement.rs`
- Modify: `tests/measure/measurement_tests.rs`
- Modify: `tests/measure/measurement_error_tests.rs`
- Modify: `examples/basic_usage.rs`

**Interfaces:**
- Consumes: `UnitDefinition::convert_value_to`, `ConversionOptions`, exact `Unit`, optional `UomUnit`.
- Produces: exact `convert_to`, `convert_to_with_options`, `parse_strict`, custom three-field Serde, `to_uom_approx`, and `from_uom_approx`.

- [ ] **Step 1: Replace old tests with failing 0.3 contract tests**

Add exact conversion and JSON tests:

```rust
#[test]
fn test_length_conversion_uses_decimal_without_f64_loss() {
    let source = measurement::Length::new(
        Decimal::from_str("12345678901234567890.12345678").unwrap(),
        unit::Length::Centimeter,
    );
    let options = ConversionOptions::maximum_precision(
        RoundingStrategy::MidpointNearestEven,
    );

    let converted = source
        .convert_to_with_options(unit::Length::Meter, options)
        .unwrap();
    assert_eq!(converted.value, Decimal::from_str("123456789012345678.9012345678").unwrap());
}

#[test]
fn test_measurement_json_contains_and_validates_quantity() {
    let value = measurement::Length::new(dec!(50.0), unit::Length::Centimeter);
    assert_eq!(
        serde_json::to_value(value).unwrap(),
        json!({"quantity": "length", "value": "50.0", "unit": "cm"}),
    );

    let error = serde_json::from_value::<measurement::Length>(
        json!({"quantity": "mass", "value": "50.0", "unit": "cm"}),
    )
    .unwrap_err();
    assert!(error.to_string().contains("expected length"));
}
```

Add tests that `convert_to` reads the process default once, explicit options override it, same-unit explicit scale is applied, unit aliases deserialize leniently and serialize canonically, and `Measurement::parse_strict` rejects aliases.

- [ ] **Step 2: Run measurement tests and verify old API failures**

Run:

```bash
cargo test --test mod measure::measurement_tests -- --nocapture
```

Expected: failures because JSON lacks quantity, `convert_to_with_options` is absent, and old uom methods still have non-`approx` names.

- [ ] **Step 3: Implement exact conversion methods**

Implement:

```rust
pub fn convert_to(self, target: U) -> Result<Self, MeasurementError> {
    self.convert_to_with_options(target, default_conversion_options())
}

pub fn convert_to_with_options(
    self,
    target: U,
    options: ConversionOptions,
) -> Result<Self, MeasurementError> {
    let source = self.unit.definition()?;
    let target_definition = target.definition()?;
    let value = source.convert_value_to(
        self.value,
        target_definition,
        options,
    )?;
    Ok(Self::new(value, target))
}
```

Add `parse_strict` using the existing measurement-part splitter and `U::parse_strict`; keep `FromStr` lenient via `U::parse_lenient`.

- [ ] **Step 4: Implement custom three-field Serde**

Remove derived Serialize/Deserialize from `Measurement`. Serialize fields in this order through `SerializeStruct`: `quantity`, `value` as `self.value.to_string()`, and canonical `unit` serialization.

Deserialize through a private helper:

```rust
#[derive(Deserialize)]
struct MeasurementWire<U> {
    quantity: String,
    #[serde(with = "rust_decimal::serde::str")]
    value: Decimal,
    unit: U,
}
```

Require all fields, let Serde reject duplicates, ignore extra fields, and return `QuantityMismatch` through `serde::de::Error::custom` when the quantity differs from `U::QUANTITY`.

- [ ] **Step 5: Move approximate methods behind UomUnit**

Add a separate `impl<U> Measurement<U> where U: UomUnit` containing:

```rust
pub fn to_uom_approx(self) -> U::Quantity;
pub fn from_uom_approx(
    quantity: U::Quantity,
    unit: U,
) -> Result<Self, MeasurementError>;
```

Delete public `to_uom` and `from_uom`. Update focused tests to use the new names and explicit floating tolerances.

- [ ] **Step 6: Run measurement and adapter tests**

Run:

```bash
cargo test --test mod measure::measurement_tests
cargo test --test mod measure::measurement_error_tests
```

Expected: exact Decimal, JSON, parsing, global/default, and approximate bridge tests pass.

- [ ] **Step 7: Review checkpoint**

Run:

```bash
rg -n "to_f64|from_f64|to_uom\(|from_uom\(" src/measure/measurement.rs
```

Expected: no matches. Approximate conversion must be delegated only through `UomUnit` methods whose names include `_approx`.

---

### Task 9: Complete Concurrency, Coverage, Documentation, and 0.3 Release Metadata

**Files:**
- Modify: `tests/measure/conversion_options_tests.rs`
- Modify: `tests/measure/decimal_conversion_tests.rs`
- Modify: `tests/measure/external_unit_tests.rs`
- Modify: `tests/measure/measurement_tests.rs`
- Modify: `tests/measure/unit_definition_tests.rs`
- Modify: `tests/measure/unit_tests.rs`
- Modify: `tests/measure/units_tests.rs`
- Modify: `README.md`
- Modify: `README.zh_CN.md`
- Modify: `examples/basic_usage.rs`
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`
- Modify: `docs/superpowers/specs/2026-07-15-rs-measure-decimal-conversion-design.md` only if implementation reveals a confirmed correction

**Interfaces:**
- Consumes: all final 0.3 APIs.
- Produces: documented, packaged, fully verified 0.3.0 release candidate.

- [ ] **Step 1: Add the final concurrency and coverage tests**

Use explicit options in every non-global precision test. Add one controlled global test that restores the original setting and spawns readers/writers; each observed value must equal one of the complete configurations, never a mixed pair of scale and rounding mode.

Assert for every family:

```rust
assert_eq!(golden_cases.len(), U::all().len());
```

Assert all lenient ambiguous aliases normalize to the canonical symbol on Display and JSON serialization. Keep approximate uom tests in clearly named test functions using tolerance, never exact Decimal equality.

- [ ] **Step 2: Run the complete test suite before documentation edits**

Run:

```bash
cargo test --all-features
```

Expected: all unit, integration, and doctests pass.

- [ ] **Step 3: Update version, example, and both READMEs**

Set:

```toml
version = "0.3.0"
```

Use `qubit-measure = "0.3"` in README installation snippets. Give English and Chinese READMEs identical section structure covering:

1. three-field JSON;
2. Decimal exact/finite-precision boundary;
3. maximum-precision and fixed-scale options;
4. process default mutation and restoration;
5. strict versus lenient parsing;
6. complete ambiguous alias table;
7. external `define_unit_family!` example;
8. `_approx` uom bridge warning;
9. 0.2-to-0.3 migration table.

Update `examples/basic_usage.rs` to import `Decimal`, `ConversionOptions`, and `RoundingStrategy` from the crate root and to show the new JSON quantity field and exact conversion API.

- [ ] **Step 4: Run README, example, and package checks**

Run:

```bash
cargo test --doc
cargo run --example basic_usage
python3 .rs-ci/readme-version-check.py
.rs-ci/cargo-package-check.sh
```

Expected: doctests and example pass, README versions match Cargo.toml, and the package contains both READMEs, source, examples, tests, and the expected metadata.

- [ ] **Step 5: Run formatting and the complete project CI gate**

Run:

```bash
./align-ci.sh
./ci-check.sh
```

Expected: format, Clippy with `-D warnings`, project style, debug/release builds, all-feature tests, rustdoc with warnings denied, README checks, feature matrix, package verification, coverage thresholds, and security audit all pass.

- [ ] **Step 6: Final review checkpoint**

Inspect `git status --short` and `git diff --stat` without staging anything. Confirm only rs-measure implementation, tests, docs, dependency lockfile, and the two superpowers documents changed. Report verification commands and results to the user; do not commit unless explicitly requested.

## Plan Self-Review

- Spec coverage: all 15 design sections map to Tasks 1–9.
- Decimal conversion: Tasks 1–2 create options, factors, checked arithmetic, and exact scale behavior; Task 8 makes it the only `convert_to` path.
- Unit coverage: Tasks 4–7 partition all 56 existing family files; count-based golden tests prevent omitted variants.
- Public contract: Task 3 covers open extension and strict/lenient parsing; Task 8 covers JSON and approximate API names.
- Global state: Tasks 1 and 9 cover `parking_lot::Mutex`, replacement, restoration, snapshots, and concurrency.
- Semantic ambiguity: Task 7 implements every agreed rename and alias mapping.
- Documentation/release: Task 9 covers both READMEs, example, version 0.3.0, package, and full CI.
- Type consistency: `ConversionOptions`, `ConversionFactor`, `UnitDefinition`, `Unit`, `UomUnit`, and `Measurement` method names are consistent across tasks.
- Placeholder scan: no unresolved markers remain; every task names exact files, commands, expected outcomes, and required API shapes.
