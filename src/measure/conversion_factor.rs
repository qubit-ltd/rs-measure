// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact Decimal ratios used by unit definitions.

use rust_decimal::Decimal;

use crate::measure::MeasurementError;

/// A positive unit coefficient represented as an unreduced Decimal ratio.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConversionFactor {
    numerator: Decimal,
    denominator: Decimal,
}

impl ConversionFactor {
    /// The identity conversion factor used by base units.
    pub(crate) const IDENTITY: Self = Self {
        numerator: Decimal::ONE,
        denominator: Decimal::ONE,
    };

    /// Creates a positive conversion factor from a numerator and denominator.
    ///
    /// # Errors
    ///
    /// Returns [`MeasurementError::InvalidUnitDefinition`] if either term is
    /// zero or negative.
    pub fn new(
        numerator: Decimal,
        denominator: Decimal,
    ) -> Result<Self, MeasurementError> {
        if numerator <= Decimal::ZERO {
            return Err(MeasurementError::InvalidUnitDefinition {
                reason: "conversion factor numerator must be positive"
                    .to_owned(),
            });
        }
        if denominator <= Decimal::ZERO {
            return Err(MeasurementError::InvalidUnitDefinition {
                reason: "conversion factor denominator must be positive"
                    .to_owned(),
            });
        }
        Ok(Self {
            numerator,
            denominator,
        })
    }

    /// Creates a positive integer or finite-Decimal conversion factor.
    ///
    /// # Errors
    ///
    /// Returns [`MeasurementError::InvalidUnitDefinition`] if `value` is zero
    /// or negative.
    pub fn from_integer(value: Decimal) -> Result<Self, MeasurementError> {
        Self::new(value, Decimal::ONE)
    }

    /// Returns the numerator of this conversion factor.
    #[must_use]
    pub const fn numerator(self) -> Decimal {
        self.numerator
    }

    /// Returns the denominator of this conversion factor.
    #[must_use]
    pub const fn denominator(self) -> Decimal {
        self.denominator
    }
}
