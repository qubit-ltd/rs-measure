// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Minimum direct-dependency contract for qubit-measure.

use qubit_measure::ConversionFactor;
use rust_decimal::Decimal;

/// Builds a Decimal conversion factor through the public API.
///
/// # Returns
///
/// A valid identity conversion factor.
#[must_use]
pub fn build_identity_factor() -> ConversionFactor {
    ConversionFactor::from_decimal(Decimal::ONE)
        .expect("one should be a valid conversion factor")
}
