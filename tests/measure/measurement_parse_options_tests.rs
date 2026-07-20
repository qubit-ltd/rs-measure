// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_measure::{
    MeasurementError,
    MeasurementParseOptions,
    measurement,
    unit,
};
use rust_decimal::dec;

/// Verifies the stable default measurement text limit.
#[test]
fn test_measurement_parse_options_exposes_default_limit() {
    let options = MeasurementParseOptions::default();

    assert_eq!(
        options.max_text_bytes(),
        MeasurementParseOptions::DEFAULT_MAX_TEXT_BYTES,
    );
}

/// Verifies that explicit parsing options are enforced before parsing.
#[test]
fn test_measurement_parse_with_options_enforces_byte_limit() {
    let options = MeasurementParseOptions::default().with_max_text_bytes(3);

    assert_eq!(
        measurement::Length::parse_strict_with_options("1 m", &options),
        Ok(measurement::Length::new(dec!(1), unit::Length::Meter)),
    );
    assert_eq!(
        measurement::Length::parse_strict_with_options("10 m", &options),
        Err(MeasurementError::MeasurementTextLimitExceeded { maximum: 3 }),
    );
    assert_eq!(
        measurement::Length::parse_lenient_with_options("10 m", &options),
        Err(MeasurementError::MeasurementTextLimitExceeded { maximum: 3 }),
    );
}

/// Verifies that default parsing rejects oversized input before scanning it.
#[test]
fn test_measurement_default_parse_limit_is_enforced() {
    let input = "1".repeat(MeasurementParseOptions::DEFAULT_MAX_TEXT_BYTES + 1);

    assert_eq!(
        measurement::Length::parse_strict(&input),
        Err(MeasurementError::MeasurementTextLimitExceeded {
            maximum: MeasurementParseOptions::DEFAULT_MAX_TEXT_BYTES,
        }),
    );
}

/// Verifies that syntax and exact-representation failures remain distinct.
#[test]
fn test_measurement_parsing_classifies_decimal_errors() {
    assert_eq!(
        measurement::Length::parse_strict("not-a-value m"),
        Err(MeasurementError::InvalidMeasurementSyntax),
    );
    assert_eq!(
        measurement::Length::parse_strict("2.5e-28 m"),
        Err(MeasurementError::UnrepresentableMeasurementValue),
    );
    assert_eq!(
        measurement::Length::parse_strict(
            "9999999999999999999999999999999999999999 m",
        ),
        Err(MeasurementError::UnrepresentableMeasurementValue),
    );
    assert_eq!(
        measurement::Length::parse_strict("1e+ m"),
        Err(MeasurementError::InvalidMeasurementSyntax),
    );
}
