// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact Torr-equivalent pressure definition tests.

use qubit_measure::{
    Unit,
    unit,
};
use rust_decimal::dec;

/// Verifies the public pressure definition reduces 101325/760 exactly.
#[test]
fn test_exact_torr_equivalent_matches_public_pressure_definition() {
    let factor = unit::Pressure::MillimeterOfMercury
        .definition()
        .expect("millimeter of mercury definition should be valid")
        .factor();

    assert_eq!(factor.numerator(), dec!(20265));
    assert_eq!(factor.denominator(), dec!(152));
}
