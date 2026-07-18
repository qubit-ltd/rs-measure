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

/// A positive unit coefficient represented as reduced Decimal ratio terms.
///
/// [`ConversionFactor::new`] removes common mantissa factors and common scale
/// from its inputs. Equality compares the stored reduced terms; it is not a
/// general mathematical-equivalence solver at Decimal's representation limits.
///
/// # Examples
///
/// Discarding a validated factor is diagnosed when unused results are denied:
///
/// ```compile_fail
/// #![deny(unused_must_use)]
/// use qubit_measure::{ConversionFactor, Decimal};
///
/// ConversionFactor::new(Decimal::ONE, Decimal::ONE)
///     .expect("identity factor should be valid");
/// ```
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConversionFactor {
    /// Positive numerator of the exact conversion ratio.
    pub(crate) numerator: Decimal,

    /// Positive denominator of the exact conversion ratio.
    pub(crate) denominator: Decimal,
}

impl ConversionFactor {
    /// The identity conversion factor used by base units.
    pub(crate) const IDENTITY: Self = Self {
        numerator: Decimal::ONE,
        denominator: Decimal::ONE,
    };

    /// Creates a positive conversion factor from a numerator and denominator.
    ///
    /// Common mantissa factors and common scale are removed before the factor
    /// is stored.
    ///
    /// # Parameters
    ///
    /// * `numerator` - The positive numerator of the conversion ratio.
    /// * `denominator` - The positive denominator of the conversion ratio.
    ///
    /// # Returns
    ///
    /// The validated factor with reduced ratio terms.
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
        if numerator.scale() == 0 && denominator.scale() == 0 {
            return Ok(Self::from_const_integers(
                numerator.mantissa(),
                denominator.mantissa(),
            ));
        }
        let (numerator, denominator) =
            reduce_ratio_terms(numerator, denominator);
        Ok(Self {
            numerator,
            denominator,
        })
    }

    /// Creates a positive finite-Decimal conversion factor.
    ///
    /// # Parameters
    ///
    /// * `value` - Positive finite Decimal coefficient.
    ///
    /// # Returns
    ///
    /// A validated factor equivalent to `value / 1`.
    ///
    /// # Errors
    ///
    /// Returns [`MeasurementError::InvalidUnitDefinition`] if `value` is zero
    /// or negative.
    #[inline(always)]
    pub fn from_decimal(value: Decimal) -> Result<Self, MeasurementError> {
        Self::new(value, Decimal::ONE)
    }

    /// Creates a reduced conversion factor from positive integer terms in const
    /// contexts.
    ///
    /// # Parameters
    ///
    /// * `numerator` - Positive integer numerator.
    /// * `denominator` - Positive integer denominator.
    ///
    /// # Returns
    ///
    /// A factor whose integer terms have no common divisor.
    ///
    /// # Panics
    ///
    /// Panics if either term is non-positive or a reduced term exceeds
    /// Decimal's 96-bit coefficient range.
    #[inline]
    pub const fn from_const_integers(
        numerator: i128,
        denominator: i128,
    ) -> Self {
        assert!(numerator > 0);
        assert!(denominator > 0);
        let divisor = greatest_common_divisor(numerator, denominator);
        Self {
            numerator: decimal_from_positive_integer(numerator / divisor),
            denominator: decimal_from_positive_integer(denominator / divisor),
        }
    }

    /// Returns the numerator of this conversion factor.
    ///
    /// # Returns
    ///
    /// The positive reduced numerator.
    #[must_use]
    #[inline(always)]
    pub const fn numerator(self) -> Decimal {
        self.numerator
    }

    /// Returns the denominator of this conversion factor.
    ///
    /// # Returns
    ///
    /// The positive reduced denominator.
    #[must_use]
    #[inline(always)]
    pub const fn denominator(self) -> Decimal {
        self.denominator
    }
}

/// Reduces a positive Decimal ratio without multiplying either term.
///
/// # Parameters
///
/// * `numerator` - The positive ratio numerator.
/// * `denominator` - The positive ratio denominator.
///
/// # Returns
///
/// An equivalent numerator and denominator with their mantissa GCD and common
/// scale removed.
pub(crate) fn reduce_ratio_terms(
    numerator: Decimal,
    denominator: Decimal,
) -> (Decimal, Decimal) {
    let numerator_scale = numerator.scale();
    let denominator_scale = denominator.scale();
    let common_scale = numerator_scale.min(denominator_scale);
    let numerator_mantissa = numerator.mantissa();
    let denominator_mantissa = denominator.mantissa();
    let divisor =
        greatest_common_divisor(numerator_mantissa, denominator_mantissa);

    (
        Decimal::from_i128_with_scale(
            numerator_mantissa / divisor,
            numerator_scale - common_scale,
        ),
        Decimal::from_i128_with_scale(
            denominator_mantissa / divisor,
            denominator_scale - common_scale,
        ),
    )
}

/// Computes the greatest common divisor of two positive integers.
///
/// # Parameters
///
/// * `lhs` - The first positive integer.
/// * `rhs` - The second positive integer.
///
/// # Returns
///
/// The positive greatest common divisor.
const fn greatest_common_divisor(mut lhs: i128, mut rhs: i128) -> i128 {
    while rhs != 0 {
        let remainder = lhs % rhs;
        lhs = rhs;
        rhs = remainder;
    }
    lhs
}

/// Converts a positive integer to an unscaled Decimal constant.
///
/// # Parameters
///
/// * `value` - The positive integer to convert.
///
/// # Returns
///
/// The equivalent unscaled Decimal value.
///
/// # Panics
///
/// Panics if `value` is non-positive or exceeds Decimal's 96-bit coefficient
/// range.
#[inline]
const fn decimal_from_positive_integer(value: i128) -> Decimal {
    assert!(value > 0);
    let magnitude = value as u128;
    assert!(magnitude >> 96 == 0);
    Decimal::from_parts(
        magnitude as u32,
        (magnitude >> 32) as u32,
        (magnitude >> 64) as u32,
        false,
        0,
    )
}
