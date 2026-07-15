// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_measure::MeasurementError;

#[test]
fn test_measurement_error_display_includes_context() {
    let error = MeasurementError::UnknownUnit {
        quantity: "length".to_owned(),
        unit: "kg".to_owned(),
    };

    assert_eq!(error.to_string(), "unknown length unit: kg");
}

#[test]
fn test_measurement_error_new_variants_include_context() {
    let cases = [
        (
            MeasurementError::InvalidScale { scale: 29, max: 28 },
            "invalid Decimal scale 29; maximum is 28",
        ),
        (
            MeasurementError::ArithmeticOverflow {
                operation: "multiply conversion ratio",
            },
            "Decimal arithmetic overflow while multiply conversion ratio",
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
    ];

    for (error, expected) in cases {
        assert_eq!(error.to_string(), expected);
    }
}
