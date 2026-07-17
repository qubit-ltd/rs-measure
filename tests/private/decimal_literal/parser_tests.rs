// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Public contract tests covering top-level literal parser orchestration.

use qubit_measure::__private::decimal_from_literal;
use rust_decimal::dec;

#[test]
fn test_parser_combines_fraction_and_scientific_exponent() {
    assert_eq!(decimal_from_literal("123.45e-2"), dec!(1.2345));
}
