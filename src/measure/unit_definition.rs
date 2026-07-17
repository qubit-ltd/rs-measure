// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact Decimal unit conversion definitions.

use rust_decimal::Decimal;

use crate::measure::decimal_conversion::convert_decimal;
use crate::measure::{
    ConversionFactor,
    ConversionOptions,
    MeasurementError,
};

/// Defines a unit relative to its quantity family's base unit.
///
/// The base value is `(value + offset) * numerator / denominator`.
///
/// # Examples
///
/// Discarding a definition is diagnosed when unused results are denied:
///
/// ```compile_fail
/// #![deny(unused_must_use)]
/// use qubit_measure::UnitDefinition;
///
/// UnitDefinition::base();
/// ```
///
/// Raw definitions cannot be converted publicly because they do not carry a
/// quantity identity. Use
/// [`Measurement::convert_to`](crate::Measurement::convert_to)
/// for dimension-safe public conversion:
///
/// ```compile_fail
/// use qubit_measure::{
///     ConversionOptions,
///     Decimal,
///     UnitDefinition,
/// };
///
/// let source = UnitDefinition::base();
/// let target = UnitDefinition::base();
/// let _ = source.convert_value_to(
///     Decimal::ONE,
///     target,
///     ConversionOptions::DEFAULT,
/// );
/// ```
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitDefinition {
    /// Exact positive factor applied after the offset.
    factor: ConversionFactor,

    /// Decimal offset applied before the factor.
    offset: Decimal,
}

impl UnitDefinition {
    /// Creates a unit definition from a validated factor and an offset.
    ///
    /// # Arguments
    ///
    /// * `factor` - Exact positive factor relative to the family base unit.
    /// * `offset` - Decimal offset applied before `factor`.
    ///
    /// # Returns
    ///
    /// A unit definition using `(value + offset) * factor`.
    #[inline(always)]
    pub const fn new(factor: ConversionFactor, offset: Decimal) -> Self {
        Self { factor, offset }
    }

    /// Returns the identity definition used by a quantity family's base unit.
    ///
    /// # Returns
    ///
    /// A definition with an identity factor and zero offset.
    #[inline(always)]
    pub const fn base() -> Self {
        Self {
            factor: ConversionFactor::IDENTITY,
            offset: Decimal::ZERO,
        }
    }

    /// Returns this definition's exact conversion factor.
    ///
    /// # Returns
    ///
    /// The exact positive factor applied after the offset.
    #[inline(always)]
    pub const fn factor(self) -> ConversionFactor {
        self.factor
    }

    /// Returns the offset applied before this definition's factor.
    ///
    /// # Returns
    ///
    /// The exact Decimal pre-factor offset.
    #[must_use]
    #[inline(always)]
    pub const fn offset(self) -> Decimal {
        self.offset
    }

    /// Converts `value` from this definition to `target` using Decimal only.
    ///
    /// # Arguments
    ///
    /// * `value` - Value expressed by this source definition.
    /// * `target` - Definition in which the returned value is expressed.
    /// * `options` - Final scale and rounding configuration.
    ///
    /// # Returns
    ///
    /// The exactly routed Decimal result expressed in `target`.
    ///
    /// # Errors
    ///
    /// Returns [`MeasurementError::ArithmeticOverflow`] if Decimal cannot
    /// represent an intermediate or the requested final scale.
    #[inline(always)]
    pub(crate) fn convert_value_to(
        self,
        value: Decimal,
        target: Self,
        options: ConversionOptions,
    ) -> Result<Decimal, MeasurementError> {
        convert_decimal(value, self, target, options)
    }
}
