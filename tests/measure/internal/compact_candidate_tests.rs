// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Public behavior tests for compact measurement candidate resolution.

use std::str::FromStr;

use qubit_measure::{
    Measurement,
    MeasurementError,
};

use crate::measure::fixtures::CompactAmbiguityUnit;

/// Verifies that multiple valid compact suffixes remain an explicit error.
#[test]
fn test_compact_candidate_reports_ambiguous_unit_suffix() {
    type CompactMeasurement = Measurement<CompactAmbiguityUnit>;

    assert_eq!(
        CompactMeasurement::from_str("12x"),
        Err(MeasurementError::AmbiguousMeasurement {
            input: "12x".to_owned(),
            units: vec!["x".to_owned(), "2x".to_owned()],
        }),
    );
    assert_eq!(
        CompactMeasurement::from_str("112x"),
        Err(MeasurementError::AmbiguousMeasurement {
            input: "112x".to_owned(),
            units: vec!["x".to_owned(), "2x".to_owned(), "12x".to_owned()],
        }),
    );
}
