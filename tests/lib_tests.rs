// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Root re-export contract tests.

use qubit_measure::{
    Measurement,
    MeasurementParseOptions,
    Unit,
    measurement,
    unit,
};
use rust_decimal::Decimal;

/// Verifies that the crate root exposes its core types and public modules.
#[test]
fn test_crate_root_reexports_measurement_and_unit_api() {
    let value: measurement::Length =
        Measurement::new(Decimal::ONE, unit::Length::Meter);

    assert_eq!(value.quantity_name(), unit::Length::QUANTITY);
    assert_eq!(
        MeasurementParseOptions::default().max_text_bytes(),
        MeasurementParseOptions::DEFAULT_MAX_TEXT_BYTES,
    );
}
