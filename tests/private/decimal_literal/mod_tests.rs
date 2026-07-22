// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Stable numeric-literal contract tests for exported macros.

use proptest::{
    prop_assert_eq,
    proptest,
};
use qubit_measure::__private::decimal_from_literal;
use rust_decimal::{
    Decimal,
    dec,
};
use std::str::FromStr;

#[test]
fn test_decimal_from_literal_matches_supported_literal_subset() {
    macro_rules! assert_literal {
        ($literal:literal) => {
            assert_eq!(
                decimal_from_literal(std::hint::black_box(stringify!(
                    $literal
                ))),
                dec!($literal),
                stringify!($literal),
            );
        };
    }

    assert_literal!(0);
    assert_literal!(-0);
    assert_literal!(1);
    assert_literal!(-1_999);
    assert_literal!(1.);
    assert_literal!(-1.111_009);
    assert_literal!(79_228_162_514_264_337_593_543_950_335);
    assert_literal!(-79_228_162_514_264_337_593_543_950_335);
    assert_literal!(0b1);
    assert_literal!(-0b1_1111);
    assert_literal!(0o1_777);
    assert_literal!(-0x1_Ffff);
    assert_literal!(1.23e2);
    assert_literal!(-1.23e-2);
    assert_literal!(9.7E-7);
    assert_literal!(1.2345E-24);
    assert_literal!(0.000_000_000_000_000_000_000_000_000_01e1);
    assert_literal!(1E28);
}

#[test]
fn test_decimal_from_literal_rejects_invalid_or_unrepresentable_values() {
    for value in [
        "",
        ".1",
        "1.e2",
        "1abc",
        "1 e1",
        "1e 1",
        "1e29",
        "1e-29",
        "0xG",
        "79_228_162_514_264_337_593_543_950_336",
        "9.000_000_000_000_000_000_000_000_000_001",
    ] {
        assert!(
            std::panic::catch_unwind(|| {
                decimal_from_literal(std::hint::black_box(value));
            })
            .is_err(),
            "accepted invalid Decimal literal {value:?}",
        );
    }
}

#[test]
fn test_decimal_from_literal_reports_arithmetic_overflow_boundaries() {
    for (value, expected_message) in [
        (
            "340282366920938463463374607431768211456",
            "invalid Decimal literal",
        ),
        (
            "3402823669209384634633746074317682114550",
            "Decimal literal cannot be represented exactly",
        ),
        (
            "340282366920938463463374607431768211455e1",
            "Decimal literal cannot be represented exactly",
        ),
        (
            "1.00e-2147483647",
            "Decimal literal exponent is out of range",
        ),
    ] {
        let panic = std::panic::catch_unwind(|| {
            decimal_from_literal(std::hint::black_box(value));
        })
        .expect_err("overflowing Decimal literal should panic");
        let message = panic
            .downcast_ref::<&str>()
            .copied()
            .or_else(|| panic.downcast_ref::<String>().map(String::as_str))
            .expect("Decimal literal panic should contain a string message");
        assert_eq!(message, expected_message, "literal {value:?}");
    }
}

/// Verifies overflow handling in radix, fraction, and exponent scanning.
#[test]
fn test_decimal_from_literal_exercises_scanner_overflow_paths() {
    assert_eq!(decimal_from_literal("0x__1"), Decimal::ONE);

    for value in [
        "0.340282366920938463463374607431768211456",
        "34028236692093846346337460743176821145501",
        "0xfffffffffffffffffffffffffffffffffffff",
        "1e340282366920938463463374607431768211456",
        "1e3402823669209384634633746074317682114550",
    ] {
        assert!(
            std::panic::catch_unwind(|| {
                decimal_from_literal(std::hint::black_box(value));
            })
            .is_err(),
            "overflowing literal {value:?} should panic",
        );
    }
}

#[test]
fn test_decimal_from_literal_preserves_interleaved_zero_runs() {
    let expected = Decimal::from_str("100200300.004005000")
        .expect("expected Decimal should be representable");

    assert_eq!(decimal_from_literal("100200300.004005000"), expected);
}

proptest! {
    #[test]
    fn prop_decimal_from_literal_matches_exact_scientific_parts(
        mantissa in -1_000_000_000_000_i64..=1_000_000_000_000_i64,
        scale in 0_u32..=9,
    ) {
        let literal = format!("{mantissa}e-{scale}");
        let expected = Decimal::new(mantissa, scale);

        prop_assert_eq!(decimal_from_literal(&literal), expected);
    }
}
