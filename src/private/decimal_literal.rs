// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact parsing for numeric literals captured by exported macros.

use rust_decimal::Decimal;

/// Parses one decimal literal captured by an exported declarative macro in a
/// const context.
///
/// This hidden entry point keeps downstream macro expansions independent of a
/// direct `rust_decimal` dependency. The crate-stable supported subset consists
/// of decimal integers and fractions with optional digit separators,
/// scientific notation, and binary, octal, or hexadecimal integers. A leading
/// unary minus is supported through macro stringification; a leading unary plus
/// is not a Rust literal token.
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
