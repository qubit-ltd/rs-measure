// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Opt-in compact measurement text Serde tests.

use qubit_measure::measurement;
use qubit_measure::measurement_text;
use qubit_measure::unit;
use rust_decimal::dec;
use serde::Deserialize;
use serde::Serialize;
use serde_json::from_value;
use serde_json::json;
use serde_json::to_value;

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
    #[serde(default, with = "measurement_text::option")]
    timeout: Option<measurement::Time>,
}

#[test]
fn test_measurement_text_serde_round_trip_uses_canonical_compact_string() {
    let config = InformationConfig {
        limit: measurement::Information::new(dec!(2), unit::Information::Mebibyte),
    };

    let value = to_value(&config).expect("compact measurement should serialize");
    assert_eq!(value, json!({"limit": "2 MiB"}));
    assert_eq!(
        from_value::<InformationConfig>(value)
            .expect("canonical compact measurement should deserialize"),
        config,
    );
}

#[test]
fn test_optional_measurement_text_serde_round_trip_uses_canonical_text() {
    let config = TimeConfig {
        timeout: Some(measurement::Time::new(dec!(2), unit::Time::Second)),
    };

    let value = to_value(&config).expect("optional compact measurement should serialize");
    assert_eq!(value, json!({"timeout": "2 s"}));
    assert_eq!(
        from_value::<TimeConfig>(value).expect("optional compact measurement should deserialize"),
        config,
    );
}

#[test]
fn test_optional_measurement_text_serde_accepts_null_and_missing_fields() {
    for value in [json!({"timeout": null}), json!({})] {
        assert_eq!(
            from_value::<TimeConfig>(value)
                .expect("null or missing optional measurement should deserialize"),
            TimeConfig { timeout: None },
        );
    }
    assert_eq!(
        to_value(TimeConfig { timeout: None })
            .expect("absent optional measurement should serialize"),
        json!({"timeout": null}),
    );
}

#[test]
fn test_measurement_text_serde_rejects_lenient_unit_aliases() {
    let error = from_value::<TimeConfig>(json!({
        "timeout": "1 year",
    }))
    .expect_err("compact adapter must reject non-canonical aliases");

    assert!(error.to_string().contains("non-canonical"));
}

#[test]
fn test_measurement_text_serde_rejects_non_string_values() {
    let error = from_value::<InformationConfig>(json!({
        "limit": 2,
    }))
    .expect_err("compact adapter must reject non-string values");

    assert!(error.to_string().contains("expected a string"));
}

#[test]
fn test_measurement_text_adapter_does_not_change_default_wire_format() {
    let measurement = measurement::Information::new(dec!(2), unit::Information::Mebibyte);

    assert_eq!(
        to_value(measurement).expect("default measurement should serialize"),
        json!({"quantity": "information", "value": "2", "unit": "MiB"}),
    );
}
