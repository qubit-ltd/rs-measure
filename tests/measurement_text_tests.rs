// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Opt-in compact measurement text Serde tests.

use qubit_measure::{
    measurement,
    measurement_text,
    unit,
};
use rust_decimal::dec;
use serde::{
    Deserialize,
    Serialize,
};
use serde_json::json;

/// Configuration carrying an information limit as compact text.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct InformationConfig {
    /// Maximum accepted information size.
    #[serde(with = "measurement_text")]
    limit: measurement::Information,
}

/// Configuration carrying a time measurement as compact text.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct TimeConfig {
    /// Configured timeout.
    #[serde(with = "measurement_text")]
    timeout: measurement::Time,
}

#[test]
fn test_measurement_text_serde_round_trip_uses_canonical_compact_string() {
    let config = InformationConfig {
        limit: measurement::Information::new(
            dec!(2),
            unit::Information::Mebibyte,
        ),
    };

    let value = serde_json::to_value(&config)
        .expect("compact measurement should serialize");
    assert_eq!(value, json!({"limit": "2 MiB"}));
    assert_eq!(
        serde_json::from_value::<InformationConfig>(value)
            .expect("canonical compact measurement should deserialize"),
        config,
    );
}

#[test]
fn test_measurement_text_serde_rejects_lenient_unit_aliases() {
    let error = serde_json::from_value::<TimeConfig>(json!({
        "timeout": "1 year",
    }))
    .expect_err("compact adapter must reject non-canonical aliases");

    assert!(error.to_string().contains("non-canonical"));
}

#[test]
fn test_measurement_text_serde_rejects_non_string_values() {
    let error = serde_json::from_value::<InformationConfig>(json!({
        "limit": 2,
    }))
    .expect_err("compact adapter must reject non-string values");

    assert!(error.to_string().contains("expected a string"));
}

#[test]
fn test_measurement_text_adapter_does_not_change_default_wire_format() {
    let measurement =
        measurement::Information::new(dec!(2), unit::Information::Mebibyte);

    assert_eq!(
        serde_json::to_value(measurement)
            .expect("default measurement should serialize"),
        json!({"quantity": "information", "value": "2", "unit": "MiB"}),
    );
}
