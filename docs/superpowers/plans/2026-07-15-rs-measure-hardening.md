# rs-measure Hardening Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Do not dispatch subagents unless the user explicitly authorizes delegation.

**Goal:** Fix the confirmed conversion defects, make the uom bridge opt-in, validate unit-family metadata, document the public contracts, and bring the rs-measure crate into the required Rust organization and style.

**Architecture:** Preserve Measurement, Unit, unit enums, and public module paths. Normalize and cross-cancel Decimal ratios before dangerous arithmetic, isolate all uom code behind one default-off Cargo feature, combine macro-time metadata checks with a public runtime test assertion for manual Unit implementations, and use re-export-only aggregators for newly split helper types and measurement aliases.

**Tech Stack:** Rust 2024, rust-version 1.94, rust_decimal 1.39+, Serde, thiserror, optional uom 0.38, repository rs-ci scripts.

## Global Constraints

- Follow docs/superpowers/specs/2026-07-15-rs-measure-hardening-design.md.
- Follow the repository Rust coding, comment, test, bug-fix, shell, and Git rules.
- Use TDD for every behavior change: write the focused test, run it and observe the expected failure, then change production code.
- Keep the uom feature disabled by default.
- Preserve all existing Decimal core public paths.
- Preserve qubit_measure::measurement::* and qubit_measure::unit::* paths through re-exports.
- Do not add a runtime registry or seal Unit.
- Do not modify .rs-ci shared sources.
- Use apply_patch for authored source and documentation edits. A mechanically generated patch is acceptable for the approved alias and test splits, but shell redirection must not write repository files.
- Do not run git add, git commit, git push, or destructive Git commands.
- Run final validation in this order: ./align-ci.sh, ./ci-check.sh, then ./coverage.sh json only if CI reports coverage below threshold.

---

## File Structure

### New source files

