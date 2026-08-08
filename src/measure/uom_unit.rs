// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Optional approximate adapters between persisted units and `uom` quantities.

use rust_decimal::Decimal;

use crate::measure::MeasurementError;
use crate::measure::Unit;

/// A unit family that can bridge through an approximate `uom/f64` quantity.
///
/// This trait and its adapters exist only when the default-off `uom` Cargo
/// feature is enabled. The bridge first applies the family's exact
/// [`UnitDefinition`](crate::UnitDefinition) to obtain its abstract SI base
/// value, then crosses binary `f64`. The physical base value therefore follows
/// `qubit-measure` semantics, although precision may be lost. Reading the
/// resulting quantity through a non-base `uom` unit still uses that unit's own
/// coefficient, which may intentionally differ from this crate's definition.
pub trait UomUnit: Unit {
    /// The corresponding strongly typed `uom/f64` quantity.
    type Quantity: Copy;

    /// Tries to create an approximate `uom` quantity from a Decimal value.
    ///
    /// Implementations must validate any fallible external unit definition
    /// before constructing the approximate quantity.
    ///
    /// # Parameters
    ///
    /// * `self` - Unit in which `value` is expressed.
    /// * `value` - Decimal value to adapt through `f64`.
    ///
    /// # Returns
    ///
    /// The corresponding approximate strongly typed `uom` quantity.
    ///
    /// # Errors
    ///
    /// Returns [`MeasurementError::InvalidUnitDefinition`] when an external
    /// unit family cannot provide a valid exact definition.
    fn try_to_uom_approx(
        self,
        value: Decimal,
    ) -> Result<Self::Quantity, MeasurementError>;

    /// Creates an approximate `uom` quantity from a Decimal value.
    ///
    /// # Parameters
    ///
    /// * `self` - Unit in which `value` is expressed.
    /// * `value` - Decimal value to adapt through `f64`.
    ///
    /// # Returns
    ///
    /// The corresponding approximate strongly typed `uom` quantity.
    ///
    /// # Panics
    ///
    /// The default implementation panics if
    /// [`UomUnit::try_to_uom_approx`] returns an error. Call the fallible
    /// method when the definition comes from an external manual
    /// implementation.
    #[must_use]
    #[inline(always)]
    fn to_uom_approx(self, value: Decimal) -> Self::Quantity {
        self.try_to_uom_approx(value)
            .expect("UomUnit::try_to_uom_approx returned an error")
    }

    /// Extracts an approximate Decimal value from a `uom` quantity.
    ///
    /// # Parameters
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
    /// Returns [`MeasurementError::InvalidUnitDefinition`] when an external
    /// unit family cannot provide a valid exact definition. Returns
    /// [`MeasurementError::DecimalConversion`] if the floating-point result
    /// cannot be represented as Decimal.
    fn value_from_uom_approx(
        self,
        quantity: Self::Quantity,
    ) -> Result<Decimal, MeasurementError>;
}
