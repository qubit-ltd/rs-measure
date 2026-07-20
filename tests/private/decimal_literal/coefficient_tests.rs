// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Public contract tests covering private coefficient construction.

use qubit_measure::__private::decimal_from_literal;
use rust_decimal::Decimal;

#[test]
fn test_coefficient_builder_preserves_decimal_range_boundaries() {
    assert_eq!(
        decimal_from_literal("79228162514264337593543950335"),
        Decimal::MAX,
    );
    assert_eq!(decimal_from_literal("1e-28"), Decimal::new(1, 28));
}

/// Verifies that scientific literals cancel trailing coefficient zeroes.
#[test]
fn test_coefficient_builder_cancels_scientific_trailing_zeroes() {
    assert_eq!(decimal_from_literal("1.0e-28"), Decimal::new(1, 28),);
    assert_eq!(decimal_from_literal("100e-29"), Decimal::new(10, 28),);
}

/// Verifies that deferred zeroes do not overflow the intermediate coefficient.
#[test]
fn test_coefficient_builder_defers_long_trailing_zero_runs() {
    assert_eq!(
        decimal_from_literal("10000000000000000000000000000000000000000e-40",),
        Decimal::ONE,
    );
}

/// Verifies scale restoration and zero exponent handling at literal boundaries.
#[test]
fn test_coefficient_builder_preserves_scale_and_extreme_zeroes() {
    let scaled = decimal_from_literal("1.00e0");

    assert_eq!(scaled, Decimal::new(100, 2));
    assert_eq!(scaled.scale(), 2);
    assert_eq!(decimal_from_literal("0e-999"), Decimal::new(0, 28));
    assert_eq!(decimal_from_literal("0e999"), Decimal::ZERO);
}
