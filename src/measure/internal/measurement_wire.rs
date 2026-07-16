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
    pub(in crate::measure) quantity: String,

    /// Exact decimal value encoded as a string.
    #[serde(with = "rust_decimal::serde::str")]
    pub(in crate::measure) value: Decimal,

    /// Canonical unit symbol or a documented input alias.
    pub(in crate::measure) unit: String,
}
