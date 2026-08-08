// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_measure::MeasurementError;
use qubit_measure::Unit;
use qubit_measure::assert_unit_family_valid;
use qubit_measure::measurement;
use qubit_measure::unit;
use rust_decimal::Decimal;

use crate::measure::support::CANONICAL_ALIAS;
use crate::measure::support::DISPLAY_MISMATCH;
use crate::measure::support::DUPLICATE_ALIAS;
use crate::measure::support::DUPLICATE_ALL;
use crate::measure::support::DUPLICATE_SYMBOL;
use crate::measure::support::INVALID_DEFINITION;
use crate::measure::support::INVALID_QUANTITY;
use crate::measure::support::LENIENT_FROM_STR;
use crate::measure::support::ManualValidationUnit;
use crate::measure::support::SELF_ALIAS;
use crate::measure::support::VALID;

#[test]
fn test_unit_trait_exposes_typed_quantity_metadata() {
    let measurement = measurement::Length::new(Decimal::new(50, 0), unit::Length::Centimeter);

    assert_eq!(measurement.quantity_name(), "length");
    assert_eq!(unit::Length::QUANTITY, "length");
    assert_eq!(unit::Length::Meter.symbol(), "m");
}

#[test]
fn test_ambiguous_aliases_resolve_leniently_and_fail_strictly() {
    assert_eq!(
        unit::Time::parse_lenient("year").expect("year alias should parse"),
        unit::Time::CommonYear365,
    );
    assert_eq!(unit::Time::CommonYear365.symbol(), "a (365 d)");
    assert!(matches!(
        unit::Time::parse_strict("year"),
        Err(MeasurementError::NonCanonicalUnit { .. }),
    ));
    assert_eq!(
        unit::Energy::parse_lenient("Btu").expect("Btu alias should parse"),
        unit::Energy::BritishThermalUnitInternationalTable,
    );
    assert_eq!(
        unit::Volume::parse_lenient("gal").expect("gallon alias should parse"),
        unit::Volume::UsLiquidGallon,
    );
    assert_eq!(
        unit::Power::parse_lenient("hp").expect("horsepower alias should parse"),
        unit::Power::MechanicalHorsepower,
    );
}

#[test]
fn test_strict_unit_parser_rejects_unknown_unit() {
    assert!(matches!(
        unit::Time::parse_strict("fortnight"),
        Err(MeasurementError::UnknownUnit { .. }),
    ));
}

#[test]
fn test_lenient_unit_parser_rejects_unknown_unit() {
    assert!(matches!(
        unit::Time::parse_lenient("fortnight"),
        Err(MeasurementError::UnknownUnit { .. }),
    ));
}

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

/// Verifies that manual metadata rejects an alias equal to its own symbol.
#[test]
#[should_panic(expected = "unit alias must not match any canonical symbol")]
fn test_assert_unit_family_valid_rejects_own_canonical_alias() {
    assert_unit_family_valid::<ManualValidationUnit<SELF_ALIAS>>();
}

/// Verifies that manual metadata rejects an alias equal to another symbol.
#[test]
#[should_panic(expected = "unit alias must not match any canonical symbol")]
fn test_assert_unit_family_valid_rejects_other_canonical_alias() {
    assert_unit_family_valid::<ManualValidationUnit<CANONICAL_ALIAS>>();
}

#[test]
#[should_panic(expected = "ASCII snake_case")]
fn test_assert_unit_family_valid_rejects_invalid_quantity() {
    assert_unit_family_valid::<ManualValidationUnit<INVALID_QUANTITY>>();
}

#[test]
#[should_panic(expected = "invalid definition for derived")]
fn test_assert_unit_family_valid_rejects_invalid_definition() {
    assert_unit_family_valid::<ManualValidationUnit<INVALID_DEFINITION>>();
}

#[test]
#[should_panic(expected = "Display must emit canonical symbol")]
fn test_assert_unit_family_valid_rejects_display_symbol_mismatch() {
    assert_unit_family_valid::<ManualValidationUnit<DISPLAY_MISMATCH>>();
}

#[test]
#[should_panic(expected = "FromStr must reject alias")]
fn test_assert_unit_family_valid_rejects_lenient_from_str() {
    assert_unit_family_valid::<ManualValidationUnit<LENIENT_FROM_STR>>();
}
