// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Checked coefficient and scale construction for Decimal literals.

use rust_decimal::Decimal;

/// Largest mantissa representable by Decimal's 96-bit coefficient.
pub(super) const MAX_MANTISSA: u128 = (1_u128 << 96) - 1;

/// Finalizes a decimal mantissa, exponent, and sign.
///
/// # Parameters
///
/// * `mantissa` - Parsed non-negative coefficient.
/// * `exponent` - Base-ten exponent after accounting for fractional digits.
/// * `trailing_zeroes` - Coefficient zeroes deferred during scanning.
/// * `negative` - Whether the original literal carried a negative sign.
///
/// # Returns
///
/// The exact Decimal represented by the supplied parts.
///
/// # Panics
///
/// Panics when the scale or expanded mantissa is outside Decimal's range.
pub(super) const fn finalize_decimal(
    mantissa: u128,
    exponent: i32,
    trailing_zeroes: u32,
    negative: bool,
) -> Decimal {
    let preferred_scale = if exponent < 0 {
        exponent.unsigned_abs()
    } else {
        0
    };
    let exponent = match exponent.checked_add_unsigned(trailing_zeroes) {
        Some(value) => value,
        None => panic!("Decimal literal exponent is out of range"),
    };
    match finalize_exact_decimal(
        mantissa,
        exponent as i64,
        preferred_scale,
        negative,
    ) {
        Some(value) => value,
        None => panic!("Decimal literal cannot be represented exactly"),
    }
}

/// Finalizes normalized base-ten components without rounding.
///
/// `preferred_scale` records the input scale before trailing-zero
/// normalization. The result restores as much of that scale as Decimal can
/// represent, while retaining every significant digit.
///
/// # Parameters
///
/// * `mantissa` - Normalized non-negative significant coefficient.
/// * `exponent` - Base-ten exponent applied to `mantissa`.
/// * `preferred_scale` - Largest input scale worth restoring.
/// * `negative` - Whether the original value carried a negative sign.
///
/// # Returns
///
/// The exact Decimal when its scale and coefficient fit; otherwise, `None`.
#[inline]
pub(crate) const fn finalize_exact_decimal(
    mut mantissa: u128,
    exponent: i64,
    preferred_scale: u32,
    negative: bool,
) -> Option<Decimal> {
    if mantissa == 0 {
        return Some(decimal_from_parts(
            0,
            if preferred_scale > Decimal::MAX_SCALE {
                Decimal::MAX_SCALE
            } else {
                preferred_scale
            },
            negative,
        ));
    }
    if exponent < -(Decimal::MAX_SCALE as i64)
        || exponent > Decimal::MAX_SCALE as i64
    {
        return None;
    }
    let mut scale = if exponent < 0 { (-exponent) as u32 } else { 0 };
    let mut remaining_exponent = if exponent > 0 { exponent as u32 } else { 0 };
    while remaining_exponent > 0 {
        mantissa = match mantissa.checked_mul(10) {
            Some(value) if value <= MAX_MANTISSA => value,
            _ => return None,
        };
        remaining_exponent -= 1;
    }
    if mantissa > MAX_MANTISSA {
        return None;
    }
    let target_scale = if preferred_scale > Decimal::MAX_SCALE {
        Decimal::MAX_SCALE
    } else {
        preferred_scale
    };
    while scale < target_scale {
        let Some(value) = mantissa.checked_mul(10) else {
            break;
        };
        if value > MAX_MANTISSA {
            break;
        }
        mantissa = value;
        scale += 1;
    }
    Some(decimal_from_parts(mantissa, scale, negative))
}

/// Constructs a Decimal from a parsed mantissa, scale, and sign.
///
/// # Parameters
///
/// * `mantissa` - Non-negative mantissa within Decimal's 96-bit range.
/// * `scale` - Decimal scale no greater than 28.
/// * `negative` - Whether a non-zero mantissa is negative.
///
/// # Returns
///
/// The exact Decimal represented by the supplied parts.
#[inline(always)]
pub(crate) const fn decimal_from_parts(
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
