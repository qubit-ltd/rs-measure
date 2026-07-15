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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConversionOptions {
    scale: Option<u32>,
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
    /// # Errors
    ///
    /// Returns [`MeasurementError::InvalidScale`] when `scale` exceeds the
    /// Decimal limit.
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
    #[must_use]
    pub const fn maximum_precision(rounding: RoundingStrategy) -> Self {
        Self {
            scale: None,
            rounding,
        }
    }

    /// Creates options that round and retain exactly `scale` decimal places.
    ///
    /// # Errors
    ///
    /// Returns [`MeasurementError::InvalidScale`] when `scale` exceeds the
    /// Decimal limit.
    pub fn fixed_scale(
        scale: u32,
        rounding: RoundingStrategy,
    ) -> Result<Self, MeasurementError> {
        Self::new(Some(scale), rounding)
    }

    /// Returns the requested output scale, or `None` for maximum precision.
    #[must_use]
    pub const fn scale(self) -> Option<u32> {
        self.scale
    }

    /// Returns the strategy used for explicit output rounding.
    #[must_use]
    pub const fn rounding(self) -> RoundingStrategy {
        self.rounding
    }
}

impl Default for ConversionOptions {
    fn default() -> Self {
        Self::DEFAULT
    }
}
