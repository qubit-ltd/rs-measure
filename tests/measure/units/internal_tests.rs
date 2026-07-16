// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Public contract tests for private unit implementation aggregation.

use qubit_measure::{
    ConversionFactor,
    Unit,
    unit,
};
use rust_decimal::dec;

/// Verifies that the private pressure helper is exposed through its unit
/// definition.
#[test]
fn test_units_internal_preserves_exact_torr_equivalent() {
    let actual = unit::Pressure::MillimeterOfMercury
        .definition()
        .expect("millimeter of mercury definition should be valid")
        .factor();
    let expected = ConversionFactor::new(dec!(101325), dec!(760))
        .expect("Torr-equivalent factor should be valid");

    assert_eq!(actual, expected);
}