- src/measure/internal.rs: private aggregation and restricted re-export only.
- src/measure/internal/measurement_wire.rs: the private MeasurementWire struct.
- src/measurement/*.rs: one public persisted-measurement alias per snake-case file.

### New test files

- tests/measure/conversion_factor_tests.rs: factor normalization behavior.
- tests/measure/uom_unit_tests.rs: all feature-gated uom bridge tests and BTU regression oracles.
- tests/measure/support.rs: test-support aggregation only.
- tests/measure/support/definition_case.rs: DefinitionCase test type.
- tests/measure/support/definition_assertions.rs: golden-definition assertion function.
- tests/measure/support/manual_validation_unit.rs: one const-generic manual Unit fixture.
- tests/measure/support/unit_assertions.rs: common unit parsing and Serde assertion.
- tests/measure/units/mod.rs and tests/measure/units/*_tests.rs: one family-level definition and invariant test module per source family.

### New configuration

- .rs-ci-cargo-matrix.json: explicit default-core and uom feature checks.

### Principal modified files

- Cargo.toml and Cargo.lock.
- src/lib.rs, src/private.rs, src/measurement.rs, src/unit.rs.
- src/measure/mod.rs, conversion_factor.rs, decimal_conversion.rs, measurement.rs, unit.rs, unit_definition.rs, conversion_options.rs, units.rs, uom_unit.rs.
- Every file under src/measure/units for feature-gated uom imports.
- tests/measure/mod.rs and existing core test modules.
- README.md and README.zh_CN.md.

---

### Task 1: Reproduce and fix the three International Table BTU mappings

**Files:**

- Create: tests/measure/uom_unit_tests.rs
- Modify: tests/measure/mod.rs
- Modify: src/measure/units/energy.rs
- Modify: src/measure/units/heat_capacity.rs
- Modify: src/measure/units/specific_heat_capacity.rs

**Interfaces:**

- Consumes the existing always-enabled UomUnit API for the red phase.
- Produces no new public API.
- Later Task 4 feature-gates this test module without changing its assertions.

- [ ] **Step 1: Add the focused SI-oracle tests**

Add mod uom_unit_tests; to tests/measure/mod.rs. Create the test file with the repository copyright header and this content:

    use qubit_measure::{
        Decimal,
        measurement,
        unit,
    };
    use uom::si::energy::joule;
    use uom::si::heat_capacity::joule_per_kelvin;
    use uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;

    /// Checks two floating-point values with a relative tolerance.
    ///
    /// # Arguments
    ///
    /// * actual - The bridge result.
    /// * expected - The SI oracle value.
    ///
    /// # Panics
    ///
    /// Panics when the values differ by more than one part in 10^12.
    fn assert_approx_eq(actual: f64, expected: f64) {
        let tolerance = expected.abs().max(1.0) * 1.0E-12;
        assert!(
            (actual - expected).abs() <= tolerance,
            "expected {actual} to approximately equal {expected}",
        );
    }

    #[test]
    fn test_energy_btu_it_uom_mapping_uses_si_oracle() {
        let measurement = measurement::Energy::new(
            Decimal::ONE,
            unit::Energy::BritishThermalUnitInternationalTable,
        );

        assert_approx_eq(measurement.to_uom_approx().get::<joule>(), 1_055.056);
    }

    #[test]
    fn test_heat_capacity_btu_it_uom_mapping_uses_si_oracle() {
        let measurement = measurement::HeatCapacity::new(
            Decimal::ONE,
            unit::HeatCapacity::
                BritishThermalUnitInternationalTablePerDegreeFahrenheit,
        );

        assert_approx_eq(
            measurement.to_uom_approx().get::<joule_per_kelvin>(),
            1_899.100_8,
        );
    }

    #[test]
    fn test_specific_heat_capacity_btu_it_uom_mapping_uses_si_oracle() {
        let measurement = measurement::SpecificHeatCapacity::new(
            Decimal::ONE,
            unit::SpecificHeatCapacity::
                BritishThermalUnitInternationalTablePerPoundDegreeFahrenheit,
        );

        assert_approx_eq(
            measurement
                .to_uom_approx()
                .get::<joule_per_kilogram_kelvin>(),
            4_186.800_307_941_667,
        );
    }

- [ ] **Step 2: Run the three tests and verify RED**

Run:

    cargo test --test mod btu_it_uom_mapping_uses_si_oracle -- --nocapture

Expected: all three tests fail with numeric assertion mismatches. They must compile and execute; an import or setup error is not an acceptable red result.

- [ ] **Step 3: Apply the minimal mapping corrections**

In energy.rs replace the imported and mapped unit:

    use uom::si::energy::{
        btu_it,
        calorie,
        electronvolt,
        joule,
        kilocalorie,
        kilojoule,
        kilowatt_hour,
        megajoule,
        watt_hour,
    };

    BritishThermalUnitInternationalTable => { symbol: "Btu (IT)"; definition: crate::consts::energy::BRITISH_THERMAL_UNIT_INTERNATIONAL_TABLE; aliases: ["Btu", "BTU"]; uom: btu_it; }

In heat_capacity.rs use:

    use uom::si::heat_capacity::{
        btu_it_per_degree_fahrenheit,
        calorie_per_kelvin,
        joule_per_degree_celsius,
        joule_per_kelvin,
        kilojoule_per_kelvin,
    };

    BritishThermalUnitInternationalTablePerDegreeFahrenheit => { symbol: "Btu (IT)/°F"; definition: crate::consts::heat_capacity::BRITISH_THERMAL_UNIT_INTERNATIONAL_TABLE_PER_DEGREE_FAHRENHEIT; aliases: ["Btu/°F", "Btu/degF"]; uom: btu_it_per_degree_fahrenheit; }

In specific_heat_capacity.rs use:

    use uom::si::specific_heat_capacity::{
        btu_it_per_pound_degree_fahrenheit,
        calorie_per_gram_kelvin,
        joule_per_gram_degree_celsius,
        joule_per_kilogram_kelvin,
        kilojoule_per_kilogram_kelvin,
    };

    BritishThermalUnitInternationalTablePerPoundDegreeFahrenheit => { symbol: "Btu (IT)/(lb · °F)"; definition: crate::consts::specific_heat_capacity::BRITISH_THERMAL_UNIT_INTERNATIONAL_TABLE_PER_POUND_DEGREE_FAHRENHEIT; aliases: ["Btu/(lb · °F)", "Btu/(lb*degF)"]; uom: btu_it_per_pound_degree_fahrenheit; }

Do not modify the three variants' symbols, Decimal definitions, aliases, or enum names.

- [ ] **Step 4: Run the focused tests and verify GREEN**

Run:

    cargo test --test mod btu_it_uom_mapping_uses_si_oracle -- --nocapture

Expected: 3 passed, 0 failed.

- [ ] **Step 5: Run the existing bridge tests**

Run:

    cargo test --test mod uom_approx -- --nocapture
    cargo test --test mod all_supported_unit_variants_bridge_through_uom -- --nocapture

Expected: every selected existing bridge test passes.

- [ ] **Step 6: Review checkpoint**

Inspect the diff. It must contain exactly three new SI-oracle tests, test registration, and three uom type substitutions.

---

### Task 2: Normalize ConversionFactor and cross-cancel combined ratios

**Files:**

- Create: tests/measure/conversion_factor_tests.rs
- Modify: tests/measure/mod.rs
- Modify: tests/measure/decimal_conversion_tests.rs
- Modify: src/measure/conversion_factor.rs
- Modify: src/measure/decimal_conversion.rs

**Interfaces:**

- ConversionFactor::new keeps its signature but now removes common mantissa
  factors and common scale from its stored representation.
- Adds the crate-private reduce_ratio_terms function for the conversion engine.
- No error variants or public paths change.

- [ ] **Step 1: Add factor-normalization tests**

Register mod conversion_factor_tests; and create:

    use qubit_measure::{
        ConversionFactor,
        Decimal,
    };
    use rust_decimal::dec;

    #[test]
    fn test_conversion_factor_new_reduces_integer_terms() {
        let reduced = ConversionFactor::new(dec!(4), dec!(6))
            .expect("factor should be valid");
        let expected = ConversionFactor::new(dec!(2), dec!(3))
            .expect("factor should be valid");

        assert_eq!(reduced, expected);
        assert_eq!(reduced.numerator(), dec!(2));
        assert_eq!(reduced.denominator(), dec!(3));
    }

    #[test]
    fn test_conversion_factor_new_cancels_common_decimal_scale() {
        let factor = ConversionFactor::new(dec!(0.4), dec!(0.1))
            .expect("factor should be valid");

        assert_eq!(factor.numerator(), dec!(4));
        assert_eq!(factor.denominator(), Decimal::ONE);
    }

- [ ] **Step 2: Add the two conversion regressions**

Append to decimal_conversion_tests.rs:

    #[test]
    fn test_decimal_conversion_reduces_two_over_two_before_max_arithmetic() {
        let identity = UnitDefinition::new(
            ConversionFactor::new(dec!(2), dec!(2))
                .expect("factor should be valid"),
            Decimal::ZERO,
        );

        assert_eq!(
            identity.convert_value_to(
                Decimal::MAX,
                UnitDefinition::base(),
                ConversionOptions::default(),
            ),
            Ok(Decimal::MAX),
        );
    }

    #[test]
    fn test_decimal_conversion_cross_cancels_equal_large_factors() {
        let factor = ConversionFactor::new(Decimal::MAX, dec!(2))
            .expect("factor should be valid");
        let source = UnitDefinition::new(factor, Decimal::ZERO);
        let target = UnitDefinition::new(factor, Decimal::ONE);

        assert_eq!(
            source.convert_value_to(
                Decimal::MAX,
                target,
                ConversionOptions::default(),
            ),
            Decimal::MAX
                .checked_sub(Decimal::ONE)
                .ok_or(MeasurementError::ArithmeticOverflow {
                    operation: "subtract expected value",
                }),
        );
    }

Prefer computing expected before assert so the final assertion is Result-to-Result without introducing a synthetic production error:

    let expected = Decimal::MAX
        .checked_sub(Decimal::ONE)
        .expect("MAX minus one should be representable");
    assert_eq!(
        source.convert_value_to(
            Decimal::MAX,
            target,
            ConversionOptions::default(),
        ),
        Ok(expected),
    );

- [ ] **Step 3: Run tests and verify RED**

Run:

    cargo test --test mod conversion_factor_new_ -- --nocapture
    cargo test --test mod decimal_conversion_reduces_two_over_two -- --nocapture
    cargo test --test mod decimal_conversion_cross_cancels_equal_large_factors -- --nocapture

Expected:

- Factor tests fail because 4/6 and 0.4/0.1 remain unreduced.
- The 2/2 MAX test fails with ArithmeticOverflow.
- The equal-large-factor test fails before subtracting the target offset.

- [ ] **Step 4: Implement exact ratio reduction**

Add these private and crate-private functions to conversion_factor.rs, with complete Arguments and Returns Rustdoc:

    /// Reduces a positive Decimal ratio without multiplying either term.
    ///
    /// # Arguments
    ///
    /// * numerator - The positive ratio numerator.
    /// * denominator - The positive ratio denominator.
    ///
    /// # Returns
    ///
    /// An equivalent numerator and denominator with their mantissa GCD and
    /// common scale removed.
    pub(crate) fn reduce_ratio_terms(
        numerator: Decimal,
        denominator: Decimal,
    ) -> (Decimal, Decimal) {
        let numerator_scale = numerator.scale();
        let denominator_scale = denominator.scale();
        let common_scale = numerator_scale.min(denominator_scale);
        let numerator_mantissa = numerator.mantissa();
        let denominator_mantissa = denominator.mantissa();
        let divisor =
            greatest_common_divisor(numerator_mantissa, denominator_mantissa);

        (
            Decimal::from_i128_with_scale(
                numerator_mantissa / divisor,
                numerator_scale - common_scale,
            ),
            Decimal::from_i128_with_scale(
                denominator_mantissa / divisor,
                denominator_scale - common_scale,
            ),
        )
    }

    /// Computes the greatest common divisor of two positive integers.
    ///
    /// # Arguments
    ///
    /// * lhs - The first positive integer.
    /// * rhs - The second positive integer.
    ///
    /// # Returns
    ///
    /// The positive greatest common divisor.
    fn greatest_common_divisor(mut lhs: i128, mut rhs: i128) -> i128 {
        while rhs != 0 {
            let remainder = lhs % rhs;
            lhs = rhs;
            rhs = remainder;
        }
        lhs
    }

After positivity validation in ConversionFactor::new, call reduce_ratio_terms and construct Self from the returned values:

    let (numerator, denominator) =
        reduce_ratio_terms(numerator, denominator);
    Ok(Self {
        numerator,
        denominator,
    })

Update the type Rustdoc to say that construction reduces common mantissa
factors and common scale. State that PartialEq compares the stored reduced
terms and is not a general mathematical-equivalence solver at Decimal's
representation limits.

- [ ] **Step 5: Cross-cancel before combined multiplication**

Import reduce_ratio_terms into decimal_conversion.rs. Replace raw factor multiplication with:

    let (source_numerator, target_numerator) = reduce_ratio_terms(
        source_factor.numerator(),
        target_factor.numerator(),
    );
    let (target_denominator, source_denominator) = reduce_ratio_terms(
        target_factor.denominator(),
        source_factor.denominator(),
    );

    let converted = match (
        source_numerator.checked_mul(target_denominator),
        source_denominator.checked_mul(target_numerator),
    ) {
        (Some(numerator), Some(denominator)) => {
            apply_ratio(adjusted, numerator, denominator)?
        }
        _ => {
            let base = apply_ratio(
                adjusted,
                source_numerator,
                source_denominator,
            )?;
            apply_ratio(base, target_denominator, target_numerator)?
        }
    };

The terms are positive by ConversionFactor construction, so remove the redundant zero guard. Preserve checked offset addition, target offset subtraction, and output-scale handling.

- [ ] **Step 6: Run focused and full conversion tests**

Run:

    cargo test --test mod conversion_factor_tests -- --nocapture
    cargo test --test mod decimal_conversion_tests -- --nocapture

Expected: all selected tests pass.

- [ ] **Step 7: Review checkpoint**

Confirm:

- No f32 or f64 appears in conversion_factor.rs or decimal_conversion.rs.
- No unchecked arithmetic was introduced.
- Existing factor getters now expose reduced terms.
- The only behavior change is equivalent-ratio normalization and safer arithmetic.

---

### Task 3: Add macro-time metadata checks and a manual-Unit test assertion

**Files:**

- Modify: src/private.rs
- Modify: src/measure/unit.rs
- Modify: src/measure/units.rs
- Modify: src/measure/mod.rs
- Modify: src/lib.rs
- Modify: src/unit.rs
- Modify: tests/measure/unit_tests.rs
- Modify: tests/measure/external_unit_tests.rs
- Create: tests/measure/fixtures.rs
- Create: tests/measure/fixtures/custom_length.rs
- Create: tests/measure/fixtures/canonical_priority_unit.rs
- Create: tests/measure/fixtures/manual_unit.rs
- Create: tests/measure/support.rs
- Create: tests/measure/support/manual_validation_unit.rs

**Interfaces:**

- Adds pub fn assert_unit_family_valid<U: Unit>().
- Adds hidden const metadata validation used by define_unit_family.
- Keeps canonical-symbol-first parsing.

- [ ] **Step 1: Protect and split existing external-unit fixtures**

Run the existing external unit tests before moving fixture definitions:

    cargo test --test mod external_unit_tests -- --nocapture

Expected: all existing tests pass.

Create fixtures.rs as an aggregation module declaring custom_length,
canonical_priority_unit, and manual_unit, then re-export CustomLength,
CanonicalPriorityUnit, and ManualUnit with pub(crate).

Move each complete enum definition and its directly related implementations
from external_unit_tests.rs into its named fixture file:

- CustomLength and its define_unit_family invocation to custom_length.rs.
- CanonicalPriorityUnit and its define_unit_family invocation to canonical_priority_unit.rs.
- ManualUnit plus Unit, Display, and FromStr implementations to manual_unit.rs.

Every fixture file receives the standard copyright header, module Rustdoc,
explicit imports, and exactly one enum definition. Register mod fixtures;
before external_unit_tests in tests/measure/mod.rs. Import the three types from
crate::measure::fixtures in external_unit_tests.rs.

Run:

    cargo test --test mod external_unit_tests -- --nocapture

Expected: behavior remains green and the test file no longer defines a type.

- [ ] **Step 2: Add RED tests for the public assertion**

Create support/manual_validation_unit.rs with one type:

    use std::fmt;
    use std::str::FromStr;

    use qubit_measure::{
        MeasurementError,
        Unit,
        UnitDefinition,
    };

    /// Selects valid metadata.
    pub(crate) const VALID: u8 = 0;

    /// Selects a duplicated all entry.
    pub(crate) const DUPLICATE_ALL: u8 = 1;

    /// Selects duplicated canonical symbols.
    pub(crate) const DUPLICATE_SYMBOL: u8 = 2;

    /// Selects duplicated aliases.
    pub(crate) const DUPLICATE_ALIAS: u8 = 3;

    /// Selects an invalid quantity identifier.
    pub(crate) const INVALID_QUANTITY: u8 = 4;

    /// Configurable manual unit family used by validation tests.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) struct ManualValidationUnit<const CASE: u8> {
        /// Index of the represented test unit.
        index: u8,
    }

    impl<const CASE: u8> Unit for ManualValidationUnit<CASE> {
        const QUANTITY: &'static str =
            if CASE == INVALID_QUANTITY { "Invalid__Quantity" } else { "manual" };

        fn all() -> &'static [Self] {
            if CASE == DUPLICATE_ALL {
                &[Self { index: 0 }, Self { index: 0 }]
            } else {
                &[Self { index: 0 }, Self { index: 1 }]
            }
        }

        fn symbol(self) -> &'static str {
            match (CASE, self.index) {
                (DUPLICATE_SYMBOL, _) => "duplicate",
                (_, 0) => "base",
                _ => "derived",
            }
        }

        fn aliases(self) -> &'static [&'static str] {
            match (CASE, self.index) {
                (DUPLICATE_ALIAS, _) => &["duplicate-alias"],
                (_, 0) => &["b"],
                _ => &["d"],
            }
        }

        fn definition(self) -> Result<UnitDefinition, MeasurementError> {
            Ok(UnitDefinition::base())
        }
    }

    impl<const CASE: u8> fmt::Display for ManualValidationUnit<CASE> {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str(self.symbol())
        }
    }

    impl<const CASE: u8> FromStr for ManualValidationUnit<CASE> {
        type Err = MeasurementError;

        fn from_str(input: &str) -> Result<Self, Self::Err> {
            Self::parse_lenient(input)
        }
    }

Create support.rs as an aggregation module and re-export the type and case constants with pub(crate). Register mod support; before test modules in tests/measure/mod.rs.

In unit_tests.rs import assert_unit_family_valid and add:

    #[test]
    fn test_assert_unit_family_valid_accepts_valid_manual_family() {
        assert_unit_family_valid::<ManualValidationUnit<VALID>>();
    }

    #[test]
    #[should_panic(expected = "duplicate all() entry")]
    fn test_assert_unit_family_valid_rejects_duplicate_all_entry() {
        assert_unit_family_valid::<ManualValidationUnit<DUPLICATE_ALL>>();
    }

    #[test]
    #[should_panic(expected = "duplicate canonical symbol")]
    fn test_assert_unit_family_valid_rejects_duplicate_symbol() {
        assert_unit_family_valid::<ManualValidationUnit<DUPLICATE_SYMBOL>>();
    }

    #[test]
    #[should_panic(expected = "duplicate alias")]
    fn test_assert_unit_family_valid_rejects_duplicate_alias() {
        assert_unit_family_valid::<ManualValidationUnit<DUPLICATE_ALIAS>>();
    }

    #[test]
    #[should_panic(expected = "ASCII snake_case")]
    fn test_assert_unit_family_valid_rejects_invalid_quantity() {
        assert_unit_family_valid::<ManualValidationUnit<INVALID_QUANTITY>>();
    }

Also call assert_unit_family_valid for the existing ManualUnit and both macro-generated families in external_unit_tests.rs.

- [ ] **Step 3: Add compile-fail and allowed-collision doctests**

Extend define_unit_family Rustdoc with:

- A runnable example where AliasOwner aliases canonical and CanonicalOwner owns canonical; parsing canonical must return CanonicalOwner.
- A compile_fail example with two variants using symbol x.
- A compile_fail example with two variants using alias duplicate.

Each example imports define_unit_family and Unit, gives every generated enum and variant Rustdoc, and uses coefficient: 1 so no private implementation detail is required.

- [ ] **Step 4: Run tests and verify RED**

Run:

    cargo test --test mod assert_unit_family_valid -- --nocapture
    cargo test --doc define_unit_family -- --nocapture

Expected:

- Integration tests fail to compile because assert_unit_family_valid is absent.
- compile_fail doctests report that the duplicate definitions unexpectedly compiled.

- [ ] **Step 5: Add const metadata validation**

Add these hidden functions to private.rs with complete Rustdoc:

    #[doc(hidden)]
    pub const fn assert_unit_family_metadata(
        quantity: &str,
        symbols: &[&str],
        aliases: &[&str],
    ) {
        assert!(
            is_ascii_snake_case(quantity),
            "unit quantity must be non-empty ASCII snake_case",
        );
        assert!(!symbols.is_empty(), "unit family must not be empty");

        let mut index = 0;
        while index < symbols.len() {
            assert!(
                !symbols[index].as_bytes().is_empty(),
                "canonical unit symbol must not be empty",
            );
            let mut other = index + 1;
            while other < symbols.len() {
                assert!(
                    !str_eq(symbols[index], symbols[other]),
                    "canonical unit symbols must be unique",
                );
                other += 1;
            }
            index += 1;
        }

        index = 0;
        while index < aliases.len() {
            assert!(
                !aliases[index].as_bytes().is_empty(),
                "unit alias must not be empty",
            );
            let mut other = index + 1;
            while other < aliases.len() {
                assert!(
                    !str_eq(aliases[index], aliases[other]),
                    "unit aliases must be unique",
                );
                other += 1;
            }
            index += 1;
        }
    }

Add the public hidden predicate and private equality helper:

    #[doc(hidden)]
    pub const fn is_ascii_snake_case(value: &str) -> bool {
        let bytes = value.as_bytes();
        if bytes.is_empty() || bytes[0] < b'a' || bytes[0] > b'z' {
            return false;
        }

        let mut index = 1;
        let mut previous_underscore = false;
        while index < bytes.len() {
            let byte = bytes[index];
            if byte == b'_' {
                if previous_underscore || index + 1 == bytes.len() {
                    return false;
                }
                previous_underscore = true;
            } else if (byte >= b'a' && byte <= b'z')
                || (byte >= b'0' && byte <= b'9')
            {
                previous_underscore = false;
            } else {
                return false;
            }
            index += 1;
        }
        true
    }

    /// Compares two strings in const contexts.
    const fn str_eq(lhs: &str, rhs: &str) -> bool {
        let lhs = lhs.as_bytes();
        let rhs = rhs.as_bytes();
        if lhs.len() != rhs.len() {
            return false;
        }

        let mut index = 0;
        while index < lhs.len() {
            if lhs[index] != rhs[index] {
                return false;
            }
            index += 1;
        }
        true
    }

- [ ] **Step 6: Invoke the const checker from the core macro**

Inside __define_unit_family_core, before the generated enum, emit:

    const _: () = {
        const SYMBOLS: &[&str] = &[
            $($symbol,)+
        ];
        const ALIASES: &[&str] = &[
            $(
                $($($alias,)*)?
            )+
        ];
        $crate::__private::assert_unit_family_metadata(
            $quantity_name,
            SYMBOLS,
            ALIASES,
        );
    };

Do not compare aliases with symbols: alias-to-canonical overlap is deliberately allowed.

- [ ] **Step 7: Implement assert_unit_family_valid**

Add to unit.rs after the Unit trait:

    #[track_caller]
    pub fn assert_unit_family_valid<U>()
    where
        U: Unit,
    {
        let units = U::all();
        assert!(!units.is_empty(), "unit family must not be empty");
        assert!(
            crate::__private::is_ascii_snake_case(U::QUANTITY),
            "unit quantity must be non-empty ASCII snake_case",
        );

        for (index, unit) in units.iter().copied().enumerate() {
            assert!(
                !units[..index].contains(&unit),
                "duplicate all() entry at index {index}",
            );
            let symbol = unit.symbol();
            assert!(!symbol.is_empty(), "canonical symbol must not be empty");
            assert!(
                !units[..index]
                    .iter()
                    .any(|other| other.symbol() == symbol),
                "duplicate canonical symbol: {symbol}",
            );
            unit.definition().unwrap_or_else(|error| {
                panic!("invalid definition for {symbol}: {error}")
            });
            assert!(
                U::parse_strict(symbol) == Ok(unit),
                "strict canonical parse failed for {symbol}",
            );
            assert!(
                U::parse_lenient(symbol) == Ok(unit),
                "lenient canonical parse failed for {symbol}",
            );
        }

        let mut seen_aliases: Vec<&str> = Vec::new();
        for unit in units.iter().copied() {
            for alias in unit.aliases() {
                assert!(!alias.is_empty(), "unit alias must not be empty");
                assert!(
                    !seen_aliases.contains(alias),
                    "duplicate alias: {alias}",
                );
                seen_aliases.push(alias);

                if let Some(owner) = units
                    .iter()
                    .copied()
                    .find(|candidate| candidate.symbol() == *alias)
                {
                    assert!(
                        U::parse_strict(alias) == Ok(owner),
                        "canonical owner must win strict parsing for {alias}",
                    );
                    assert!(
                        U::parse_lenient(alias) == Ok(owner),
                        "canonical owner must win lenient parsing for {alias}",
                    );
                } else {
                    assert!(
                        matches!(
                            U::parse_strict(alias),
                            Err(MeasurementError::NonCanonicalUnit {
                                canonical,
                                ..
                            }) if canonical == unit.symbol()
                        ),
                        "strict parsing must reject alias {alias}",
                    );
                    assert!(
                        U::parse_lenient(alias) == Ok(unit),
                        "lenient alias parse failed for {alias}",
                    );
                }
            }
        }
    }

Add complete Arguments, Panics, limitations, and Examples Rustdoc. The function has loops and receives no inline attribute.

Re-export it from measure/mod.rs, lib.rs, and unit.rs.

- [ ] **Step 8: Run focused tests and doctests**

Run:

    cargo test --test mod assert_unit_family_valid -- --nocapture
    cargo test --test mod external_unit_tests -- --nocapture
    cargo test --doc define_unit_family -- --nocapture

Expected: all selected tests pass. The duplicate doctest examples pass only because compilation fails.

- [ ] **Step 9: Review checkpoint**

Confirm:

- Canonical symbols are checked only against other canonical symbols.
- Aliases are checked only against aliases.
- Alias-to-canonical overlap remains legal and canonical parsing wins.
- Macro all completeness is structural; manual all completeness is documented as unprovable.

---

### Task 4: Make uom an opt-in Cargo feature

**Files:**

- Create: .rs-ci-cargo-matrix.json
- Modify: Cargo.toml and Cargo.lock
- Modify: src/lib.rs
- Modify: src/private.rs
- Modify: src/unit.rs
- Modify: src/measure/mod.rs
- Modify: src/measure/measurement.rs
- Modify: src/measure/units.rs
- Modify: every Rust file under src/measure/units
- Modify: tests/measure/mod.rs
- Modify: tests/measure/measurement_tests.rs
- Modify: tests/measure/uom_unit_tests.rs

**Interfaces:**

- Default build omits UomUnit and approximate methods.
- features = ["uom"] restores the bridge.
- Exact unit families and Decimal conversion remain available in both modes.

- [ ] **Step 1: Add the feature-matrix RED configuration**

Create .rs-ci-cargo-matrix.json:

    {
      "version": 1,
      "checks": [
        {
          "name": "default-core",
          "commands": ["check", "test", "doc", "doc-test", "clippy"],
          "defaultFeatures": true
        },
        {
          "name": "uom",
          "commands": ["check", "test", "doc", "doc-test", "clippy"],
          "defaultFeatures": false,
          "features": ["uom"]
        }
      ]
    }

Run:

    ./.rs-ci/cargo-feature-check.sh validate
    ./.rs-ci/cargo-feature-check.sh run-all

Expected: validation succeeds; the uom matrix entry fails because Cargo.toml does not yet declare a uom feature.

- [ ] **Step 2: Define the feature and optional dependency**

Update Cargo.toml:

    [features]
    default = []
    uom = ["dep:uom"]

    [dependencies]
    rust_decimal = { version = "1.39", features = ["macros", "serde-with-str"] }
    serde = { version = "1.0", features = ["derive"] }
    thiserror = "2.0"
    uom = {
        version = "0.38",
        optional = true,
        default-features = false,
        features = ["f64", "si", "std"],
    }

Use cargo metadata or cargo check to refresh Cargo.lock; do not hand-edit the lockfile.

- [ ] **Step 3: Gate core modules and exports**

Apply these conditions:

    // src/measure/mod.rs
    #[cfg(feature = "uom")]
    mod uom_unit;
    #[cfg(feature = "uom")]
    pub use uom_unit::UomUnit;

    // src/lib.rs and src/unit.rs
    #[cfg(feature = "uom")]
    pub use measure::UomUnit;

In measurement.rs:

- Remove UomUnit from the unconditional import.
- Add a separate cfg(feature = "uom") import.
- Put cfg(feature = "uom") on the entire impl<U> Measurement<U> where U: UomUnit block.
- Update the type Rustdoc so default documentation does not link to a missing approximate method; use a cfg-specific paragraph or plain text without an unresolved link.

In units.rs and the crate/module-level Rustdoc:

- Replace unconditional intra-doc links to UomUnit or approximate methods with
  feature-safe prose, or put the complete documented item behind the uom cfg.
- State that exact Unit generation is unconditional and approximate bridge
  generation occurs only when the dependency feature is enabled.
- Update lib.rs, measure/mod.rs, measurement.rs, and unit.rs module summaries
  so default rustdoc describes optional rather than always-present adapters.

In private.rs:

- Put cfg(feature = "uom") on pub use uom, FromPrimitive/ToPrimitive imports, and both f64 bridge helpers.
- Keep rust_decimal, serde, and metadata validation unconditional.

- [ ] **Step 4: Make exported macro uom generation feature-owned**

Extract the duplicated UomUnit impl into two cfg-selected hidden macros:

    #[cfg(feature = "uom")]
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __define_uom_unit {
        (
            $unit:ident,
            $quantity_ty:ty,
            {
                $($variant:ident => $uom_unit:ty;)+
            }
        ) => {
            impl $crate::UomUnit for $unit {
                type Quantity = $quantity_ty;

                #[inline(always)]
                fn to_uom_approx(
                    self,
                    value: $crate::Decimal,
                ) -> Self::Quantity {
                    let value =
                        $crate::__private::decimal_to_f64_approx(value);
                    match self {
                        $(
                            Self::$variant =>
                                <$quantity_ty>::new::<$uom_unit>(value),
                        )+
                    }
                }

                #[inline]
                fn value_from_uom_approx(
                    self,
                    quantity: Self::Quantity,
                ) -> Result<$crate::Decimal, $crate::MeasurementError> {
                    let value = match self {
                        $(
                            Self::$variant =>
                                quantity.get::<$uom_unit>(),
                        )+
                    };
                    $crate::__private::decimal_from_f64_approx(value)
                }
            }
        };
    }

    #[cfg(not(feature = "uom"))]
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __define_uom_unit {
        ($($tokens:tt)*) => {};
    }

Both uom-capable define_unit_family arms must generate exact Unit through
__define_unit_family_core and then invoke:

    $crate::__define_uom_unit! {
        $unit,
        $quantity_ty,
        {
            $($variant => $uom_unit;)+
        }
    }

Do not emit cfg(feature = "uom") inside downstream expansion.

- [ ] **Step 5: Gate every built-in uom import**

For every direct uom use block in these files, add cfg(feature = "uom"):

- acceleration.rs
- amount_of_substance.rs
- angle.rs
- angular_velocity.rs
- area.rs
- capacitance.rs
- catalytic_activity.rs
- catalytic_activity_concentration.rs
- dynamic_viscosity.rs
- electric_charge.rs
- electric_current.rs
- electric_current_density.rs
- electric_field.rs
- electric_potential.rs
- electrical_conductance.rs
- electrical_conductivity.rs
- electrical_resistance.rs
- electrical_resistivity.rs
- energy.rs
- force.rs
- frequency.rs
- heat_capacity.rs
- heat_flux_density.rs
- illuminance.rs
- inductance.rs
- kinematic_viscosity.rs
- length.rs
- luminance.rs
- luminous_intensity.rs
- magnetic_field_strength.rs
- magnetic_flux.rs
- magnetic_flux_density.rs
- mass.rs
- mass_concentration.rs
- mass_density.rs
- mass_rate.rs
- molality.rs
- molar_concentration.rs
- molar_mass.rs
- molar_volume.rs
- power.rs
- pressure.rs
- radioactivity.rs
- solid_angle.rs
- specific_heat_capacity.rs
- specific_radioactivity.rs
- surface_tension.rs
- temperature.rs
- temperature_interval.rs
- thermal_conductivity.rs
- thermal_resistance.rs
- time.rs
- torque.rs
- velocity.rs
- volume.rs
- volume_rate.rs

For each listed file, place #[cfg(feature = "uom")] immediately above every
top-level use declaration whose path begins with uom::. Do not gate the
define_unit_family import.

Keep use crate::define_unit_family unconditional so exact unit enums always compile.

- [ ] **Step 6: Move all bridge tests into the gated test module**

Add cfg(feature = "uom") before mod uom_unit_tests in tests/measure/mod.rs.

Move these complete functions, unchanged, from measurement_tests.rs to uom_unit_tests.rs:

- assert_approx_eq
- assert_all_unit_variants_bridge_uom
- test_length_measurement_to_uom_approx_converts_unit
- test_mass_measurement_to_uom_approx_converts_unit
- test_time_measurement_to_uom_approx_converts_unit
- test_area_and_volume_measurements_to_uom_approx_convert_units
- test_new_quantity_families_to_uom_approx_convert_units
- test_electrical_measurements_to_uom_approx_convert_units
- test_all_supported_unit_variants_bridge_through_uom
- test_length_measurement_from_uom_approx_uses_target_unit
- test_mass_measurement_from_uom_approx_uses_target_unit
- test_time_area_and_volume_measurements_from_uom_approx_use_target_unit
- test_new_quantity_families_from_uom_approx_use_target_unit
- test_electrical_measurements_from_uom_approx_use_target_unit
- test_measurement_from_uom_approx_rejects_nan

Move their uom imports and UomUnit import as well. Deduplicate joule and
assert_approx_eq with the three BTU tests already in uom_unit_tests.rs.
measurement_tests.rs must retain only exact Measurement, parsing, Serde, display, and Decimal conversion tests.

- [ ] **Step 7: Run the feature matrix and dependency checks**

Run:

    cargo tree --no-default-features
    ./.rs-ci/cargo-feature-check.sh run-all

Expected:

- cargo tree output contains no uom package.
- default-core checks pass without UomUnit.
- uom checks pass with every bridge test, doctest, rustdoc build, and Clippy.

- [ ] **Step 8: Review checkpoint**

Search:

    rg -n "use uom|UomUnit|to_uom_approx|from_uom_approx" src tests

Every source hit outside units.rs macro token handling must be feature-gated. Exact unit family definitions must remain available with no features.

---

### Task 5: Split private types and persisted measurement aliases

**Files:**

- Create: src/measure/internal.rs
- Create: src/measure/internal/measurement_wire.rs
- Modify: src/measure/mod.rs
- Modify: src/measure/measurement.rs
- Modify: src/measurement.rs
- Create: the alias files listed below under src/measurement

**Interfaces:**

- MeasurementWire remains private.
- All qubit_measure::measurement::* aliases retain identical names and target types.

- [ ] **Step 1: Protect public alias paths**

Add or retain compile-time uses in measurement_tests.rs for:

    let _: measurement::Length =
        Measurement::new(Decimal::ONE, unit::Length::Meter);
    let _: measurement::ElectricPotential =
        Measurement::new(Decimal::ONE, unit::ElectricPotential::Volt);
    let _: measurement::Voltage =
        Measurement::new(Decimal::ONE, unit::ElectricPotential::Volt);

Run:

    cargo test --test mod generic_measurement_type_remains_available -- --nocapture

Expected: pass before the structural change. These are behavior-preservation tests, so the red phase is not applicable; TDD protects an existing public contract before refactoring.

- [ ] **Step 2: Extract MeasurementWire**

Create internal/measurement_wire.rs with the standard header:

    use rust_decimal::Decimal;
    use serde::Deserialize;

    /// String-based persistence representation owned by Measurement.
    #[derive(Deserialize)]
    pub(super) struct MeasurementWire {
        /// Stable quantity identifier used to reject cross-quantity data.
        pub(super) quantity: String,

        /// Exact decimal value encoded as a string.
        #[serde(with = "rust_decimal::serde::str")]
        pub(super) value: Decimal,

        /// Canonical unit symbol or a documented input alias.
        pub(super) unit: String,
    }

Create internal.rs:

    //! Private implementation types for persisted measurements.

    mod measurement_wire;

    pub(super) use measurement_wire::MeasurementWire;

Declare mod internal; in measure/mod.rs. Import MeasurementWire from
crate::measure::internal in measurement.rs and remove the local struct plus its Deserialize derive import if no longer used elsewhere.

- [ ] **Step 3: Create one alias file per manifest row**

Each row below names the exact file to create under src/measurement, the alias
to move from src/measurement.rs, its unit target, and the noun retained in its
Rustdoc:

| File | Alias | Target unit | Rustdoc noun |
| --- | --- | --- | --- |
| acceleration.rs | Acceleration | Acceleration | acceleration |
| amount_of_substance.rs | AmountOfSubstance | AmountOfSubstance | amount of substance |
| angle.rs | Angle | Angle | angle |
| angular_velocity.rs | AngularVelocity | AngularVelocity | angular velocity |
| area.rs | Area | Area | area |
| capacitance.rs | Capacitance | Capacitance | capacitance |
| catalytic_activity.rs | CatalyticActivity | CatalyticActivity | catalytic activity |
| catalytic_activity_concentration.rs | CatalyticActivityConcentration | CatalyticActivityConcentration | catalytic activity concentration |
| dynamic_viscosity.rs | DynamicViscosity | DynamicViscosity | dynamic viscosity |
| electric_charge.rs | ElectricCharge | ElectricCharge | electric charge |
| electric_current.rs | ElectricCurrent | ElectricCurrent | electric current |
| electric_current_density.rs | ElectricCurrentDensity | ElectricCurrentDensity | electric current density |
| electric_field.rs | ElectricField | ElectricField | electric field |
| electric_potential.rs | ElectricPotential | ElectricPotential | electric potential |
| voltage.rs | Voltage | ElectricPotential | voltage |
| electrical_conductance.rs | ElectricalConductance | ElectricalConductance | electrical conductance |
| electrical_conductivity.rs | ElectricalConductivity | ElectricalConductivity | electrical conductivity |
| electrical_resistance.rs | ElectricalResistance | ElectricalResistance | electrical resistance |
| electrical_resistivity.rs | ElectricalResistivity | ElectricalResistivity | electrical resistivity |
| energy.rs | Energy | Energy | energy |
| force.rs | Force | Force | force |
| frequency.rs | Frequency | Frequency | frequency |
| heat_capacity.rs | HeatCapacity | HeatCapacity | heat capacity |
| heat_flux_density.rs | HeatFluxDensity | HeatFluxDensity | heat flux density |
| illuminance.rs | Illuminance | Illuminance | illuminance |
| inductance.rs | Inductance | Inductance | inductance |
| kinematic_viscosity.rs | KinematicViscosity | KinematicViscosity | kinematic viscosity |
| length.rs | Length | Length | length |
| luminance.rs | Luminance | Luminance | luminance |
| luminous_intensity.rs | LuminousIntensity | LuminousIntensity | luminous intensity |
| magnetic_field_strength.rs | MagneticFieldStrength | MagneticFieldStrength | magnetic field strength |
| magnetic_flux.rs | MagneticFlux | MagneticFlux | magnetic flux |
| magnetic_flux_density.rs | MagneticFluxDensity | MagneticFluxDensity | magnetic flux density |
| mass.rs | Mass | Mass | mass |
| mass_concentration.rs | MassConcentration | MassConcentration | mass concentration |
| mass_density.rs | MassDensity | MassDensity | mass density |
| mass_rate.rs | MassRate | MassRate | mass rate |
| molality.rs | Molality | Molality | molality |
| molar_concentration.rs | MolarConcentration | MolarConcentration | molar concentration |
| molar_mass.rs | MolarMass | MolarMass | molar mass |
| molar_volume.rs | MolarVolume | MolarVolume | molar volume |
| power.rs | Power | Power | power |
| pressure.rs | Pressure | Pressure | pressure |
| radioactivity.rs | Radioactivity | Radioactivity | radioactivity |
| solid_angle.rs | SolidAngle | SolidAngle | solid angle |
| specific_heat_capacity.rs | SpecificHeatCapacity | SpecificHeatCapacity | specific heat capacity |
| specific_radioactivity.rs | SpecificRadioactivity | SpecificRadioactivity | specific radioactivity |
| surface_tension.rs | SurfaceTension | SurfaceTension | surface tension |
| temperature.rs | Temperature | Temperature | thermodynamic temperature |
| temperature_interval.rs | TemperatureInterval | TemperatureInterval | temperature interval |
| thermal_conductivity.rs | ThermalConductivity | ThermalConductivity | thermal conductivity |
| thermal_resistance.rs | ThermalResistance | ThermalResistance | thermal resistance |
| time.rs | Time | Time | time |
| torque.rs | Torque | Torque | torque |
| velocity.rs | Velocity | Velocity | velocity |
| volume.rs | Volume | Volume | volume |
| volume_rate.rs | VolumeRate | VolumeRate | volume rate |

For every row except Voltage, move the existing alias Rustdoc and declaration
verbatim from src/measurement.rs into the listed file, then make its two
dependencies explicit:

    use crate::Measurement;
    use crate::unit;

ElectricPotential additionally retains:

    /// This is the SI quantity commonly called voltage.

Voltage uses Measurement<unit::ElectricPotential> and retains:

    /// This is an ergonomic alias for ElectricPotential.

- [ ] **Step 4: Turn measurement.rs into a pure aggregator**

Keep the copyright header and module Rustdoc. Remove direct imports and alias definitions. Add one private mod declaration and one pub use for every manifest row. The order must match the current public documentation order, with voltage immediately after electric_potential.

    mod acceleration;
    mod amount_of_substance;
    mod angle;
    mod angular_velocity;
    mod area;
    mod capacitance;
    mod catalytic_activity;
    mod catalytic_activity_concentration;
    mod dynamic_viscosity;
    mod electric_charge;
    mod electric_current;
    mod electric_current_density;
    mod electric_field;
    mod electric_potential;
    mod voltage;
    mod electrical_conductance;
    mod electrical_conductivity;
    mod electrical_resistance;
    mod electrical_resistivity;
    mod energy;
    mod force;
    mod frequency;
    mod heat_capacity;
    mod heat_flux_density;
    mod illuminance;
    mod inductance;
    mod kinematic_viscosity;
    mod length;
    mod luminance;
    mod luminous_intensity;
    mod magnetic_field_strength;
    mod magnetic_flux;
    mod magnetic_flux_density;
    mod mass;
    mod mass_concentration;
    mod mass_density;
    mod mass_rate;
    mod molality;
    mod molar_concentration;
    mod molar_mass;
    mod molar_volume;
    mod power;
    mod pressure;
    mod radioactivity;
    mod solid_angle;
    mod specific_heat_capacity;
    mod specific_radioactivity;
    mod surface_tension;
    mod temperature;
    mod temperature_interval;
    mod thermal_conductivity;
    mod thermal_resistance;
    mod time;
    mod torque;
    mod velocity;
    mod volume;
    mod volume_rate;

    pub use acceleration::Acceleration;
    pub use amount_of_substance::AmountOfSubstance;
    pub use angle::Angle;
    pub use angular_velocity::AngularVelocity;
    pub use area::Area;
    pub use capacitance::Capacitance;
    pub use catalytic_activity::CatalyticActivity;
    pub use catalytic_activity_concentration::CatalyticActivityConcentration;
    pub use dynamic_viscosity::DynamicViscosity;
    pub use electric_charge::ElectricCharge;
    pub use electric_current::ElectricCurrent;
    pub use electric_current_density::ElectricCurrentDensity;
    pub use electric_field::ElectricField;
    pub use electric_potential::ElectricPotential;
    pub use voltage::Voltage;
    pub use electrical_conductance::ElectricalConductance;
    pub use electrical_conductivity::ElectricalConductivity;
    pub use electrical_resistance::ElectricalResistance;
    pub use electrical_resistivity::ElectricalResistivity;
    pub use energy::Energy;
    pub use force::Force;
    pub use frequency::Frequency;
    pub use heat_capacity::HeatCapacity;
    pub use heat_flux_density::HeatFluxDensity;
    pub use illuminance::Illuminance;
    pub use inductance::Inductance;
    pub use kinematic_viscosity::KinematicViscosity;
    pub use length::Length;
    pub use luminance::Luminance;
    pub use luminous_intensity::LuminousIntensity;
    pub use magnetic_field_strength::MagneticFieldStrength;
    pub use magnetic_flux::MagneticFlux;
    pub use magnetic_flux_density::MagneticFluxDensity;
    pub use mass::Mass;
    pub use mass_concentration::MassConcentration;
    pub use mass_density::MassDensity;
    pub use mass_rate::MassRate;
    pub use molality::Molality;
    pub use molar_concentration::MolarConcentration;
    pub use molar_mass::MolarMass;
    pub use molar_volume::MolarVolume;
    pub use power::Power;
    pub use pressure::Pressure;
    pub use radioactivity::Radioactivity;
    pub use solid_angle::SolidAngle;
    pub use specific_heat_capacity::SpecificHeatCapacity;
    pub use specific_radioactivity::SpecificRadioactivity;
    pub use surface_tension::SurfaceTension;
    pub use temperature::Temperature;
    pub use temperature_interval::TemperatureInterval;
    pub use thermal_conductivity::ThermalConductivity;
    pub use thermal_resistance::ThermalResistance;
    pub use time::Time;
    pub use torque::Torque;
    pub use velocity::Velocity;
    pub use volume::Volume;
    pub use volume_rate::VolumeRate;

No struct, trait, enum, type alias, function, impl, or macro remains in measurement.rs.

- [ ] **Step 5: Verify public paths and file layout**

Run:

    cargo test --test mod measurement_tests -- --nocapture
    ./style-check.sh

Expected: measurement behavior passes and style-check reports no public type layout error.

- [ ] **Step 6: Review checkpoint**

Confirm all 57 aliases exist once, all public names are re-exported, Voltage still targets ElectricPotential, and MeasurementWire is inaccessible publicly.

---

### Task 6: Mirror family tests and split test helper types

**Files:**

- Create: tests/measure/support/definition_case.rs
- Create: tests/measure/support/definition_assertions.rs
- Create: tests/measure/support/unit_assertions.rs
- Modify: tests/measure/support.rs
- Create: tests/measure/units/mod.rs
- Create: tests/measure/units/*_tests.rs for every unit source family
- Modify: tests/measure/mod.rs
- Modify: tests/measure/unit_definition_tests.rs
- Modify: tests/measure/units_tests.rs

**Interfaces:**

- Production API does not change.
- Existing golden values and parsing expectations are moved, not rewritten.

- [ ] **Step 1: Split DefinitionCase into its own file**

Create definition_case.rs:

    /// One independently written exact unit-definition oracle.
    pub(crate) struct DefinitionCase<U> {
        /// Unit being checked.
        pub(crate) unit: U,

        /// Expected canonical factor numerator.
        pub(crate) numerator: &'static str,

        /// Expected canonical factor denominator.
        pub(crate) denominator: &'static str,

        /// Expected pre-factor offset.
        pub(crate) offset: &'static str,
    }

Move assert_definition_cases unchanged into definition_assertions.rs, add complete Arguments and Panics Rustdoc, and import DefinitionCase explicitly. Move
assert_unit_symbols_parse_display_and_serde_round_trip into unit_assertions.rs, rename it assert_unit_contract, and add complete docs.

Re-export the type and functions from support.rs with pub(crate).

- [ ] **Step 2: Register the family test tree**

Create tests/measure/units/mod.rs with the standard header and one module declaration per source family:

    mod acceleration_tests;
    mod amount_of_substance_tests;
    mod angle_tests;
    mod angular_velocity_tests;
    mod area_tests;
    mod capacitance_tests;
    mod catalytic_activity_concentration_tests;
    mod catalytic_activity_tests;
    mod dynamic_viscosity_tests;
    mod electric_charge_tests;
    mod electric_current_density_tests;
    mod electric_current_tests;
    mod electric_field_tests;
    mod electric_potential_tests;
    mod electrical_conductance_tests;
    mod electrical_conductivity_tests;
    mod electrical_resistance_tests;
    mod electrical_resistivity_tests;
    mod energy_tests;
    mod force_tests;
    mod frequency_tests;
    mod heat_capacity_tests;
    mod heat_flux_density_tests;
    mod illuminance_tests;
    mod inductance_tests;
    mod kinematic_viscosity_tests;
    mod length_tests;
    mod luminance_tests;
    mod luminous_intensity_tests;
    mod magnetic_field_strength_tests;
    mod magnetic_flux_density_tests;
    mod magnetic_flux_tests;
    mod mass_concentration_tests;
    mod mass_density_tests;
    mod mass_rate_tests;
    mod mass_tests;
    mod molality_tests;
    mod molar_concentration_tests;
    mod molar_mass_tests;
    mod molar_volume_tests;
    mod power_tests;
    mod pressure_tests;
    mod radioactivity_tests;
    mod solid_angle_tests;
    mod specific_heat_capacity_tests;
    mod specific_radioactivity_tests;
    mod surface_tension_tests;
    mod temperature_interval_tests;
    mod temperature_tests;
    mod thermal_conductivity_tests;
    mod thermal_resistance_tests;
    mod time_tests;
    mod torque_tests;
    mod velocity_tests;
    mod volume_rate_tests;
    mod volume_tests;

Register mod units; in tests/measure/mod.rs in addition to the macro-level units_tests module.

- [ ] **Step 3: Move each golden test to its mirrored family file**

For every module declared in Step 2, move the complete golden function whose
name has the same family stem from unit_definition_tests.rs into the matching
file in tests/measure/units. For example, move
test_acceleration_definitions_match_exact_golden_values into
acceleration_tests.rs and
test_specific_heat_capacity_definitions_match_exact_golden_values into
specific_heat_capacity_tests.rs. The Step 2 declaration list is exhaustive;
do not change any DefinitionCase data.

Every family file imports:

    use qubit_measure::{
        assert_unit_family_valid,
        unit,
    };

    use crate::measure::support::{
        DefinitionCase,
        assert_definition_cases,
        assert_unit_contract,
    };

In addition to the moved golden test, every file adds one contract test whose
name concatenates test_, the family file stem, and _unit_contract. It calls
assert_unit_family_valid and
assert_unit_contract with that file's concrete unit enum. The exact
family-to-type mapping is the Task 5 manifest excluding Voltage. For example,
acceleration_tests.rs calls both helpers with unit::Acceleration, while
specific_heat_capacity_tests.rs calls both with unit::SpecificHeatCapacity.
Voltage is excluded because it is a measurement alias, not a distinct Unit
family.

- [ ] **Step 4: Reduce aggregate test modules to their owning responsibilities**

unit_definition_tests.rs removes the moved family golden functions and local
DefinitionCase type/helper, then retains this focused behavior:

    use qubit_measure::{
        ConversionFactor,
        ConversionOptions,
        Decimal,
        UnitDefinition,
    };
    use rust_decimal::dec;

    #[test]
    fn test_unit_definition_new_preserves_factor_and_offset() {
        let factor = ConversionFactor::new(dec!(2), dec!(3))
            .expect("factor should be valid");
        let definition = UnitDefinition::new(factor, dec!(4));

        assert_eq!(definition.factor(), factor);
        assert_eq!(definition.offset(), dec!(4));
    }

    #[test]
    fn test_unit_definition_base_uses_identity_and_zero_offset() {
        let definition = UnitDefinition::base();

        assert_eq!(definition.factor().numerator(), Decimal::ONE);
        assert_eq!(definition.factor().denominator(), Decimal::ONE);
        assert_eq!(definition.offset(), Decimal::ZERO);
    }

    #[test]
    fn test_unit_definition_convert_value_to_delegates_exact_conversion() {
        let source = UnitDefinition::new(
            ConversionFactor::new(dec!(2), Decimal::ONE)
                .expect("factor should be valid"),
            Decimal::ZERO,
        );

        assert_eq!(
            source.convert_value_to(
                dec!(3),
                UnitDefinition::base(),
                ConversionOptions::default(),
            ),
            Ok(dec!(6)),
        );
    }

units_tests.rs retains define_unit_family and shared parsing behavior only:

- canonical symbols win over aliases;
- unknown symbol errors contain quantity context;
- non-string Serde input is rejected;
- documented common alias categories normalize correctly.

Remove family availability and all-list assertions now covered by family files. Replace the single all-family round-trip function with per-family assert_unit_contract calls in mirrored files.

- [ ] **Step 5: Run the reorganized tests**

Run:

    cargo test --test mod measure::units -- --nocapture
    cargo test --test mod unit_definition_tests -- --nocapture
    cargo test --test mod units_tests -- --nocapture

Expected: all moved assertions retain their prior results and every built-in family passes assert_unit_family_valid.

- [ ] **Step 6: Review checkpoint**

Compare the old and new count of DefinitionCase entries and family contract calls. There must be 56 family files and no lost golden unit entry.

---

### Task 7: Complete Rustdoc, method order, inline attributes, and bilingual contracts

**Files:**

- Modify all hand-authored Rust files under src that contain functions, methods, fields, aliases, or ambiguous variants.
- Modify README.md and README.zh_CN.md.
- Modify Cargo.toml package description to:

      description = "Persistent typed Decimal measurements with explicit units and optional uom adapters"
- Create examples/uom_bridge.rs.

**Interfaces:**

- Documentation and attributes only, except method movement.
- No business behavior changes.

- [ ] **Step 1: Document the Cargo feature in both READMEs**

Add matching installation sections.

English:

    The exact Decimal core is the default and does not compile uom:

        qubit-measure = "0.3"

    Enable the approximate f64 bridge explicitly:

        qubit-measure = { version = "0.3", features = ["uom"] }

Chinese:

    默认构建只包含精确 Decimal 核心，不编译 uom：

        qubit-measure = "0.3"

    需要近似 f64 桥接时显式启用：

        qubit-measure = { version = "0.3", features = ["uom"] }

Update the approximate-bridge section to state that UomUnit and both
approximate methods are absent unless the feature is enabled.

Create examples/uom_bridge.rs with the standard header:

    use qubit_measure::{
        Decimal,
        measurement,
        unit,
    };
    use uom::si::length::meter;

    /// Demonstrates the explicitly enabled approximate uom bridge.
    fn main() {
        let value = measurement::Length::new(
            Decimal::new(50, 0),
            unit::Length::Centimeter,
        );
        assert_eq!(value.to_uom_approx().get::<meter>(), 0.5);
    }

Register it in Cargo.toml so default builds skip it:

    [[example]]
    name = "uom_bridge"
    required-features = ["uom"]

Use the same body in both README bridge examples. Put runnable bridge Rustdoc
examples only on items compiled under cfg(feature = "uom"), so default rustdoc
contains no unavailable import or method.

- [ ] **Step 2: Document Unit-family invariants and conflict policy**

Add equivalent English and Chinese lists:

- quantity is non-empty ASCII snake_case, begins with a lowercase letter, and has no leading, trailing, or repeated underscores;
- canonical symbols are non-empty and unique;
- aliases are non-empty and unique among aliases;
- an alias may equal another variant's canonical symbol;
- canonical symbols are checked first and therefore win;
- macro families are checked at compilation;
- manual Unit implementations should call assert_unit_family_valid in their tests;
- stable Rust cannot prove that a manual enum omitted no variant from all.

Put the same contract in Rustdoc for Unit, Unit::all, Unit::parse_lenient,
define_unit_family, and assert_unit_family_valid.

- [ ] **Step 3: Document exact mmHg semantics and sources**

Update the MillimeterOfMercury variant in pressure.rs:

    /// Millimeter of mercury using the exact Torr-equivalent definition
    /// 101325/760 Pa (20265/152 Pa), with canonical symbol mmHg.
    ///
    /// This differs from the conventional rounded 133.3224 Pa value used by
    /// some conversion tables.

Add matching README notes with:

- https://www.nist.gov/pml/special-publication-811/nist-guide-si-chapter-5-units-outside-si
- https://www.nist.gov/pml/special-publication-811/nist-guide-si-appendix-b-conversion-factors/nist-guide-si-appendix-b9

Do not change the constant, symbol, or variant name.

- [ ] **Step 4: Correct all previously ambiguous variant summaries**

Replace generic summaries with definition-qualified summaries in:

- time.rs: CommonYear365.
- energy.rs: ThermochemicalCalorie, ThermochemicalKilocalorie, BritishThermalUnitInternationalTable.
- power.rs: MechanicalHorsepower.
- volume.rs: UsFluidOunce, UsCustomaryCup, UsLiquidPint, UsLiquidQuart, UsLiquidGallon.
- volume_rate.rs: UsGallonPerMinute.
- mass_density.rs: PoundPerUsGallon.
- heat_capacity.rs: ThermochemicalCaloriePerKelvin and BritishThermalUnitInternationalTablePerDegreeFahrenheit.
- specific_heat_capacity.rs: ThermochemicalCaloriePerGramKelvin and BritishThermalUnitInternationalTablePerPoundDegreeFahrenheit.

Every summary states the definition qualifier and canonical symbol.

- [ ] **Step 5: Complete field and function Rustdoc**

Apply this exact inventory:

- conversion_options.rs: document scale and rounding fields; add Arguments and Returns to new, maximum_precision, fixed_scale, scale, rounding; retain Errors where applicable.
- conversion_factor.rs: document fields, new/from_integer arguments and
  returns, reduction semantics and equality limit, getters,
  reduce_ratio_terms, and the GCD helper.
- unit_definition.rs: document factor and offset fields; add Arguments and Returns to new, base, factor, offset, convert_value_to; retain complete Errors.
- measurement.rs: add Arguments and Returns to new, parse_strict, quantity_name, convert methods, approximate methods, Serde methods, split_measurement_parts, decimal_prefix_len, exponent_end; add Errors or Option-state descriptions where applicable.
- unit.rs: document every trait method parameter, return value, errors, and invariant; document assert_unit_family_valid and Panics.
- uom_unit.rs: document feature availability, associated Quantity, both method parameters, returns, approximation, and conversion errors.
- private.rs: document all hidden helpers and any panic invariant in decimal_to_f64_approx.
- units.rs: document hidden macros, public macro grammar, generated APIs, feature behavior, invariant failures, and examples.
- every new alias file: retain its complete type-alias Rustdoc.
- every new test helper function: add Arguments, Returns if non-unit, and Panics.

Trait implementation methods need additional Rustdoc only where their generated or local behavior adds semantics beyond the documented trait.

- [ ] **Step 6: Reorder inherent methods**

Use this final order:

- ConversionOptions: new, maximum_precision, fixed_scale, then scale, rounding.
- ConversionFactor: new, from_integer, then numerator, denominator.
- UnitDefinition: new, base, then factor, offset, convert_value_to.
- Measurement<U: Unit>: new, parse_strict, then quantity_name, convert_to, convert_to_with_options.
- Measurement<U: UomUnit>: from_uom_approx, then to_uom_approx.

Move each complete method with its Rustdoc and attributes. Do not reorder trait implementation methods.

- [ ] **Step 7: Apply the inline decision table**

Apply these attributes:

- inline(always): ConversionOptions maximum_precision, fixed_scale, scale, rounding, Default::default.
- inline: ConversionOptions::new.
- no inline: ConversionFactor::new and ratio GCD/reduction helpers.
- inline(always): ConversionFactor::from_integer, numerator, denominator.
- inline(always): UnitDefinition::new, base, factor, offset, convert_value_to.
- inline(always): Measurement::new, quantity_name, convert_to, from_uom_approx, to_uom_approx.
- inline: Measurement::convert_to_with_options and short Serde/display forwarding implementations.
- no inline: Measurement::parse_strict, FromStr parser, split_measurement_parts, decimal_prefix_len, exponent_end.
- no inline: Unit::parse_strict, Unit::parse_lenient, assert_unit_family_valid.
- inline or inline(always) in generated macro bodies according to the same table; deserialize keeps inline rather than inline(always) because it performs parsing and allocation.
- inline: private Decimal/f64 adapters; no inline on metadata validators with loops.
- no inline: convert_decimal, apply_ratio, apply_output_scale.

Do not add inline to declaration-only trait methods.

- [ ] **Step 8: Run documentation and style checks**

Run:

    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --features uom
    cargo test --doc
    cargo test --doc --features uom
    cargo test --example uom_bridge --features uom
    ./style-check.sh

Expected: all commands exit 0 with no missing-doc, broken-link, file-layout, inline-test, test-name, aggregation, or import error.

- [ ] **Step 9: Review checkpoint**

Re-run the source inventory:

    rg -n "^(pub )?(struct|enum|trait|type) " src -g "*.rs"
    rg -n "^[[:space:]]*(pub([^)]*)[[:space:]]+)?(const[[:space:]]+)?fn[[:space:]]+" src -g "*.rs"

Check every listed item against one-type-per-file, Rustdoc, method order, and inline rules.

---

### Task 8: Final verification and requirements audit

**Files:**

- Inspect every changed file.
- Modify only in-scope failures discovered by verification.

- [ ] **Step 1: Re-read the confirmed spec and this plan**

Check every acceptance criterion in the design against Tasks 1 through 7. Record any missing item before running final scripts.

- [ ] **Step 2: Inspect repository state and diff**

Run:

    git status --short
    git --no-pager diff --stat
    git --no-pager diff

Confirm no unrelated crate, .rs-ci shared source, generated dependency, or user-owned file changed.

- [ ] **Step 3: Run repository alignment**

Run:

    ./align-ci.sh

Expected: exit 0. Because this command may edit files, immediately inspect git status and diff again. Keep only in-scope formatting/alignment changes.

- [ ] **Step 4: Run CI-equivalent checks**

Run:

    ./ci-check.sh

Expected: exit 0 for formatting, Clippy, style checks, default build, release build, all-features tests, default rustdoc, README version checks, configured feature matrix, package checks, audit, and coverage threshold step as implemented by the script.

If it fails, record the command, exit status, and first relevant error. Fix only an in-scope cause, rerun ./align-ci.sh if formatting may change, then rerun ./ci-check.sh.

- [ ] **Step 5: Conditionally run detailed coverage**

Only if ./ci-check.sh explicitly reports coverage below its threshold and
coverage.sh exists, run exactly:

    ./coverage.sh json

Use the JSON report to add meaningful tests for uncovered in-scope branches, then rerun alignment and CI in order. If CI does not report a coverage shortfall, do not run this command and report it as not run because the condition was not met.

- [ ] **Step 6: Verify the two original regressions afresh**

Run:

    cargo test --features uom --test mod btu_it_uom_mapping_uses_si_oracle -- --nocapture
    cargo test --test mod decimal_conversion_reduces_two_over_two -- --nocapture
    cargo test --test mod decimal_conversion_cross_cancels_equal_large_factors -- --nocapture

Expected: 3 BTU tests pass and both Decimal regression tests pass.

- [ ] **Step 7: Verify feature isolation afresh**

Run:

    cargo tree --no-default-features
    ./.rs-ci/cargo-feature-check.sh run-all

Expected: uom is absent from the first dependency tree and both matrix entries pass.

- [ ] **Step 8: Final correction-mode report**

Report:

- corrected BTU mappings and red-green evidence;
- factor normalization and cross-cancellation evidence;
- default-off uom feature and matrix evidence;
- macro-time and manual Unit validation;
- file organization, test layout, Rustdoc, method order, and inline corrections;
- public-path preservation;
- every verification command actually run with exit status and key result;
- whether coverage JSON ran and why;
- unresolved items, remaining risks, and unchecked scope;
- confirmation that no Git commit or push occurred.
