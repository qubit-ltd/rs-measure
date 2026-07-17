// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Public contract tests covering private literal syntax scanning.

use qubit_measure::__private::decimal_from_literal;
use rust_decimal::dec;

#[test]
fn test_scanner_recognizes_sign_radix_fraction_and_exponent_phases() {
    assert_eq!(decimal_from_literal("- 0x1_FF"), dec!(-511));
    assert_eq!(decimal_from_literal("-1.25e-2"), dec!(-0.0125));
}
