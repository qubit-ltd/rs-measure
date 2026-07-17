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
    mut mantissa: u128,
    mut exponent: i32,
    negative: bool,
) -> Decimal {
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
    decimal_from_parts(mantissa, (-exponent) as u32, negative)
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
pub(super) const fn decimal_from_parts(
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
