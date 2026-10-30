// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Approximate Decimal/f64 hidden-helper contract tests.

use qubit_measure::__private::base_f64_to_unit_value;
use qubit_measure::__private::decimal_from_f64_approx;
use qubit_measure::__private::decimal_to_f64_approx;
use qubit_measure::__private::unit_value_to_base_f64;
use qubit_measure::ConversionFactor;
use qubit_measure::MeasurementError;
use qubit_measure::UnitDefinition;
use rust_decimal::Decimal;
use rust_decimal::dec;
use rust_decimal::prelude::ToPrimitive;

#[test]
fn test_uom_bridge_helpers_convert_finite_values() {
    let value = Decimal::new(125, 2);
    let approximate = decimal_to_f64_approx(value);

    assert_eq!(approximate, 1.25);
    assert_eq!(decimal_from_f64_approx(approximate), Ok(value));
}

#[test]
fn test_decimal_from_f64_approx_rejects_non_finite_values() {
    assert_eq!(
        decimal_from_f64_approx(f64::INFINITY),
        Err(MeasurementError::DecimalConversion("inf".to_owned())),
    );
}

#[test]
fn test_unit_value_and_base_f64_follow_exact_definition() {
    let definition = UnitDefinition::new(
        ConversionFactor::new(dec!(5), dec!(9)).expect("positive ratio should be valid"),
        dec!(459.67),
    );

    let base = unit_value_to_base_f64(dec!(32), definition);
    assert!((base - 273.15).abs() <= 1.0E-12);

    let round_trip =
        base_f64_to_unit_value(base, definition).expect("finite base value should convert back to Decimal");
    let round_trip = round_trip.to_f64().expect("round-trip Decimal should fit f64");
    assert!((round_trip - 32.0).abs() <= 1.0E-12);
}

#[test]
fn test_unit_value_to_base_f64_preserves_affine_range() {
    let large_offset = UnitDefinition::new(
        ConversionFactor::new(dec!(1), dec!(2)).expect("positive ratio should be valid"),
        Decimal::MAX,
    );
    assert_eq!(
        unit_value_to_base_f64(Decimal::MAX, large_offset),
        decimal_to_f64_approx(Decimal::MAX),
    );

    let tiny_positive_factor = UnitDefinition::new(
        ConversionFactor::new(Decimal::ONE, Decimal::MAX).expect("positive ratio should be valid"),
        Decimal::ZERO,
    );
    let tiny_positive = unit_value_to_base_f64(Decimal::ONE, tiny_positive_factor);
    assert!(tiny_positive.is_finite());
    assert!(tiny_positive > 0.0);

    let large_factor = UnitDefinition::new(
        ConversionFactor::new(Decimal::MAX, Decimal::ONE).expect("positive ratio should be valid"),
        Decimal::ZERO,
    );
    let large_positive = unit_value_to_base_f64(Decimal::MAX, large_factor);
    let large_negative = unit_value_to_base_f64(-Decimal::MAX, large_factor);
    assert!(large_positive.is_finite() && large_positive > 0.0);
    assert!(large_negative.is_finite() && large_negative < 0.0);
}
