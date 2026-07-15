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
pub trait UomUnit: Unit {
    /// The corresponding strongly typed `uom` quantity.
    type Quantity: Copy;

    /// Creates an approximate `uom` quantity from a Decimal value.
    #[must_use]
    fn to_uom_approx(self, value: Decimal) -> Self::Quantity;

    /// Extracts an approximate Decimal value from a `uom` quantity.
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
