// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Public behavior tests for private conversion-policy states.

use qubit_measure::{
    ConversionOptions,
    RoundingStrategy,
};

#[test]
fn test_conversion_modes_expose_only_semantically_valid_state() {
    let maximum = ConversionOptions::maximum_precision();
    let fixed = ConversionOptions::fixed_scale(2, RoundingStrategy::ToZero)
        .expect("scale should be valid");

    assert_eq!((maximum.scale(), maximum.rounding()), (None, None));
    assert_eq!(
        (fixed.scale(), fixed.rounding()),
        (Some(2), Some(RoundingStrategy::ToZero)),
    );
}
