/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_measure::MeasurementError;

#[test]
fn test_measurement_error_display_includes_context() {
    let error = MeasurementError::UnknownUnit {
        quantity: "length".to_owned(),
        unit: "kg".to_owned(),
    };

    assert_eq!(error.to_string(), "unknown length unit: kg");
}
