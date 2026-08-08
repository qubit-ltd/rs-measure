// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Public boundary tests for internal measurement-text splitting paths.

use std::str::FromStr;

use qubit_measure::MeasurementError;
use qubit_measure::measurement;
use qubit_measure::unit;
use rust_decimal::dec;

#[test]
fn test_measurement_text_handles_signed_fractional_and_exponent_boundaries() {
    for (input, expected) in [
        (
            "  +1.25e+2   m  ",
            measurement::Length::new(dec!(125), unit::Length::Meter),
        ),
        (
            "-.5 m",
            measurement::Length::new(dec!(-0.5), unit::Length::Meter),
        ),
        (
            ".5\tm",
            measurement::Length::new(dec!(0.5), unit::Length::Meter),
        ),
    ] {
        assert_eq!(measurement::Length::from_str(input), Ok(expected));
    }
}

#[test]
fn test_measurement_text_accepts_strict_canonical_length() {
    assert_eq!(
        measurement::Length::parse_strict("1 m"),
        Ok(measurement::Length::new(dec!(1), unit::Length::Meter)),
    );
}

#[test]
fn test_measurement_text_reports_unknown_unit_in_lenient_mode() {
    assert_eq!(
        measurement::Length::parse_lenient("1 bogus"),
        Err(MeasurementError::UnknownUnit {
            quantity: "length".to_owned(),
            unit: "bogus".to_owned(),
        }),
    );
}

#[test]
fn test_measurement_text_rejects_incomplete_numeric_boundaries() {
    for input in ["", " ", "+", "-", ".", "+.", "-.", "1 ", "1\t", "1.m"] {
        assert_eq!(
            measurement::Length::from_str(input),
            Err(MeasurementError::InvalidMeasurementSyntax),
            "input={input:?}",
        );
    }
    assert_eq!(
        measurement::Length::from_str("1e999 m"),
        Err(MeasurementError::UnrepresentableMeasurementValue),
    );
}
