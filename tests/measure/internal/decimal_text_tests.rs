// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Public-boundary tests for exact internal Decimal text parsing.

use qubit_measure::{
    Measurement,
    measurement,
    unit,
};
use rust_decimal::dec;

/// Verifies that exactly representable scientific text remains supported.
#[test]
fn test_decimal_text_accepts_exact_scientific_values() {
    assert_eq!(
        "1.25e2 m"
            .parse::<measurement::Length>()
            .expect("exact scientific value should parse"),
        Measurement::new(dec!(125), unit::Length::Meter),
    );
}
