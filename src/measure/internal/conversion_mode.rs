// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Valid internal states for Decimal conversion output policy.

use rust_decimal::RoundingStrategy;

/// Represents one valid final-output policy for a Decimal conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::measure) enum ConversionMode {
    /// Preserves the maximum precision produced by Decimal arithmetic.
    MaximumPrecision,

    /// Rounds the result to an exact Decimal scale.
    FixedScale {
        /// Number of decimal places retained in the output.
        scale: u32,

        /// Strategy used to round to `scale` decimal places.
        rounding: RoundingStrategy,
    },
}
