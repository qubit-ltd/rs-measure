// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Implementation dependencies used by exported declarative macros.

pub use rust_decimal;
pub use serde;
#[cfg(feature = "uom")]
pub use uom;

use rust_decimal::Decimal;
#[cfg(feature = "uom")]
use rust_decimal::prelude::{
    FromPrimitive,
    ToPrimitive,
};

/// Parses one decimal literal captured by an exported declarative macro in a
/// const context.
///
/// This hidden entry point keeps downstream macro expansions independent of a
/// direct `rust_decimal` dependency. It accepts the numeric literal forms that
/// the public macro's `$literal` grammar can pass to
/// `rust_decimal_macros::dec!`: decimal integers and fractions, scientific
/// notation, digit separators, and binary, octal, or hexadecimal integers.
///
/// # Arguments
///
/// * `value` - Stringified decimal literal to parse exactly.
///
/// # Returns
///
/// The exact Decimal represented by `value` without runtime parsing or
/// rounding.
///
/// # Panics
///
/// Panics during constant evaluation if `value` is not a supported numeric
/// literal or cannot be represented exactly by Decimal.
#[doc(hidden)]
pub const fn decimal_from_literal(value: &str) -> Decimal {
    const MAX_MANTISSA: u128 = (1_u128 << 96) - 1;

    let bytes = value.as_bytes();
    let length = bytes.len();
    let (negative, mut index) = parse_decimal_literal_sign(bytes);
    if index >= length {
        panic!("Decimal literal must contain digits");
    }

    let radix = if index + 1 < length && bytes[index] == b'0' {
        match bytes[index + 1] {
            b'b' => 2,
            b'o' => 8,
            b'x' => 16,
            _ => 10,
        }
    } else {
        10
    };

    if radix != 10 {
        index += 2;
        while index < length && bytes[index] == b'_' {
            index += 1;
        }
        let (mantissa, digits, end, overflowed) =
            parse_decimal_literal_digits(bytes, index, radix, 0);
        if digits == 0 || end != length || overflowed || mantissa > MAX_MANTISSA
        {
            panic!("invalid or unrepresentable radix Decimal literal");
        }
        return decimal_from_literal_parts(mantissa, 0, negative);
    }

    let (mut mantissa, integer_digits, mut end, mut overflowed) =
        parse_decimal_literal_digits(bytes, index, 10, 0);
    if integer_digits == 0 {
        panic!("Decimal literal must contain an integer part");
    }

    let mut exponent = 0_i32;
    if end < length && bytes[end] == b'.' {
        let fraction_start = end + 1;
        let (
            fractional_mantissa,
            fractional_digits,
            fraction_end,
            fraction_overflowed,
        ) = parse_decimal_literal_digits(bytes, fraction_start, 10, mantissa);
        mantissa = fractional_mantissa;
        end = fraction_end;
        overflowed = overflowed || fraction_overflowed;
        if fractional_digits == 0 && end < length {
            panic!(
                "Decimal literal fraction must contain digits before an exponent"
            );
        }
        if fractional_digits > i32::MAX as u32
            || overflowed
            || mantissa > MAX_MANTISSA
        {
            panic!("Decimal literal precision exceeds Decimal's exact range");
        }
        exponent = -(fractional_digits as i32);
    }

    if end < length && (bytes[end] == b'e' || bytes[end] == b'E') {
        end += 1;
        let mut exponent_is_negative = false;
        if end < length && (bytes[end] == b'+' || bytes[end] == b'-') {
            exponent_is_negative = bytes[end] == b'-';
            end += 1;
        }
        let (
            parsed_exponent,
            exponent_digits,
            exponent_end,
            exponent_overflowed,
        ) = parse_decimal_literal_digits(bytes, end, 10, 0);
        if exponent_digits == 0
            || exponent_overflowed
            || parsed_exponent > i32::MAX as u128
        {
            panic!("invalid Decimal literal exponent");
        }
        let parsed_exponent = parsed_exponent as i32;
        exponent = if exponent_is_negative {
            match exponent.checked_sub(parsed_exponent) {
                Some(value) => value,
                None => panic!("Decimal literal exponent is out of range"),
            }
        } else {
            match exponent.checked_add(parsed_exponent) {
                Some(value) => value,
                None => panic!("Decimal literal exponent is out of range"),
            }
        };
        end = exponent_end;
    }

    if end != length || overflowed {
        panic!("invalid Decimal literal");
    }
    if exponent > 28 || exponent < -28 {
        panic!("Decimal literal exponent exceeds Decimal's scale range");
    }
    if exponent > 0 {
        let mut remaining = exponent;
        while remaining > 0 {
            mantissa = match mantissa.checked_mul(10) {
                Some(value) => value,
                None => {
                    panic!("Decimal literal exceeds Decimal's mantissa range")
                }
            };
            remaining -= 1;
        }
        exponent = 0;
    }
    if mantissa > MAX_MANTISSA {
        panic!("Decimal literal exceeds Decimal's mantissa range");
    }
    decimal_from_literal_parts(mantissa, (-exponent) as u32, negative)
}

