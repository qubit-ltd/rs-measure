// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact built-in conversion-factor tests.

use qubit_measure::{
    ConversionFactor,
    ConversionOptions,
    Decimal,
    Unit,
    UnitDefinition,
    unit,
};
use rust_decimal::dec;

#[test]
fn test_builtin_revolution_factor_uses_reduced_terms() {
    let builtin = unit::Angle::Revolution
        .definition()
        .expect("revolution definition should be valid")
        .factor();
    let normalized =
        ConversionFactor::new(dec!(3141592653589793), dec!(500000000000000))
            .expect("normalized revolution factor should be valid");

    assert_eq!(builtin, normalized);
}

#[test]
fn test_builtin_revolution_matches_equivalent_external_definition_at_max() {
    let builtin = unit::Angle::Revolution
        .definition()
        .expect("revolution definition should be valid");
    let normalized = UnitDefinition::new(
        ConversionFactor::new(dec!(3141592653589793), dec!(500000000000000))
            .expect("normalized revolution factor should be valid"),
        Decimal::ZERO,
    );

    assert_eq!(
        builtin
            .convert_value_to(
                Decimal::MAX,
                normalized,
                ConversionOptions::default(),
            )
            .expect("equivalent definitions should preserve Decimal::MAX"),
        Decimal::MAX,
    );
}
