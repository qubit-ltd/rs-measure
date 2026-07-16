// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_measure::{
    ConversionFactor,
    Decimal,
};
use rust_decimal::dec;

#[test]
fn test_conversion_factor_new_reduces_integer_terms() {
    let reduced = ConversionFactor::new(dec!(4), dec!(6))
        .expect("factor should be valid");
    let expected = ConversionFactor::new(dec!(2), dec!(3))
        .expect("factor should be valid");

    assert_eq!(reduced, expected);
    assert_eq!(reduced.numerator(), dec!(2));
    assert_eq!(reduced.denominator(), dec!(3));
}

#[test]
fn test_conversion_factor_new_cancels_common_decimal_scale() {
    let factor = ConversionFactor::new(dec!(0.4), dec!(0.1))
        .expect("factor should be valid");

    assert_eq!(factor.numerator(), dec!(4));
    assert_eq!(factor.denominator(), Decimal::ONE);
}
