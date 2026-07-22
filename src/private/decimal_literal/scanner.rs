// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Syntax scanning for numeric literals captured by exported macros.

/// Parses an optional negative sign and macro-stringification whitespace.
///
/// # Parameters
///
/// * `bytes` - Stringified literal bytes.
///
/// # Returns
///
/// The sign flag and index of the first numeric byte.
pub(super) const fn parse_sign(bytes: &[u8]) -> (bool, usize) {
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

/// Detects a supported integer radix prefix.
///
/// # Parameters
///
/// * `bytes` - Complete stringified literal bytes.
/// * `index` - Index of the first numeric byte.
///
/// # Returns
///
/// Radix 2, 8, 16, or the default radix 10.
pub(super) const fn detect_radix(bytes: &[u8], index: usize) -> u32 {
    if index + 1 < bytes.len() && bytes[index] == b'0' {
        match bytes[index + 1] {
            b'b' => 2,
            b'o' => 8,
            b'x' => 16,
            _ => 10,
        }
    } else {
        10
    }
}

/// Accumulates literal digits until a non-digit, non-separator byte.
///
/// # Parameters
///
/// * `bytes` - Complete stringified literal bytes.
/// * `index` - Index of the first digit to inspect.
/// * `radix` - Radix used to interpret each digit.
/// * `initial` - Mantissa accumulated by an earlier literal component.
///
/// # Returns
///
/// The mantissa, digit count, stopping index, and arithmetic-overflow flag.
pub(super) const fn parse_digits(
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
        let Some(digit) = digit_value(byte, radix) else {
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

/// Accumulates decimal digits while deferring a trailing run of zeroes.
///
/// # Parameters
///
/// * `bytes` - Complete stringified literal bytes.
/// * `index` - Index of the first digit to inspect.
/// * `initial` - Significant mantissa accumulated by an earlier component.
/// * `pending_zeroes` - Deferred zeroes from an earlier component.
///
/// # Returns
///
/// The significant mantissa, deferred zero count, parsed digit count, stopping
/// index, and arithmetic-overflow flag.
pub(super) const fn parse_decimal_digits(
    bytes: &[u8],
    mut index: usize,
    mut initial: u128,
    mut pending_zeroes: u32,
) -> (u128, u32, u32, usize, bool) {
    let mut digits = 0_u32;
    let mut overflowed = false;
    while index < bytes.len() {
        let byte = bytes[index];
        if byte == b'_' && digits > 0 {
            index += 1;
            continue;
        }
        if !byte.is_ascii_digit() {
            break;
        }
        digits = digits.saturating_add(1);
        if byte == b'0' {
            pending_zeroes = pending_zeroes.saturating_add(1);
        } else {
            let (flushed, flush_overflowed) =
                flush_decimal_zeroes(initial, pending_zeroes, overflowed);
            initial = flushed;
            overflowed = flush_overflowed;
            pending_zeroes = 0;
            let (appended, append_overflowed) =
                append_decimal_digit(initial, byte - b'0', overflowed);
            initial = appended;
            overflowed = append_overflowed;
        }
        index += 1;
    }
    (initial, pending_zeroes, digits, index, overflowed)
}

/// Appends a run of decimal zeroes with saturating overflow reporting.
///
/// # Parameters
///
/// * `value` - Significant mantissa accumulated so far.
/// * `zeroes` - Number of decimal zeroes to append.
/// * `overflowed` - Whether an earlier accumulation already overflowed.
///
/// # Returns
///
/// The saturated mantissa and cumulative overflow flag.
const fn flush_decimal_zeroes(
    mut value: u128,
    mut zeroes: u32,
    mut overflowed: bool,
) -> (u128, bool) {
    while zeroes > 0 {
        let (multiplied, multiply_overflowed) =
            multiply_decimal(value, overflowed);
        value = multiplied;
        overflowed = multiply_overflowed;
        zeroes -= 1;
    }
    (value, overflowed)
}

/// Appends one non-zero decimal digit with saturating overflow reporting.
///
/// # Parameters
///
/// * `value` - Significant mantissa accumulated so far.
/// * `digit` - Decimal digit in the inclusive range `1..=9`.
/// * `overflowed` - Whether an earlier accumulation already overflowed.
///
/// # Returns
///
/// The saturated mantissa and cumulative overflow flag.
const fn append_decimal_digit(
    value: u128,
    digit: u8,
    overflowed: bool,
) -> (u128, bool) {
    let (multiplied, multiply_overflowed) = multiply_decimal(value, overflowed);
    match multiplied.checked_add(digit as u128) {
        Some(value) => (value, multiply_overflowed),
        None => (u128::MAX, true),
    }
}

/// Multiplies a decimal mantissa by ten with saturating overflow reporting.
///
/// # Parameters
///
/// * `value` - Mantissa to multiply.
/// * `overflowed` - Whether an earlier accumulation already overflowed.
///
/// # Returns
///
/// The saturated product and cumulative overflow flag.
const fn multiply_decimal(value: u128, overflowed: bool) -> (u128, bool) {
    match value.checked_mul(10) {
        Some(value) => (value, overflowed),
        None => (u128::MAX, true),
    }
}

/// Parses and combines a scientific-notation exponent.
///
/// # Parameters
///
/// * `bytes` - Complete stringified literal bytes.
/// * `index` - Offset immediately after the `e` or `E` marker.
/// * `current` - Exponent already contributed by fractional digits.
///
/// # Returns
///
/// The combined exponent and stopping offset.
///
/// # Panics
///
/// Panics for a missing, malformed, or out-of-range exponent.
pub(super) const fn parse_exponent(
    bytes: &[u8],
    mut index: usize,
    current: i32,
) -> (i32, usize) {
    let mut negative = false;
    if index < bytes.len() && (bytes[index] == b'+' || bytes[index] == b'-') {
        negative = bytes[index] == b'-';
        index += 1;
    }
    let (parsed, digits, end, overflowed) = parse_digits(bytes, index, 10, 0);
    if digits == 0 || overflowed || parsed > i32::MAX as u128 {
        panic!("invalid Decimal literal exponent");
    }
    let parsed = parsed as i32;
    let combined = if negative {
        match current.checked_sub(parsed) {
            Some(value) => value,
            None => panic!("Decimal literal exponent is out of range"),
        }
    } else {
        // `current` is zero or negative after scanning a Decimal coefficient,
        // so adding a non-negative `parsed` exponent cannot overflow `i32`.
        current + parsed
    };
    (combined, end)
}

/// Converts one ASCII byte to a digit for `radix`.
///
/// # Parameters
///
/// * `byte` - Candidate ASCII digit or letter.
/// * `radix` - Radix against which the digit is validated.
///
/// # Returns
///
/// The digit value when valid for `radix`; otherwise, `None`.
#[inline]
const fn digit_value(byte: u8, radix: u32) -> Option<u32> {
    let digit = match byte {
        b'0'..=b'9' => (byte - b'0') as u32,
        b'a'..=b'z' => (byte - b'a') as u32 + 10,
        b'A'..=b'Z' => (byte - b'A') as u32 + 10,
        _ => return None,
    };
    if digit < radix { Some(digit) } else { None }
}
