// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_measure::MeasurementError;
use rust_decimal::dec;

#[test]
fn test_measurement_error_display_includes_context() {
    let error = MeasurementError::UnknownUnit {
        quantity: "length".to_owned(),
        unit: "kg".to_owned(),
    };

    assert_eq!(error.to_string(), "unknown length unit: kg");
}

#[test]
fn test_ambiguous_measurement_error_lists_candidate_units() {
    let error = MeasurementError::AmbiguousMeasurement {
        input: "12x".to_owned(),
        units: vec!["x".to_owned(), "2x".to_owned()],
    };

    assert_eq!(
        error.to_string(),
        "ambiguous measurement 12x; matching units: x, 2x",
    );
}

#[test]
fn test_measurement_error_new_variants_include_context() {
    let cases = [
        (
            MeasurementError::MeasurementTextLimitExceeded { maximum: 3 },
            "measurement text exceeds the 3-byte limit",
        ),
        (
            MeasurementError::InvalidMeasurementSyntax,
            "invalid measurement syntax",
        ),
        (
            MeasurementError::UnrepresentableMeasurementValue,
            "measurement value cannot be represented exactly as Decimal",
        ),
        (
            MeasurementError::InvalidScale { scale: 29, max: 28 },
            "invalid Decimal scale 29; maximum is 28",
        ),
        (
            MeasurementError::ValueOutOfRange,
            "converted value is outside the Decimal range",
        ),
        (
            MeasurementError::OutputScaleUnrepresentable { scale: 4 },
            "converted value cannot retain Decimal scale 4",
        ),
        (
            MeasurementError::InvalidUnitDefinition {
                reason: "denominator must be positive".to_owned(),
            },
            "invalid unit definition: denominator must be positive",
        ),
        (
            MeasurementError::NonCanonicalUnit {
                quantity: "volume".to_owned(),
                unit: "gal".to_owned(),
                canonical: "gal (US)".to_owned(),
            },
            "non-canonical volume unit gal; use gal (US)",
        ),
        (
            MeasurementError::QuantityMismatch {
                expected: "length".to_owned(),
                actual: "mass".to_owned(),
            },
            "quantity mismatch: expected length, got mass",
        ),
        (
            MeasurementError::NegativeDuration {
                value: dec!(-1),
                unit: "s".to_owned(),
            },
            "negative duration: -1 s",
        ),
        (
            MeasurementError::SubnanosecondDuration {
                value: dec!(0.1),
                unit: "ns".to_owned(),
            },
            "duration has subnanosecond precision: 0.1 ns",
        ),
        (
            MeasurementError::DurationOutOfRange {
                value: dec!(1),
                unit: "a (365 d)".to_owned(),
            },
            "duration is out of range: 1 a (365 d)",
        ),
        (
            MeasurementError::NegativeInformation {
                value: dec!(-1),
                unit: "B".to_owned(),
            },
            "negative information size: -1 B",
        ),
        (
            MeasurementError::FractionalByteInformation {
                value: dec!(1),
                unit: "b".to_owned(),
            },
            "information size is not a whole number of bytes: 1 b",
        ),
        (
            MeasurementError::InformationOutOfRange {
                value: dec!(1),
                unit: "TiB".to_owned(),
                target: "usize",
            },
            "information size is out of range for usize: 1 TiB",
        ),
    ];

    for (error, expected) in cases {
        assert_eq!(error.to_string(), expected);
    }
}
