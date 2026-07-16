// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Hidden macro-helper contract tests.

use qubit_measure::__private::{
    assert_unit_family_metadata,
    decimal_from_literal,
    is_ascii_snake_case,
};
use rust_decimal::dec;

#[test]
fn test_decimal_from_literal_matches_dec_macro_grammar() {
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
            "invalid Decimal literal",
        ),
        (
            "340282366920938463463374607431768211455e1",
            "Decimal literal exceeds Decimal's mantissa range",
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

#[test]
fn test_is_ascii_snake_case_accepts_valid_identifiers() {
    assert!(is_ascii_snake_case("a"));
    assert!(is_ascii_snake_case("unit_family_2"));
}

#[test]
fn test_is_ascii_snake_case_rejects_invalid_identifiers() {
    for value in [
        "",
        "1unit",
        "Unit",
        "_unit",
        "unit__family",
        "unit_",
        "unit-family",
        "unitFamily",
        "单位",
    ] {
        assert!(!is_ascii_snake_case(value), "accepted {value:?}");
    }
}

#[test]
fn test_assert_unit_family_metadata_accepts_valid_metadata() {
    assert_unit_family_metadata(
        "test_family_2",
        &["m", "mm", "cm"],
        &["meter", "metre", "centimeter"],
    );
}

#[test]
#[should_panic(expected = "ASCII snake_case")]
fn test_assert_unit_family_metadata_rejects_invalid_quantity() {
    assert_unit_family_metadata("TestFamily", &["m"], &[]);
}

#[test]
#[should_panic(expected = "unit family must not be empty")]
fn test_assert_unit_family_metadata_rejects_empty_family() {
    assert_unit_family_metadata("test_family", &[], &[]);
}

#[test]
#[should_panic(expected = "canonical unit symbol must not be empty")]
fn test_assert_unit_family_metadata_rejects_empty_symbol() {
    assert_unit_family_metadata("test_family", &[""], &[]);
}

#[test]
#[should_panic(expected = "canonical unit symbols must be unique")]
fn test_assert_unit_family_metadata_rejects_duplicate_symbol() {
    assert_unit_family_metadata("test_family", &["m", "m"], &[]);
}

#[test]
#[should_panic(expected = "unit alias must not be empty")]
fn test_assert_unit_family_metadata_rejects_empty_alias() {
    assert_unit_family_metadata("test_family", &["m"], &[""]);
}

#[test]
#[should_panic(expected = "unit aliases must be unique")]
fn test_assert_unit_family_metadata_rejects_duplicate_alias() {
    assert_unit_family_metadata("test_family", &["m"], &["meter", "meter"]);
}

#[test]
fn test_assert_unit_family_metadata_rejects_surrounding_unicode_whitespace() {
    const WHITESPACE: &[char] = &[
        '\u{0009}', '\u{000A}', '\u{000B}', '\u{000C}', '\u{000D}', '\u{0020}',
        '\u{0085}', '\u{00A0}', '\u{1680}', '\u{2000}', '\u{2001}', '\u{2002}',
        '\u{2003}', '\u{2004}', '\u{2005}', '\u{2006}', '\u{2007}', '\u{2008}',
        '\u{2009}', '\u{200A}', '\u{2028}', '\u{2029}', '\u{202F}', '\u{205F}',
        '\u{3000}',
    ];

    for whitespace in WHITESPACE {
        for symbol in [format!("{whitespace}m"), format!("m{whitespace}")] {
            assert!(
                std::panic::catch_unwind(|| {
                    assert_unit_family_metadata(
                        "test_family",
                        &[symbol.as_str()],
                        &[],
                    );
                })
                .is_err(),
                "accepted canonical symbol {symbol:?}",
            );
        }
        for alias in
            [format!("{whitespace}meter"), format!("meter{whitespace}")]
        {
            assert!(
                std::panic::catch_unwind(|| {
                    assert_unit_family_metadata(
                        "test_family",
                        &["m"],
                        &[alias.as_str()],
                    );
                })
                .is_err(),
                "accepted alias {alias:?}",
            );
        }
    }
}
