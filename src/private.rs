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

#[cfg(feature = "uom")]
use rust_decimal::Decimal;
#[cfg(feature = "uom")]
use rust_decimal::prelude::{
    FromPrimitive,
    ToPrimitive,
};

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
/// a symbol is empty, canonical symbols repeat, or aliases are empty or repeat.
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
        assert!(
            !symbols[index].is_empty(),
            "canonical unit symbol must not be empty",
        );
        let mut other = index + 1;
        while other < symbols.len() {
            assert!(
                !str_eq(symbols[index], symbols[other]),
                "canonical unit symbols must be unique",
            );
            other += 1;
        }
        index += 1;
    }

    index = 0;
    while index < aliases.len() {
        assert!(!aliases[index].is_empty(), "unit alias must not be empty",);
        let mut other = index + 1;
        while other < aliases.len() {
            assert!(
                !str_eq(aliases[index], aliases[other]),
                "unit aliases must be unique",
            );
            other += 1;
        }
        index += 1;
    }
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
