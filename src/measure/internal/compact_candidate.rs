// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! One valid interpretation of a compact measurement suffix.

use rust_decimal::Decimal;

/// One valid interpretation of a compact measurement suffix.
#[derive(Clone, Copy)]
pub(super) struct CompactCandidate {
    /// Exact Decimal value preceding the matched suffix.
    pub(super) value: Decimal,

    /// Index of the matched unit in [`Unit::all`](crate::Unit::all).
    pub(super) unit_index: usize,

    /// Canonical symbol or alias that matched the input suffix.
    pub(super) symbol: &'static str,
}
