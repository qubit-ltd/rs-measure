// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Public behavior tests for the private measurement wire aggregation.

use qubit_measure::{
    measurement,
    unit,
};
use rust_decimal::dec;
use serde_json::json;

/// Verifies that Measurement Serde round-trips through its private wire type.
#[test]
fn test_measure_internal_wire_round_trips_public_measurement() {
    let value = measurement::Length::new(dec!(12.5), unit::Length::Centimeter);
    let json = serde_json::to_value(value)
        .expect("measurement should serialize through its wire type");

    assert_eq!(
        serde_json::from_value::<measurement::Length>(json.clone())
            .expect("measurement should deserialize through its wire type"),
        value,
    );
    assert_eq!(
        json,
        json!({"quantity": "length", "value": "12.5", "unit": "cm"}),
    );
}
