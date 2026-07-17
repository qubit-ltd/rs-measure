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
use crate::measure::internal::ConversionMode;

/// Controls the final scale and rounding applied to a converted value.
///
/// # Examples
///
/// Discarding configured options is diagnosed when unused results are denied:
///
/// ```compile_fail
/// #![deny(unused_must_use)]
/// use qubit_measure::ConversionOptions;
///
/// ConversionOptions::maximum_precision();
/// ```
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConversionOptions {
    /// Valid output policy selected by the public constructors.
    mode: ConversionMode,
}

impl ConversionOptions {
    /// The deterministic options used by [`Measurement::convert_to`].
    ///
    /// [`Measurement::convert_to`]: crate::Measurement::convert_to
    pub const DEFAULT: Self = Self {
        mode: ConversionMode::MaximumPrecision,
    };

    /// Creates options that do not impose an additional output scale.
    ///
    /// # Returns
    ///
    /// Options that preserve the maximum Decimal precision.
    #[inline(always)]
    pub const fn maximum_precision() -> Self {
        Self {
            mode: ConversionMode::MaximumPrecision,
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
        if scale > Decimal::MAX_SCALE {
            return Err(MeasurementError::InvalidScale {
                scale,
                max: Decimal::MAX_SCALE,
            });
        }
        Ok(Self {
            mode: ConversionMode::FixedScale { scale, rounding },
        })
    }

    /// Returns the requested output scale, or `None` for maximum precision.
    ///
    /// # Returns
    ///
    /// The requested Decimal scale, if one was configured.
    #[must_use]
    #[inline(always)]
    pub const fn scale(self) -> Option<u32> {
        match self.mode {
            ConversionMode::MaximumPrecision => None,
            ConversionMode::FixedScale { scale, .. } => Some(scale),
        }
    }

    /// Returns the strategy used for explicit output rounding, if configured.
    ///
    /// # Returns
    ///
    /// The configured strategy for fixed-scale output, or `None` for maximum
    /// precision.
    #[must_use]
    #[inline(always)]
    pub const fn rounding(self) -> Option<RoundingStrategy> {
        match self.mode {
            ConversionMode::MaximumPrecision => None,
            ConversionMode::FixedScale { rounding, .. } => Some(rounding),
        }
    }

    /// Returns the validated internal conversion mode.
    ///
    /// # Returns
    ///
    /// The internal maximum-precision or fixed-scale policy.
    #[inline(always)]
    pub(super) const fn mode(self) -> ConversionMode {
        self.mode
    }
}

impl Default for ConversionOptions {
    #[inline(always)]
    fn default() -> Self {
        Self::DEFAULT
    }
}
