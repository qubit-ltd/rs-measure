// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_measure::ConversionOptions;
use qubit_measure::MeasurementError;
use qubit_measure::measurement;
use qubit_measure::unit;
use rust_decimal::RoundingStrategy;
use serde_json::json;

/// Demonstrates fixed-scale conversion, persistence, and strict parsing.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let original = measurement::Length::parse_strict("50.0 cm")?;
    // The meter value is derived; persistence keeps the user's original
    // centimeters.
    let meters = original.convert_to_with_options(
        unit::Length::Meter,
        ConversionOptions::fixed_scale(4, RoundingStrategy::MidpointNearestEven)?,
    )?;
    assert_eq!(meters.value.to_string(), "0.5000");

    let stored = serde_json::to_value(original)?;
    assert_eq!(stored, json!({"quantity":"length","value":"50.0","unit":"cm"}));
    let restored: measurement::Length = serde_json::from_value(stored)?;
    assert_eq!(restored, original);
    assert_eq!(restored.value.to_string(), "50.0");
    assert_eq!(restored.value.scale(), 1);

    match measurement::Time::parse_strict("1 year") {
        Err(MeasurementError::NonCanonicalUnit { canonical, .. }) => {
            println!("Please use the canonical time unit: {canonical}");
        }
        result => panic!("expected a non-canonical unit error, got {result:?}"),
    }

    println!("{original} = {meters}");
    Ok(())
}
