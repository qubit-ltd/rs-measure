// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Approximate Decimal/f64 conversion helpers for generated uom bridges.

use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::prelude::ToPrimitive;

use crate::UnitDefinition;

/// Converts Decimal into the finite `f64` storage used by `uom`.
///
/// # Parameters
///
/// * `value` - Finite Decimal value to approximate as `f64`.
///
/// # Returns
///
/// The nearest `f64` representation selected by `rust_decimal`.
///
/// # Panics
///
/// Panics if `rust_decimal` cannot represent the finite Decimal as `f64`.
#[must_use]
#[inline]
pub fn decimal_to_f64_approx(value: Decimal) -> f64 {
    value
        .to_f64()
        .expect("Decimal is finite and within the f64 exponent range")
}

/// Converts a unit value to its approximate SI base value.
///
/// # Parameters
///
/// * `value` - Decimal value expressed by `definition`.
/// * `definition` - Exact affine definition relative to the SI base unit.
///
/// # Returns
///
/// The corresponding approximate `f64` SI base value.
#[must_use]
#[inline]
pub fn unit_value_to_base_f64(
    value: Decimal,
    definition: UnitDefinition,
) -> f64 {
    let factor = definition.factor();
    let value = decimal_to_f64_approx(value);
    let offset = decimal_to_f64_approx(definition.offset());
    let numerator = decimal_to_f64_approx(factor.numerator());
    let denominator = decimal_to_f64_approx(factor.denominator());
    (value + offset) * numerator / denominator
}

/// Converts an approximate SI base value into one exact-definition unit.
///
/// # Parameters
///
/// * `base_value` - Approximate value expressed by the SI base unit.
/// * `definition` - Exact affine definition of the requested unit.
///
/// # Returns
///
/// The approximate Decimal value expressed by `definition`.
///
/// # Errors
///
/// Returns [`crate::MeasurementError::DecimalConversion`] when the computed
/// floating-point value is non-finite or outside Decimal's representation.
#[inline]
pub fn base_f64_to_unit_value(
    base_value: f64,
    definition: UnitDefinition,
) -> Result<Decimal, crate::MeasurementError> {
    let factor = definition.factor();
    let numerator = decimal_to_f64_approx(factor.numerator());
    let denominator = decimal_to_f64_approx(factor.denominator());
    let offset = decimal_to_f64_approx(definition.offset());
    let value = base_value * denominator / numerator - offset;
    decimal_from_f64_approx(value)
}

/// Converts an `f64` value from `uom` into Decimal.
///
/// # Parameters
///
/// * `value` - Approximate floating-point value to convert.
///
/// # Returns
///
/// A finite Decimal representation when available.
///
/// # Errors
///
/// Returns [`crate::MeasurementError::DecimalConversion`] for NaN, infinity,
/// or a finite value outside Decimal's representation.
#[inline]
pub fn decimal_from_f64_approx(
    value: f64,
) -> Result<Decimal, crate::MeasurementError> {
    Decimal::from_f64(value).ok_or_else(|| {
        crate::MeasurementError::DecimalConversion(value.to_string())
    })
}
