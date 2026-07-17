// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Public contract tests covering top-level literal parser orchestration.

use qubit_measure::__private::{
    decimal_from_literal,
    positive_decimal_from_literal,
};
use rust_decimal::dec;

#[test]
fn test_parser_combines_fraction_and_scientific_exponent() {
    assert_eq!(decimal_from_literal("123.45e-2"), dec!(1.2345));
}

/// Verifies that the coefficient parser preserves a positive Decimal literal.
#[test]
fn test_positive_decimal_parser_accepts_positive_literal() {
    assert_eq!(positive_decimal_from_literal("1.25"), dec!(1.25));
}

/// Verifies that zero and negative coefficient terms are rejected.
#[test]
fn test_positive_decimal_parser_rejects_non_positive_literal() {
    for value in ["0", "-1"] {
        let panic = std::panic::catch_unwind(|| {
            positive_decimal_from_literal(std::hint::black_box(value));
        })
        .expect_err("non-positive coefficient term should panic");
        let message = panic
            .downcast_ref::<&str>()
            .copied()
            .or_else(|| panic.downcast_ref::<String>().map(String::as_str))
            .expect("coefficient panic should contain a string message");

        assert_eq!(message, "unit coefficient terms must be positive");
    }
}
