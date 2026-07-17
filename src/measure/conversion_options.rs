// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Decimal conversion precision and rounding configuration.

use rust_decimal::{
    Decimal,
    RoundingStrategy,
};

use crate::measure::MeasurementError;

/// Controls the final scale and rounding applied to a converted value.
///
/// # Examples
///
/// Discarding configured options is diagnosed when unused results are denied:
///
/// ```compile_fail
/// #![deny(unused_must_use)]
/// use qubit_measure::{ConversionOptions, RoundingStrategy};
///
/// ConversionOptions::maximum_precision(
///     RoundingStrategy::MidpointNearestEven,
/// );
/// ```
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConversionOptions {
    /// Requested output scale, or `None` to keep maximum precision.
    scale: Option<u32>,

    /// Rounding strategy used when an output scale is requested.
    rounding: RoundingStrategy,
}

impl ConversionOptions {
    /// The deterministic options used by [`Measurement::convert_to`].
    ///
    /// [`Measurement::convert_to`]: crate::Measurement::convert_to
    pub const DEFAULT: Self = Self {
        scale: None,
        rounding: RoundingStrategy::MidpointNearestEven,
    };

    /// Creates conversion options.
    ///
    /// A `None` scale preserves the maximum precision available from Decimal
    /// arithmetic. A concrete scale must not exceed [`Decimal::MAX_SCALE`].
    ///
    /// # Arguments
    ///
    /// * `scale` - Requested output scale, or `None` for maximum precision.
    /// * `rounding` - Strategy used when rounding to a requested scale.
    ///
    /// # Returns
    ///
    /// Validated immutable conversion options.
    ///
    /// # Errors
    ///
    /// Returns [`MeasurementError::InvalidScale`] when `scale` exceeds the
    /// Decimal limit.
    #[inline]
    pub fn new(
        scale: Option<u32>,
        rounding: RoundingStrategy,
    ) -> Result<Self, MeasurementError> {
        if let Some(scale) = scale
            && scale > Decimal::MAX_SCALE
        {
            return Err(MeasurementError::InvalidScale {
                scale,
                max: Decimal::MAX_SCALE,
            });
        }
        Ok(Self { scale, rounding })
    }

    /// Creates options that do not impose an additional output scale.
    ///
    /// # Arguments
    ///
    /// * `rounding` - Strategy retained for any later explicit rounding.
    ///
    /// # Returns
    ///
    /// Options that preserve the maximum Decimal precision.
    #[inline(always)]
    pub const fn maximum_precision(rounding: RoundingStrategy) -> Self {
        Self {
            scale: None,
            rounding,
        }
    }

    /// Creates options that round and retain exactly `scale` decimal places.
    ///
    /// # Arguments
    ///
    /// * `scale` - Exact number of decimal places to retain.
    /// * `rounding` - Strategy used to round the result.
    ///
    /// # Returns
    ///
    /// Validated fixed-scale conversion options.
    ///
    /// # Errors
    ///
    /// Returns [`MeasurementError::InvalidScale`] when `scale` exceeds the
    /// Decimal limit.
    #[inline(always)]
    pub fn fixed_scale(
        scale: u32,
        rounding: RoundingStrategy,
    ) -> Result<Self, MeasurementError> {
        Self::new(Some(scale), rounding)
    }

    /// Returns the requested output scale, or `None` for maximum precision.
    ///
    /// # Returns
    ///
    /// The requested Decimal scale, if one was configured.
    #[must_use]
    #[inline(always)]
    pub const fn scale(self) -> Option<u32> {
        self.scale
    }

    /// Returns the strategy used for explicit output rounding.
    ///
    /// # Returns
    ///
    /// The configured Decimal rounding strategy.
    #[must_use]
    #[inline(always)]
    pub const fn rounding(self) -> RoundingStrategy {
        self.rounding
    }
}

impl Default for ConversionOptions {
    #[inline(always)]
    fn default() -> Self {
        Self::DEFAULT
    }
}
