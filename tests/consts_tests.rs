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
    Unit,
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
