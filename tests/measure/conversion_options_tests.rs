// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_measure::ConversionOptions;
use qubit_measure::MeasurementError;
use rust_decimal::RoundingStrategy;

#[test]
fn test_maximum_precision_has_no_rounding_strategy() {
    let options = ConversionOptions::maximum_precision();

    assert_eq!(options.scale(), None);
    assert_eq!(options.rounding(), None);
    assert_eq!(options, ConversionOptions::DEFAULT);
}

#[test]
fn test_fixed_scale_exposes_its_rounding_strategy() {
    let options = ConversionOptions::fixed_scale(3, RoundingStrategy::ToZero)
        .expect("valid scale should produce conversion options");

    assert_eq!(options.scale(), Some(3));
    assert_eq!(options.rounding(), Some(RoundingStrategy::ToZero));
}

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
