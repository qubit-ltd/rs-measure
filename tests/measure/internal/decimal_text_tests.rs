// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Public-boundary tests for exact internal Decimal text parsing.

use std::str::FromStr;

use qubit_measure::Measurement;
use qubit_measure::measurement;
use qubit_measure::unit;
use rust_decimal::Decimal;
use rust_decimal::dec;

/// Verifies that exactly representable scientific text remains supported.
#[test]
fn test_decimal_text_accepts_exact_scientific_values() {
    assert_eq!(
        "1.25e2 m"
            .parse::<measurement::Length>()
            .expect("exact scientific value should parse"),
        Measurement::new(dec!(125), unit::Length::Meter),
    );
}

/// Verifies that scientific exponent and coefficient scale cancel exactly.
#[test]
fn test_decimal_text_accepts_scientific_scale_cancellation() {
    let large = Decimal::try_from_i128_with_scale(10_i128.pow(28), 0)
        .expect("ten to the twenty-eighth should fit Decimal");
    let cases = [
        ("1.0e-28 m", Decimal::new(1, 28), 28),
        ("100e-29 m", Decimal::new(10, 28), 28),
        ("0.1e29 m", large, 0),
    ];

    for (input, expected, expected_scale) in cases {
        let actual = input
            .parse::<measurement::Length>()
            .expect("exact scientific value should parse");

        assert_eq!(actual.value, expected, "input {input:?}");
        assert_eq!(actual.value.scale(), expected_scale, "input {input:?}");
    }
}

/// Verifies that scientific parsing retains an already representable scale.
#[test]
fn test_decimal_text_preserves_representable_scientific_scale() {
    let value = "1.00e0 m"
        .parse::<measurement::Length>()
        .expect("representable scale should parse")
        .value;

    assert_eq!(value, Decimal::new(100, 2));
    assert_eq!(value.scale(), 2);
}

/// Verifies that zero remains representable across extreme exponents.
#[test]
fn test_decimal_text_accepts_zero_with_extreme_exponents() {
    let tiny = "0e-999999999999999999999999999999999999999999 m"
        .parse::<measurement::Length>()
        .expect("zero with a negative exponent should parse")
        .value;
    let large = "0e999999999999999999999999999999999999999999 m"
        .parse::<measurement::Length>()
        .expect("zero with a positive exponent should parse")
        .value;

    assert_eq!(tiny, Decimal::new(0, 28));
    assert_eq!(tiny.scale(), 28);
    assert_eq!(large, Decimal::ZERO);
}

/// Verifies that significant digits separated by zero runs remain exact.
#[test]
fn test_decimal_text_preserves_interleaved_and_trailing_zero_runs() {
    let expected = Decimal::from_str("100200300.004005000")
        .expect("expected Decimal should be representable");
    let actual = "100200300.004005000 m"
        .parse::<measurement::Length>()
        .expect("zero runs should parse exactly")
        .value;

    assert_eq!(actual, expected);
    assert_eq!(actual.scale(), 9);
}
