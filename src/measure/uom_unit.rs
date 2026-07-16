// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Optional approximate adapters between persisted units and `uom` quantities.

use rust_decimal::Decimal;

use crate::measure::{
    MeasurementError,
    Unit,
};

/// A unit family that can bridge through an approximate `uom/f64` quantity.
///
/// This trait and its adapters exist only when the default-off `uom` Cargo
/// feature is enabled. The bridge converts through binary `f64`, so it is not
/// part of the exact Decimal conversion path and may lose precision.
pub trait UomUnit: Unit {
    /// The corresponding strongly typed `uom/f64` quantity.
    type Quantity: Copy;

    /// Creates an approximate `uom` quantity from a Decimal value.
    ///
    /// # Arguments
    ///
    /// * `self` - Unit in which `value` is expressed.
    /// * `value` - Decimal value to adapt through `f64`.
    ///
    /// # Returns
    ///
    /// The corresponding approximate strongly typed `uom` quantity.
    #[must_use]
    fn to_uom_approx(self, value: Decimal) -> Self::Quantity;

    /// Extracts an approximate Decimal value from a `uom` quantity.
    ///
    /// # Arguments
    ///
    /// * `self` - Unit in which the returned Decimal is expressed.
    /// * `quantity` - Strongly typed `uom/f64` quantity to adapt.
    ///
    /// # Returns
    ///
    /// The approximate Decimal value expressed in `self`.
    ///
    /// # Errors
    ///
    /// Returns [`MeasurementError::DecimalConversion`] if the floating-point
    /// result cannot be represented as Decimal.
    fn value_from_uom_approx(
        self,
        quantity: Self::Quantity,
    ) -> Result<Decimal, MeasurementError>;
}
