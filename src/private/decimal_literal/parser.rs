// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Top-level orchestration for exact Decimal literal parsing.

use rust_decimal::Decimal;

use super::coefficient::{
    MAX_MANTISSA,
    decimal_from_parts,
    finalize_decimal,
};
use super::scanner::{
    detect_radix,
    parse_digits,
    parse_exponent,
    parse_sign,
};

/// Parses one decimal literal captured by an exported macro in a const context.
///
/// The stable subset contains decimal integers and fractions with optional
/// separators, scientific notation, and binary, octal, or hexadecimal
/// integers. A leading unary minus is supported through stringification.
///
/// # Parameters
///
/// * `value` - Stringified numeric literal to parse exactly.
///
/// # Returns
///
/// The exact Decimal represented by `value` without rounding.
///
/// # Panics
///
/// Panics during constant evaluation for unsupported syntax or a value that
/// Decimal cannot represent exactly.
#[doc(hidden)]
pub const fn decimal_from_literal(value: &str) -> Decimal {
    let bytes = value.as_bytes();
    let length = bytes.len();
    let (negative, mut index) = parse_sign(bytes);
    if index >= length {
        panic!("Decimal literal must contain digits");
    }

    let radix = detect_radix(bytes, index);
    if radix != 10 {
        index += 2;
        while index < length && bytes[index] == b'_' {
            index += 1;
        }
        let (mantissa, digits, end, overflowed) =
            parse_digits(bytes, index, radix, 0);
        if digits == 0 || end != length || overflowed || mantissa > MAX_MANTISSA
        {
            panic!("invalid or unrepresentable radix Decimal literal");
        }
        return decimal_from_parts(mantissa, 0, negative);
    }

    let (mut mantissa, integer_digits, mut end, mut overflowed) =
        parse_digits(bytes, index, 10, 0);
    if integer_digits == 0 {
        panic!("Decimal literal must contain an integer part");
    }

    let mut exponent = 0_i32;
    if end < length && bytes[end] == b'.' {
        let (value, digits, fraction_end, fraction_overflowed) =
            parse_digits(bytes, end + 1, 10, mantissa);
        mantissa = value;
        end = fraction_end;
        overflowed = overflowed || fraction_overflowed;
        if digits == 0 && end < length {
            panic!(
                "Decimal literal fraction must contain digits before an exponent"
            );
        }
        if digits > i32::MAX as u32 || overflowed || mantissa > MAX_MANTISSA {
            panic!("Decimal literal precision exceeds Decimal's exact range");
        }
        exponent = -(digits as i32);
    }

    if end < length && (bytes[end] == b'e' || bytes[end] == b'E') {
        (exponent, end) = parse_exponent(bytes, end + 1, exponent);
    }
    if end != length || overflowed {
        panic!("invalid Decimal literal");
    }
    finalize_decimal(mantissa, exponent, negative)
}
