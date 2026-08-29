// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! String-based persistence representation for measurements.

use rust_decimal::Decimal;
use serde::Deserialize;

/// String-based persistence representation owned by `Measurement`.
#[derive(Deserialize)]
pub(in crate::measure) struct MeasurementWire {
    /// Stable quantity identifier used to reject cross-quantity data.
    #[serde(deserialize_with = "super::decimal_text::deserialize_bounded_string")]
    pub(in crate::measure) quantity: String,

    /// Exact decimal value encoded as a string.
    #[serde(deserialize_with = "super::decimal_text::deserialize_decimal_text_exact")]
    pub(in crate::measure) value: Decimal,

    /// Canonical unit symbol accepted by strict unit parsing.
    #[serde(deserialize_with = "super::decimal_text::deserialize_bounded_string")]
    pub(in crate::measure) unit: String,
}
