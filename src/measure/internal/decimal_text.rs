// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Lossless Decimal parsing at measurement text boundaries.

use rust_decimal::Decimal;
use serde::{
    Deserialize,
    Deserializer,
    de::Error,
};

/// Parses ordinary or scientific Decimal text without rounding.
///
/// # Parameters
///
/// * `value` - Candidate Decimal text.
///
/// # Returns
///
/// The exactly represented Decimal, or `None` when the syntax is invalid or
/// the value would require rounding or exceed Decimal's range.
#[inline]
pub(in crate::measure) fn parse_decimal_text_exact(
    value: &str,
) -> Option<Decimal> {
    let Some(exponent_index) = value.find(['e', 'E']) else {
        return Decimal::from_str_exact(value).ok();
    };

    Decimal::from_str_exact(&value[..exponent_index]).ok()?;
    Decimal::from_scientific(value).ok()
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
pub(super) fn deserialize_decimal_text_exact<'de, D>(
    deserializer: D,
) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    parse_decimal_text_exact(&value).ok_or_else(|| {
        D::Error::custom(format_args!("invalid Decimal value: {value}"))
    })
}
