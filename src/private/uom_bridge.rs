// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Approximate Decimal/f64 conversion helpers for generated uom bridges.

use rust_decimal::{
    Decimal,
    prelude::{
        FromPrimitive,
        ToPrimitive,
    },
};

/// Converts Decimal into the finite `f64` storage used by `uom`.
///
/// # Arguments
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

/// Converts an `f64` value from `uom` into Decimal.
///
/// # Arguments
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
