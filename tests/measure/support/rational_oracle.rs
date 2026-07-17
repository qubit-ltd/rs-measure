// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Independent exact-rational oracle for Decimal unit conversion tests.

use num_bigint::BigInt;
use num_rational::BigRational;
use qubit_measure::{
    Decimal,
    UnitDefinition,
};

/// Converts a Decimal into its exact rational representation.
///
/// # Parameters
///
/// * `value` - Decimal value to represent independently.
///
/// # Returns
///
/// An exact rational with the Decimal mantissa over its power-of-ten scale.
pub(crate) fn decimal_as_rational(value: Decimal) -> BigRational {
    let denominator = BigInt::from(10_u8).pow(value.scale());
    BigRational::new(BigInt::from(value.mantissa()), denominator)
}

/// Computes the exact affine conversion independently of production ordering.
///
/// # Parameters
///
/// * `value` - Decimal value expressed by `source`.
/// * `source` - Source unit definition.
/// * `target` - Target unit definition.
///
/// # Returns
///
/// The exact mathematical result as a rational number.
pub(crate) fn expected_conversion(
    value: Decimal,
    source: UnitDefinition,
    target: UnitDefinition,
) -> BigRational {
    let source_factor = source.factor();
    let target_factor = target.factor();
    let source_ratio = decimal_as_rational(source_factor.numerator())
        / decimal_as_rational(source_factor.denominator());
    let target_ratio = decimal_as_rational(target_factor.numerator())
        / decimal_as_rational(target_factor.denominator());

    (decimal_as_rational(value) + decimal_as_rational(source.offset()))
        * source_ratio
        / target_ratio
        - decimal_as_rational(target.offset())
}
