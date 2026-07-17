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
        match current.checked_add(parsed) {
            Some(value) => value,
            None => panic!("Decimal literal exponent is out of range"),
        }
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
