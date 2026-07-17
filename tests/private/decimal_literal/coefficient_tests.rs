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
