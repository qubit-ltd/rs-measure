// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Lossless Decimal parsing at measurement text boundaries.

use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Deserializer;
use serde::de::Error;

use crate::measure::MeasurementError;
use crate::measure::MeasurementParseOptions;

/// Parses ordinary or scientific Decimal text without rounding.
///
/// # Parameters
///
/// * `value` - Candidate Decimal text.
///
/// # Returns
///
/// The exactly represented Decimal.
///
/// # Errors
///
/// Returns [`MeasurementError::InvalidMeasurementSyntax`] for invalid grammar
/// or [`MeasurementError::UnrepresentableMeasurementValue`] when exact Decimal
/// representation is impossible.
#[inline]
pub(in crate::measure) fn parse_decimal_text_exact(
    value: &str,
) -> Result<Decimal, MeasurementError> {
    let (base, scientific_exponent) =
        split_scientific_text(value).ok_or(MeasurementError::InvalidMeasurementSyntax)?;
    let (mantissa, trailing_zeroes, fraction_digits, negative) = parse_decimal_base(base)?;
    let raw_exponent =
        scientific_exponent.saturating_sub(i64::try_from(fraction_digits).unwrap_or(i64::MAX));
    let normalized_exponent =
        raw_exponent.saturating_add(i64::try_from(trailing_zeroes).unwrap_or(i64::MAX));
    let preferred_scale = negative_exponent_scale(raw_exponent);

    crate::__private::finalize_exact_decimal(
        mantissa,
        normalized_exponent,
        preferred_scale,
        negative,
    )
    .ok_or(MeasurementError::UnrepresentableMeasurementValue)
}

/// Splits Decimal text into its coefficient and scientific exponent.
///
/// # Parameters
///
/// * `value` - Candidate complete Decimal text.
///
/// # Returns
///
/// The base text and parsed exponent, or `None` for malformed notation.
#[inline]
fn split_scientific_text(value: &str) -> Option<(&str, i64)> {
    let mut exponent_index = None;
    for (index, byte) in value.bytes().enumerate() {
        if byte == b'e' || byte == b'E' {
            if exponent_index.is_some() {
                return None;
            }
            exponent_index = Some(index);
        }
    }
    match exponent_index {
        Some(index) => Some((
            &value[..index],
            parse_scientific_exponent(&value[index + 1..])?,
        )),
        None => Some((value, 0)),
    }
}

/// Parses a signed scientific exponent, saturating only beyond `i64` range.
///
/// # Parameters
///
/// * `value` - Text following the scientific-notation marker.
///
/// # Returns
///
/// The signed exponent, or `None` for invalid exponent syntax.
#[inline]
fn parse_scientific_exponent(value: &str) -> Option<i64> {
    let bytes = value.as_bytes();
    let (negative, digits) = match bytes.first() {
        Some(b'+') => (false, &bytes[1..]),
        Some(b'-') => (true, &bytes[1..]),
        _ => (false, bytes),
    };
    if digits.is_empty() || !digits.iter().all(u8::is_ascii_digit) {
        return None;
    }
    let mut exponent = 0_i64;
    for digit in digits {
        exponent = exponent
            .saturating_mul(10)
            .saturating_add(i64::from(digit - b'0'));
    }
    Some(if negative {
        exponent.saturating_neg()
    } else {
        exponent
    })
}

/// Parses the signed coefficient while deferring insignificant zeroes.
///
/// # Parameters
///
/// * `value` - Signed integer or fractional coefficient text.
///
/// # Returns
///
/// The significant mantissa, trailing-zero count, fractional-digit count, and
/// sign.
///
/// # Errors
///
/// Returns a classified syntax or exact-representation error.
fn parse_decimal_base(value: &str) -> Result<(u128, usize, usize, bool), MeasurementError> {
    let bytes = value.as_bytes();
    let (negative, start) = decimal_sign(bytes);
    let mut mantissa = 0_u128;
    let mut trailing_zeroes = 0_usize;
    let mut fraction_digits = 0_usize;
    let mut has_digit = false;
    let mut after_decimal_point = false;

    for &byte in &bytes[start..] {
        match byte {
            b'.' if !after_decimal_point => after_decimal_point = true,
            b'0' => {
                has_digit = true;
                fraction_digits += usize::from(after_decimal_point);
                trailing_zeroes += 1;
            }
            b'1'..=b'9' => {
                has_digit = true;
                fraction_digits += usize::from(after_decimal_point);
                mantissa = append_decimal_zeroes(mantissa, trailing_zeroes)?;
                trailing_zeroes = 0;
                mantissa = append_decimal_digit(mantissa, byte - b'0')?;
            }
            _ => return Err(MeasurementError::InvalidMeasurementSyntax),
        }
    }
    if !has_digit {
        return Err(MeasurementError::InvalidMeasurementSyntax);
    }
    Ok((mantissa, trailing_zeroes, fraction_digits, negative))
}

