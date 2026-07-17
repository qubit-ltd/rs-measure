// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Compile-time validation for generated unit-family metadata.

/// Validates metadata emitted by the unit-family macro at compile time.
///
/// # Arguments
///
/// * `quantity` - The persisted quantity identifier.
/// * `symbols` - Canonical symbols in variant order.
/// * `aliases` - All aliases in declaration order.
///
/// Canonical symbols and aliases are unique within their own sets. An alias
/// may equal a canonical symbol because canonical parsing has priority.
///
/// # Panics
///
/// Panics when the quantity is not non-empty ASCII `snake_case`, the family or
/// a symbol is empty, a symbol or alias contains surrounding Unicode
/// whitespace, canonical symbols repeat, or aliases are empty or repeat.
#[doc(hidden)]
pub const fn assert_unit_family_metadata(
    quantity: &str,
    symbols: &[&str],
    aliases: &[&str],
) {
    assert!(
        is_ascii_snake_case(quantity),
        "unit quantity must be non-empty ASCII snake_case",
    );
    assert!(!symbols.is_empty(), "unit family must not be empty");

    let mut index = 0;
    while index < symbols.len() {
        let symbol = symbols[index];
        assert!(
            !symbol.is_empty(),
            "canonical unit symbol must not be empty",
        );
        assert!(
            !has_leading_unit_whitespace(symbol)
                && !has_trailing_unit_whitespace(symbol),
            "canonical unit symbol must not contain surrounding whitespace",
        );
        let mut other = index + 1;
        while other < symbols.len() {
            assert!(
                !str_eq(symbol, symbols[other]),
                "canonical unit symbols must be unique",
            );
            other += 1;
        }
        index += 1;
    }

    index = 0;
    while index < aliases.len() {
        let alias = aliases[index];
        assert!(!alias.is_empty(), "unit alias must not be empty",);
        assert!(
            !has_leading_unit_whitespace(alias)
                && !has_trailing_unit_whitespace(alias),
            "unit alias must not contain surrounding whitespace",
        );
        let mut other = index + 1;
        while other < aliases.len() {
            assert!(
                !str_eq(alias, aliases[other]),
                "unit aliases must be unique",
            );
            other += 1;
        }
        index += 1;
    }
}

/// Reports whether text starts with a Unicode White_Space character.
///
/// # Arguments
///
/// * `value` - Text whose first scalar value is inspected.
///
/// # Returns
///
/// `true` when the first scalar has the Unicode White_Space property.
const fn has_leading_unit_whitespace(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.is_empty() {
        return false;
    }
    if matches!(bytes[0], b'\t'..=b'\r' | b' ') {
        return true;
    }
    if bytes.len() >= 2 && bytes[0] == 0xC2 && matches!(bytes[1], 0x85 | 0xA0) {
        return true;
    }
    if bytes.len() >= 3 {
        return (bytes[0] == 0xE1 && bytes[1] == 0x9A && bytes[2] == 0x80)
            || (bytes[0] == 0xE2
                && bytes[1] == 0x80
                && matches!(bytes[2], 0x80..=0x8A | 0xA8 | 0xA9 | 0xAF))
            || (bytes[0] == 0xE2 && bytes[1] == 0x81 && bytes[2] == 0x9F)
            || (bytes[0] == 0xE3 && bytes[1] == 0x80 && bytes[2] == 0x80);
    }
    false
}

/// Reports whether text ends with a Unicode White_Space character.
///
/// # Arguments
///
/// * `value` - Text whose final scalar value is inspected.
///
/// # Returns
///
/// `true` when the final scalar has the Unicode White_Space property.
const fn has_trailing_unit_whitespace(value: &str) -> bool {
    let bytes = value.as_bytes();
    let length = bytes.len();
    if length == 0 {
        return false;
    }
    if matches!(bytes[length - 1], b'\t'..=b'\r' | b' ') {
        return true;
    }
    if length >= 2
        && bytes[length - 2] == 0xC2
        && matches!(bytes[length - 1], 0x85 | 0xA0)
    {
        return true;
    }
    if length >= 3 {
        return (bytes[length - 3] == 0xE1
            && bytes[length - 2] == 0x9A
            && bytes[length - 1] == 0x80)
            || (bytes[length - 3] == 0xE2
                && bytes[length - 2] == 0x80
                && matches!(
                    bytes[length - 1],
                    0x80..=0x8A | 0xA8 | 0xA9 | 0xAF
                ))
            || (bytes[length - 3] == 0xE2
                && bytes[length - 2] == 0x81
                && bytes[length - 1] == 0x9F)
            || (bytes[length - 3] == 0xE3
                && bytes[length - 2] == 0x80
                && bytes[length - 1] == 0x80);
    }
    false
}

/// Reports whether a value is non-empty ASCII `snake_case`.
///
/// # Arguments
///
/// * `value` - The value to validate.
///
/// # Returns
///
/// `true` when the value starts with a lowercase ASCII letter, contains only
/// lowercase ASCII letters, digits, and single internal underscores, and does
/// not end with an underscore; otherwise, `false`.
#[doc(hidden)]
pub const fn is_ascii_snake_case(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.is_empty() || bytes[0] < b'a' || bytes[0] > b'z' {
        return false;
    }

    let mut index = 1;
    let mut previous_underscore = false;
    while index < bytes.len() {
        let byte = bytes[index];
        if byte == b'_' {
            if previous_underscore || index + 1 == bytes.len() {
                return false;
            }
            previous_underscore = true;
        } else if (byte >= b'a' && byte <= b'z')
            || (byte >= b'0' && byte <= b'9')
        {
            previous_underscore = false;
        } else {
            return false;
        }
        index += 1;
    }
    true
}

/// Compares two strings in const contexts.
///
/// # Arguments
///
/// * `lhs` - The first string.
/// * `rhs` - The second string.
///
/// # Returns
///
/// `true` when both strings contain the same bytes; otherwise, `false`.
const fn str_eq(lhs: &str, rhs: &str) -> bool {
    let lhs = lhs.as_bytes();
    let rhs = rhs.as_bytes();
    if lhs.len() != rhs.len() {
        return false;
    }

    let mut index = 0;
    while index < lhs.len() {
        if lhs[index] != rhs[index] {
            return false;
        }
        index += 1;
    }
    true
}
