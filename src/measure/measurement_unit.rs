/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Traits shared by persisted measurement units.

use crate::measure::MeasurementError;
use rust_decimal::Decimal;
use std::fmt;
use std::str::FromStr;

/// A persisted unit marker for one concrete `uom` quantity.
///
/// Implementations bridge stable serialized unit symbols, decimal persistence
/// values, and the strongly typed `uom` quantity used for calculation.
pub trait MeasurementUnit: Copy + Eq + fmt::Display + FromStr<Err = MeasurementError> + 'static {
    /// The `uom` quantity type represented by this unit family.
    type Quantity: Copy;

    /// Stable lower-case quantity name used in diagnostics.
    const QUANTITY: &'static str;

    /// Returns all unit variants supported by this crate version.
    #[must_use]
    fn all() -> &'static [Self];

    /// Returns the stable symbol used when serializing this unit.
    #[must_use]
    fn symbol(self) -> &'static str;

    /// Creates a `uom` quantity from a decimal value expressed in this unit.
    ///
    /// Returns [`MeasurementError::DecimalConversion`] when the decimal value
    /// cannot be represented as a finite `f64` for the configured `uom` backend.
    fn to_uom(self, value: Decimal) -> Result<Self::Quantity, MeasurementError>;

    /// Extracts a decimal value from a `uom` quantity in this unit.
    ///
    /// Returns [`MeasurementError::DecimalConversion`] when the `uom` value
    /// cannot be represented as [`Decimal`].
    fn value_from_uom(self, quantity: Self::Quantity) -> Result<Decimal, MeasurementError>;
}