/// Parses an optional leading sign from a Decimal coefficient.
///
/// # Parameters
///
/// * `bytes` - Complete Decimal coefficient bytes.
///
/// # Returns
///
/// The negative-sign flag and index of the first unsigned coefficient byte.
#[inline(always)]
fn decimal_sign(bytes: &[u8]) -> (bool, usize) {
    match bytes.first() {
        Some(b'+') => (false, 1),
        Some(b'-') => (true, 1),
        _ => (false, 0),
    }
}

/// Appends deferred decimal zeroes using checked arithmetic.
///
/// # Parameters
///
/// * `value` - Significant mantissa accumulated so far.
/// * `zeroes` - Number of decimal zeroes to append.
///
/// # Returns
///
/// The mantissa after multiplying by ten once per deferred zero.
///
/// # Errors
///
/// Returns [`MeasurementError::UnrepresentableMeasurementValue`] when the
/// exact mantissa exceeds `u128`.
fn append_decimal_zeroes(mut value: u128, zeroes: usize) -> Result<u128, MeasurementError> {
    for _ in 0..zeroes {
        value = value
            .checked_mul(10)
            .ok_or(MeasurementError::UnrepresentableMeasurementValue)?;
    }
    Ok(value)
}

/// Appends one non-zero decimal digit using checked arithmetic.
///
/// # Parameters
///
/// * `value` - Significant mantissa accumulated so far.
/// * `digit` - Decimal digit in the inclusive range `1..=9`.
///
/// # Returns
///
/// The exact mantissa with `digit` appended.
///
/// # Errors
///
/// Returns [`MeasurementError::UnrepresentableMeasurementValue`] when the
/// exact mantissa exceeds `u128`.
#[inline]
fn append_decimal_digit(value: u128, digit: u8) -> Result<u128, MeasurementError> {
    value
        .checked_mul(10)
        .and_then(|value| value.checked_add(u128::from(digit)))
        .ok_or(MeasurementError::UnrepresentableMeasurementValue)
}

/// Converts a negative exponent into a bounded preferred Decimal scale.
///
/// # Parameters
///
/// * `exponent` - Raw base-ten exponent after fractional digits.
///
/// # Returns
///
/// Zero for a non-negative exponent, otherwise its magnitude capped at
/// `u32::MAX`.
#[inline]
fn negative_exponent_scale(exponent: i64) -> u32 {
    if exponent >= 0 {
        return 0;
    }
    u32::try_from(exponent.unsigned_abs()).unwrap_or(u32::MAX)
}

/// Deserializes a string containing an exactly representable Decimal.
///
/// # Parameters
///
/// * `deserializer` - Serde deserializer providing the Decimal string.
///
/// # Returns
///
/// The exactly represented Decimal.
///
/// # Errors
///
/// Returns the deserializer's error when the input is not a string, has
/// invalid Decimal syntax, or cannot be represented without rounding.
#[inline]
pub(super) fn deserialize_decimal_text_exact<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    let value = deserialize_bounded_string(deserializer)?;
    parse_decimal_text_exact(&value).map_err(D::Error::custom)
}

/// Deserializes a string subject to the default measurement text byte limit.
///
/// # Parameters
///
/// * `deserializer` - Serde deserializer providing the string.
///
/// # Returns
///
/// The owned string when its UTF-8 byte length is within the default limit.
///
/// # Errors
///
/// Returns the deserializer's error for a non-string value or a decoded string
/// that exceeds [`MeasurementParseOptions::DEFAULT_MAX_TEXT_BYTES`].
///
/// This function first asks Serde to construct an owned [`String`] and then
/// checks its byte length. The limit therefore controls field acceptance and
/// subsequent parsing work; it is neither a transport payload limit nor an
/// allocation limit enforced before deserialization.
#[inline]
pub(super) fn deserialize_bounded_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    if value.len() > MeasurementParseOptions::DEFAULT_MAX_TEXT_BYTES {
        return Err(D::Error::custom(
            MeasurementError::MeasurementTextLimitExceeded {
                maximum: MeasurementParseOptions::DEFAULT_MAX_TEXT_BYTES,
            },
        ));
    }
    Ok(value)
}
