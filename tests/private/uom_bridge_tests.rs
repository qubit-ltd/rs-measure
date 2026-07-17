// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Approximate Decimal/f64 hidden-helper contract tests.

use qubit_measure::{
    __private::{
        decimal_from_f64_approx,
        decimal_to_f64_approx,
    },
    Decimal,
    MeasurementError,
};

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
