// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_measure::{
    MeasurementError,
    Unit,
    measurement,
    unit,
};
use rust_decimal::Decimal;

#[test]
fn test_unit_trait_exposes_typed_quantity_metadata() {
    let measurement =
        measurement::Length::new(Decimal::new(50, 0), unit::Length::Centimeter);

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
        unit::Power::parse_lenient("hp")
            .expect("horsepower alias should parse"),
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
