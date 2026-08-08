// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Independent exact-rational oracle for Decimal unit conversion tests.

use std::cmp::Ordering;

use num_bigint_04::BigInt;
use num_rational::BigRational;
use qubit_measure::UnitDefinition;
use rust_decimal::Decimal;
use rust_decimal::RoundingStrategy;

/// Converts a Decimal into its exact rational representation.
///
/// # Parameters
///
/// * `value` - Decimal value to represent independently.
///
/// # Returns
///
/// An exact rational with the Decimal mantissa over its power-of-ten scale.
pub(crate) fn decimal_as_rational(value: Decimal) -> BigRational {
    let denominator = BigInt::from(10_u8).pow(value.scale());
    BigRational::new(BigInt::from(value.mantissa()), denominator)
}

/// Computes the exact affine conversion independently of production ordering.
///
/// # Parameters
///
/// * `value` - Decimal value expressed by `source`.
/// * `source` - Source unit definition.
/// * `target` - Target unit definition.
///
/// # Returns
///
/// The exact mathematical result as a rational number.
pub(crate) fn expected_conversion(
    value: Decimal,
    source: UnitDefinition,
    target: UnitDefinition,
) -> BigRational {
    let source_factor = source.factor();
    let target_factor = target.factor();
    let source_ratio = decimal_as_rational(source_factor.numerator())
        / decimal_as_rational(source_factor.denominator());
    let target_ratio = decimal_as_rational(target_factor.numerator())
        / decimal_as_rational(target_factor.denominator());

    (decimal_as_rational(value) + decimal_as_rational(source.offset())) * source_ratio
        / target_ratio
        - decimal_as_rational(target.offset())
}

/// Rounds an exact rational to a Decimal with the requested fixed scale.
///
/// # Parameters
///
/// * `value` - Exact rational result to round.
/// * `scale` - Decimal places retained in the result.
/// * `strategy` - Direction and midpoint policy.
///
/// # Returns
///
/// The independently rounded Decimal result.
///
/// # Panics
///
/// Panics when the rounded mantissa does not fit Decimal or a deprecated
/// strategy is supplied by a test.
pub(crate) fn round_rational(
    value: &BigRational,
    scale: u32,
    strategy: RoundingStrategy,
) -> Decimal {
    let zero = BigInt::from(0_u8);
    let scaled_numerator = value.numer() * BigInt::from(10_u8).pow(scale);
    let denominator = value.denom();
    let quotient = &scaled_numerator / denominator;
    let remainder = &scaled_numerator % denominator;
    let negative = scaled_numerator < zero;
    let remainder_magnitude = if remainder < zero {
        -remainder
    } else {
        remainder.clone()
    };
    let midpoint_ordering = (&remainder_magnitude * 2_u8).cmp(denominator);
    let has_fraction = remainder_magnitude != zero;
    let direction = if negative { -1_i8 } else { 1_i8 };
    let increment = match strategy {
        RoundingStrategy::MidpointNearestEven => {
            let quotient_is_odd = (&quotient % 2_u8) != zero;
            (midpoint_ordering == Ordering::Greater
                || (midpoint_ordering == Ordering::Equal && quotient_is_odd))
                .then_some(direction)
        }
        RoundingStrategy::MidpointAwayFromZero => {
            (midpoint_ordering != Ordering::Less).then_some(direction)
        }
        RoundingStrategy::MidpointTowardZero => {
            (midpoint_ordering == Ordering::Greater).then_some(direction)
        }
        RoundingStrategy::ToZero => None,
        RoundingStrategy::AwayFromZero => has_fraction.then_some(direction),
        RoundingStrategy::ToNegativeInfinity => (negative && has_fraction).then_some(-1),
        RoundingStrategy::ToPositiveInfinity => (!negative && has_fraction).then_some(1),
        _ => panic!("deprecated rounding strategy is outside this oracle"),
    };
    let rounded = quotient + BigInt::from(increment.unwrap_or(0));
    let mantissa = i128::try_from(rounded).expect("rounded rational mantissa should fit i128");
    Decimal::try_from_i128_with_scale(mantissa, scale).expect("rounded rational should fit Decimal")
}