/// Parses an optional negative sign and following macro-stringification
/// whitespace.
///
/// # Arguments
///
/// * `bytes` - Stringified literal bytes.
///
/// # Returns
///
/// The sign flag and index of the first numeric byte.
const fn parse_decimal_literal_sign(bytes: &[u8]) -> (bool, usize) {
    if !bytes.is_empty() && bytes[0] == b'-' {
        let mut index = 1;
        while index < bytes.len()
            && matches!(bytes[index], b' ' | b'\t' | b'\n' | b'\r')
        {
            index += 1;
        }
        (true, index)
    } else {
        (false, 0)
    }
}

/// Accumulates literal digits until the first non-digit, non-separator byte.
///
/// # Arguments
///
/// * `bytes` - Complete stringified literal bytes.
/// * `index` - Index of the first digit to inspect.
/// * `radix` - Radix used to interpret each digit.
/// * `initial` - Mantissa accumulated by an earlier literal component.
///
/// # Returns
///
/// The mantissa, digit count, stopping index, and arithmetic-overflow flag.
const fn parse_decimal_literal_digits(
    bytes: &[u8],
    mut index: usize,
    radix: u32,
    mut initial: u128,
) -> (u128, u32, usize, bool) {
    let mut digits = 0_u32;
    let mut overflowed = false;
    while index < bytes.len() {
        let byte = bytes[index];
        if byte == b'_' && digits > 0 {
            index += 1;
            continue;
        }
        let Some(digit) = decimal_literal_digit(byte, radix) else {
            break;
        };
        initial = match initial.checked_mul(radix as u128) {
            Some(value) => match value.checked_add(digit as u128) {
                Some(value) => value,
                None => {
                    overflowed = true;
                    u128::MAX
                }
            },
            None => {
                overflowed = true;
                u128::MAX
            }
        };
        digits += 1;
        index += 1;
    }
    (initial, digits, index, overflowed)
}

/// Converts one ASCII byte to a digit for `radix`.
///
/// # Arguments
///
/// * `byte` - Candidate ASCII digit or letter.
/// * `radix` - Radix against which the digit is validated.
///
/// # Returns
///
/// The digit value when valid for `radix`; otherwise, `None`.
#[inline]
const fn decimal_literal_digit(byte: u8, radix: u32) -> Option<u32> {
    let digit = match byte {
        b'0'..=b'9' => (byte - b'0') as u32,
        b'a'..=b'z' => (byte - b'a') as u32 + 10,
        b'A'..=b'Z' => (byte - b'A') as u32 + 10,
        _ => return None,
    };
    if digit < radix { Some(digit) } else { None }
}

/// Constructs a Decimal from an exact parsed mantissa, scale, and sign.
///
/// # Arguments
///
/// * `mantissa` - Non-negative mantissa that fits Decimal's 96-bit range.
/// * `scale` - Decimal scale no greater than 28.
/// * `negative` - Whether a non-zero mantissa is negative.
///
/// # Returns
///
/// The exact Decimal represented by the supplied parts.
#[inline(always)]
const fn decimal_from_literal_parts(
    mantissa: u128,
    scale: u32,
    negative: bool,
) -> Decimal {
    Decimal::from_parts(
        mantissa as u32,
        (mantissa >> 32) as u32,
        (mantissa >> 64) as u32,
        negative && mantissa != 0,
        scale,
    )
}

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

#[cfg(feature = "uom")]
/// Converts Decimal into the finite `f64` storage used by `uom`.
///
/// # Arguments
///
/// * `value` - Finite Decimal value to approximate as `f64`.
///
/// # Returns
///
/// The nearest `f64` representation selected by `rust_decimal`.
///
/// # Panics
///
/// Panics if `rust_decimal` cannot represent the finite Decimal as `f64`.
#[must_use]
#[inline]
pub fn decimal_to_f64_approx(value: Decimal) -> f64 {
    value
        .to_f64()
        .expect("Decimal is finite and within the f64 exponent range")
}

#[cfg(feature = "uom")]
/// Converts an `f64` value from `uom` into Decimal.
///
/// # Arguments
///
/// * `value` - Approximate floating-point value to convert.
///
/// # Returns
///
/// A finite Decimal representation when available.
///
/// # Errors
///
/// Returns [`crate::MeasurementError::DecimalConversion`] for NaN, infinity,
/// or a finite value outside Decimal's representation.
#[inline]
pub fn decimal_from_f64_approx(
    value: f64,
) -> Result<Decimal, crate::MeasurementError> {
    Decimal::from_f64(value).ok_or_else(|| {
        crate::MeasurementError::DecimalConversion(value.to_string())
    })
}
