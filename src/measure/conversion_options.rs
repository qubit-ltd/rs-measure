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

    /// Creates options that do not impose a fixed output scale.
    ///
    /// Conversion arithmetic remains exact while represented as a rational.
    /// If the result is not exactly representable as Decimal, conversion uses
    /// nearest-even rounding at the greatest scale whose mantissa fits, then
    /// normalizes trailing zeroes. When the source and target definitions are
    /// equal, conversion instead preserves the original Decimal representation,
    /// including its scale and trailing zeroes.
    ///
    /// # Returns
    ///
    /// Options that retain the greatest representable Decimal precision.
    #[inline(always)]
    pub const fn maximum_precision() -> Self {
        Self {
            mode: ConversionMode::MaximumPrecision,
        }
    }

    /// Creates options that round and retain exactly `scale` decimal places.
    ///
    /// # Parameters
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
    #[inline]
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
    /// The configured strategy for fixed-scale output. `None` means no
    /// caller-selected fixed-scale strategy; maximum-precision conversion may
    /// still use nearest-even rounding at Decimal's representation boundary.
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
