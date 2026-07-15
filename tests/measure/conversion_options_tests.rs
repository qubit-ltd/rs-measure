// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_measure::{
    ConversionOptions,
    MeasurementError,
    RoundingStrategy,
};

#[test]
fn test_conversion_options_reject_scale_above_decimal_limit() {
    assert_eq!(
        ConversionOptions::fixed_scale(
            29,
            RoundingStrategy::MidpointNearestEven
        ),
        Err(MeasurementError::InvalidScale { scale: 29, max: 28 }),
    );
}
