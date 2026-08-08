// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_measure::ConversionOptions;
use qubit_measure::measurement;
use qubit_measure::unit;
use rust_decimal::Decimal;
use rust_decimal::RoundingStrategy;
use serde_json::json;

/// Demonstrates exact conversion and persistence.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let centimeters = measurement::Length::new(
        Decimal::new(500, 1),
        unit::Length::Centimeter,
    );
    let meters = centimeters.convert_to_with_options(
        unit::Length::Meter,
        ConversionOptions::fixed_scale(
            4,
            RoundingStrategy::MidpointNearestEven,
        )?,
    )?;
    let json_value = serde_json::to_value(centimeters)?;

    assert_eq!(meters.value.to_string(), "0.5000");
    assert_eq!(
        json_value,
        json!({"quantity": "length", "value": "50.0", "unit": "cm"}),
    );
    println!("{centimeters} = {meters}");
    Ok(())
}
