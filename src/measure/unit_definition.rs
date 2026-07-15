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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitDefinition {
    factor: ConversionFactor,
    offset: Decimal,
}

impl UnitDefinition {
    /// Creates a unit definition from a validated factor and an offset.
    #[must_use]
    pub const fn new(factor: ConversionFactor, offset: Decimal) -> Self {
        Self { factor, offset }
    }

    /// Returns the identity definition used by a quantity family's base unit.
    #[must_use]
    pub fn base() -> Self {
        Self {
            factor: ConversionFactor::IDENTITY,
            offset: Decimal::ZERO,
        }
    }

    /// Returns this definition's exact conversion factor.
    #[must_use]
    pub const fn factor(self) -> ConversionFactor {
        self.factor
    }

    /// Returns the offset applied before this definition's factor.
    #[must_use]
    pub const fn offset(self) -> Decimal {
        self.offset
    }

    /// Converts `value` from this definition to `target` using Decimal only.
    ///
    /// # Errors
    ///
    /// Returns [`MeasurementError::ArithmeticOverflow`] if Decimal cannot
    /// represent an intermediate or the requested final scale.
    pub fn convert_value_to(
        self,
        value: Decimal,
        target: Self,
        options: ConversionOptions,
    ) -> Result<Decimal, MeasurementError> {
        convert_decimal(value, self, target, options)
    }
}
