// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Public Serde contract tests for the private measurement wire type.

use qubit_measure::{
    MeasurementParseOptions,
    measurement,
};
use serde_json::json;

/// Verifies that every persisted measurement field remains required.
#[test]
fn test_measurement_wire_rejects_missing_required_fields() {
    let cases = [
        (
            json!({"value": "1", "unit": "m"}),
            "missing field `quantity`",
        ),
        (
            json!({"quantity": "length", "unit": "m"}),
            "missing field `value`",
        ),
        (
            json!({"quantity": "length", "value": "1"}),
            "missing field `unit`",
        ),
    ];

    for (value, expected_message) in cases {
        let error = serde_json::from_value::<measurement::Length>(value)
            .expect_err("missing required field should fail");

        assert!(
            error.to_string().contains(expected_message),
            "unexpected missing-field error: {error}",
        );
    }
}

/// Verifies that Decimal values remain string-only in JSON.
#[test]
fn test_measurement_wire_rejects_numeric_decimal_value() {
    let error = serde_json::from_value::<measurement::Length>(json!({
        "quantity": "length",
        "value": 1,
        "unit": "m",
    }))
    .expect_err("numeric Decimal value should fail");
    let message = error.to_string();

    assert!(
        message.contains("invalid type: integer"),
        "unexpected error: {error}",
    );
    assert!(
        message.contains("expected a string"),
        "unexpected error: {error}",
    );
}

/// Verifies that persisted Decimal text is never rounded during decoding.
#[test]
fn test_measurement_wire_rejects_lossy_decimal_text() {
    for value in ["9.000000000000000000000000000001", "2.5e-28"] {
        let error = serde_json::from_value::<measurement::Length>(json!({
            "quantity": "length",
            "value": value,
            "unit": "m",
        }))
        .expect_err("lossy Decimal text should fail");

        assert!(
            error
                .to_string()
                .contains("cannot be represented exactly as Decimal"),
            "unexpected error for {value:?}: {error}",
        );
    }
}

/// Verifies that malformed Decimal grammar is classified during wire decoding.
#[test]
fn test_measurement_wire_rejects_malformed_decimal_text() {
    for value in ["1e2e3", "1e+", "+", "1..0"] {
        let error = serde_json::from_value::<measurement::Length>(json!({
            "quantity": "length",
            "value": value,
            "unit": "m",
        }))
        .expect_err("malformed Decimal text should fail");

        assert!(
            error.to_string().contains("invalid measurement syntax"),
            "unexpected error for {value:?}: {error}",
        );
    }
}

/// Verifies that every persisted string field uses the default byte limit.
#[test]
fn test_measurement_wire_rejects_oversized_string_fields() {
    let oversized =
        "x".repeat(MeasurementParseOptions::DEFAULT_MAX_TEXT_BYTES + 1);
    let values = [
        json!({
            "quantity": oversized.clone(),
            "value": "1",
            "unit": "m",
        }),
        json!({
            "quantity": "length",
            "value": oversized.clone(),
            "unit": "m",
        }),
        json!({
            "quantity": "length",
            "value": "1",
            "unit": oversized,
        }),
    ];

    for value in values {
        let error = serde_json::from_value::<measurement::Length>(value)
            .expect_err("oversized wire field should fail");

        assert!(
            error.to_string().contains("byte limit"),
            "unexpected oversized-field error: {error}",
        );
    }
}

/// Verifies that wire decoding accepts exact scientific boundary values.
#[test]
fn test_measurement_wire_accepts_exact_scientific_boundaries() {
    for value in ["1.0e-28", "100e-29", "0.1e29"] {
        let _ = serde_json::from_value::<measurement::Length>(json!({
            "quantity": "length",
            "value": value,
            "unit": "m",
        }))
        .expect("exact scientific wire value should parse");
    }
}

/// Verifies that unknown units retain their quantity context.
#[test]
fn test_measurement_wire_rejects_unknown_unit_with_quantity_context() {
    let error = serde_json::from_value::<measurement::Length>(json!({
        "quantity": "length",
        "value": "1",
        "unit": "kg",
    }))
    .expect_err("unknown length unit should fail");

    assert!(error.to_string().contains("unknown length unit: kg"));
}

/// Verifies that future JSON fields remain forward-compatible.
#[test]
fn test_measurement_wire_ignores_additional_fields() {
    let value = serde_json::from_value::<measurement::Length>(json!({
        "quantity": "length",
        "value": "1",
        "unit": "m",
        "future": {"version": 2},
    }))
    .expect("additional fields should be ignored");

    assert_eq!(value.value.to_string(), "1");
}

/// Verifies that quantity mismatches identify expected and actual families.
#[test]
fn test_measurement_wire_rejects_quantity_mismatch_with_context() {
    let error = serde_json::from_value::<measurement::Length>(json!({
        "quantity": "mass",
        "value": "1",
        "unit": "m",
    }))
    .expect_err("mismatched quantity should fail");

    assert!(
        error
            .to_string()
            .contains("quantity mismatch: expected length, got mass"),
    );
}

/// Verifies that persisted unit aliases are rejected as non-canonical.
#[test]
fn test_measurement_wire_rejects_alias() {
    let error = serde_json::from_value::<measurement::Time>(json!({
        "quantity": "time",
        "value": "1",
        "unit": "year",
    }))
    .expect_err("persisted aliases must be rejected");

    assert!(error.to_string().contains("non-canonical"));
}
