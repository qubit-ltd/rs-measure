// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Implementation dependencies used by exported declarative macros.

pub use rust_decimal;
pub use serde;
pub use uom;

use rust_decimal::Decimal;
use rust_decimal::prelude::{
    FromPrimitive,
    ToPrimitive,
};

/// Converts Decimal into the finite `f64` storage used by `uom`.
#[must_use]
pub fn decimal_to_f64_approx(value: Decimal) -> f64 {
    value
        .to_f64()
        .expect("Decimal is finite and within the f64 exponent range")
}

/// Converts an `f64` value from `uom` into Decimal.
pub fn decimal_from_f64_approx(
    value: f64,
) -> Result<Decimal, crate::MeasurementError> {
    Decimal::from_f64(value).ok_or_else(|| {
        crate::MeasurementError::DecimalConversion(value.to_string())
    })
}
