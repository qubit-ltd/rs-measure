// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Compile-time unit-metadata helper contract tests.

use qubit_measure::__private::assert_unit_family_metadata;
use qubit_measure::__private::is_ascii_snake_case;

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
#[should_panic(expected = "unit alias must not match any canonical symbol")]
fn test_assert_unit_family_metadata_rejects_canonical_alias_collision() {
    assert_unit_family_metadata("test_family", &["alias-owner", "canonical"], &["canonical"]);
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
    assert_unit_family_metadata("test_family", &["m", "cm"], &["meter", "meter"]);
}

/// Verifies that a unit cannot repeat its own canonical symbol as an alias.
#[test]
#[should_panic(expected = "unit alias must not match any canonical symbol")]
fn test_assert_unit_family_metadata_rejects_own_canonical_alias() {
    assert_unit_family_metadata("test_family", &["m"], &["m"]);
}

#[test]
fn test_assert_unit_family_metadata_rejects_surrounding_unicode_whitespace() {
    const WHITESPACE: &[char] = &[
        '\u{0009}', '\u{000A}', '\u{000B}', '\u{000C}', '\u{000D}', '\u{0020}', '\u{0085}',
        '\u{00A0}', '\u{1680}', '\u{2000}', '\u{2001}', '\u{2002}', '\u{2003}', '\u{2004}',
        '\u{2005}', '\u{2006}', '\u{2007}', '\u{2008}', '\u{2009}', '\u{200A}', '\u{2028}',
        '\u{2029}', '\u{202F}', '\u{205F}', '\u{3000}',
    ];

    for whitespace in WHITESPACE {
        for symbol in [format!("{whitespace}m"), format!("m{whitespace}")] {
            assert!(
                std::panic::catch_unwind(|| {
                    assert_unit_family_metadata("test_family", &[symbol.as_str()], &[]);
                })
                .is_err(),
                "accepted canonical symbol {symbol:?}",
            );
        }
        for alias in [format!("{whitespace}meter"), format!("meter{whitespace}")] {
            assert!(
                std::panic::catch_unwind(|| {
                    assert_unit_family_metadata("test_family", &["m"], &[alias.as_str()]);
                })
                .is_err(),
                "accepted alias {alias:?}",
            );
        }
    }
}
