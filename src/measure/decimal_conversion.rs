// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Decimal-only conversion arithmetic.

use std::cmp::Ordering;

use num_bigint::BigInt;
use num_rational::BigRational;
use rust_decimal::{
    Decimal,
    RoundingStrategy,
};

use crate::measure::conversion_factor::reduce_ratio_terms;
use crate::measure::internal::ConversionMode;
use crate::measure::{
    ConversionOptions,
    MeasurementError,
    UnitDefinition,
};

/// Converts a value between validated unit definitions without using floats.
///
/// # Arguments
///
/// * `value` - Decimal value expressed by `source`.
/// * `source` - Definition of the input unit.
/// * `target` - Definition of the requested output unit.
/// * `options` - Final scale and rounding configuration.
///
/// # Returns
///
/// The converted Decimal value expressed by `target`.
///
/// # Errors
///
/// Returns [`MeasurementError::ArithmeticOverflow`] if the exact result or
/// requested final scale is outside the Decimal representation.
pub(crate) fn convert_decimal(
    value: Decimal,
    source: UnitDefinition,
    target: UnitDefinition,
    options: ConversionOptions,
) -> Result<Decimal, MeasurementError> {
    if source == target {
        return apply_output_scale(value, options);
    }

    let adjusted =
        decimal_as_rational(value) + decimal_as_rational(source.offset());
    let source_factor = source.factor();
    let target_factor = target.factor();
    let scaled = adjusted * decimal_as_rational(source_factor.numerator())
        / decimal_as_rational(source_factor.denominator())
        * decimal_as_rational(target_factor.denominator())
        / decimal_as_rational(target_factor.numerator());
    let exact = &scaled - decimal_as_rational(target.offset());

    match options.mode() {
        ConversionMode::MaximumPrecision => maximum_precision_decimal(&exact)
            .ok_or_else(|| MeasurementError::ArithmeticOverflow {
                operation: maximum_precision_overflow_operation(
                    value, source, target, &scaled,
                ),
            }),
        ConversionMode::FixedScale { scale, rounding } => {
            fixed_scale_decimal(&exact, scale, rounding).ok_or_else(|| {
                let operation = if maximum_precision_decimal(&exact).is_none() {
                    maximum_precision_overflow_operation(
                        value, source, target, &scaled,
                    )
                } else {
                    "set output scale"
                };
                MeasurementError::ArithmeticOverflow { operation }
            })
        }
    }
}

/// Converts a Decimal to its exact rational representation.
///
/// # Parameters
///
/// * `value` - Decimal value to represent without rounding.
///
/// # Returns
///
/// The signed mantissa divided by the power of ten selected by its scale.
fn decimal_as_rational(value: Decimal) -> BigRational {
    BigRational::new(
        BigInt::from(value.mantissa()),
        BigInt::from(10_u8).pow(value.scale()),
    )
}

/// Multiplies two positive ratio terms only when no rounding is required.
///
/// # Arguments
///
/// * `lhs` - The first positive Decimal term.
/// * `rhs` - The second positive Decimal term.
///
/// # Returns
///
/// The exact product when its normalized mantissas and combined scale fit in
/// Decimal, or `None` when multiplication would overflow or require rounding.
#[inline]
fn checked_mul_exact(lhs: Decimal, rhs: Decimal) -> Option<Decimal> {
    let lhs = lhs.normalize();
    let rhs = rhs.normalize();
    let scale = lhs.scale() + rhs.scale();
    if scale > Decimal::MAX_SCALE {
        return None;
    }
    let mantissa = lhs.mantissa().checked_mul(rhs.mantissa())?;
    Decimal::try_from_i128_with_scale(mantissa, scale).ok()
}

/// Selects the legacy operation label for an unrepresentable exact result.
///
/// # Arguments
///
/// * `value` - Original Decimal value.
/// * `source` - Source unit definition.
/// * `target` - Target unit definition.
/// * `scaled` - Exact value after factor conversion and before target offset.
///
/// # Returns
///
/// The operation name that identifies the first legacy Decimal boundary.
fn maximum_precision_overflow_operation(
    value: Decimal,
    source: UnitDefinition,
    target: UnitDefinition,
    scaled: &BigRational,
) -> &'static str {
    let Some(adjusted) = value.checked_add(source.offset()) else {
        return "add source offset";
    };
    if maximum_precision_decimal(scaled).is_none() {
        let source_factor = source.factor();
        let target_factor = target.factor();
        let (source_numerator, target_numerator) = reduce_ratio_terms(
            source_factor.numerator(),
            target_factor.numerator(),
        );
        let (target_denominator, source_denominator) = reduce_ratio_terms(
            target_factor.denominator(),
            source_factor.denominator(),
        );
        if let (Some(numerator), Some(_)) = (
            checked_mul_exact(source_numerator, target_denominator),
            checked_mul_exact(source_denominator, target_numerator),
        ) && adjusted.checked_mul(numerator).is_some()
        {
            return "divide conversion ratio";
        }
        return "multiply conversion ratio";
    }
    "subtract target offset"
}

/// Converts an exact rational to the most precise representable Decimal.
///
/// # Parameters
///
/// * `value` - Exact rational value to convert.
///
/// # Returns
///
/// The nearest-even Decimal at the greatest representable scale, or `None`
/// when even its integral magnitude exceeds Decimal's range.
fn maximum_precision_decimal(value: &BigRational) -> Option<Decimal> {
    for scale in (0..=Decimal::MAX_SCALE).rev() {
        let mantissa =
            round_rational(value, scale, RoundingStrategy::MidpointNearestEven);
        if let Some(decimal) = decimal_from_mantissa(mantissa, scale) {
            return Some(decimal.normalize());
        }
    }
    None
}

/// Converts an exact rational to a Decimal with one requested scale.
///
/// # Parameters
///
/// * `value` - Exact rational value to round.
/// * `scale` - Exact output scale.
/// * `rounding` - Strategy applied at the output boundary.
///
/// # Returns
///
/// The rounded Decimal retaining `scale`, or `None` when its mantissa exceeds
/// Decimal's range.
fn fixed_scale_decimal(
    value: &BigRational,
    scale: u32,
    rounding: RoundingStrategy,
) -> Option<Decimal> {
    decimal_from_mantissa(round_rational(value, scale, rounding), scale)
}

/// Rounds an exact rational to an integer mantissa at one Decimal scale.
///
/// # Parameters
///
/// * `value` - Exact rational value to round.
/// * `scale` - Decimal scale applied before integer rounding.
/// * `strategy` - Direction and midpoint policy.
///
/// # Returns
///
/// The signed rounded mantissa, without applying Decimal's 96-bit limit.
#[allow(deprecated)]
fn round_rational(
    value: &BigRational,
    scale: u32,
    strategy: RoundingStrategy,
) -> BigInt {
    let zero = BigInt::from(0_u8);
    let scaled_numerator = value.numer() * BigInt::from(10_u8).pow(scale);
    let denominator = value.denom();
    let quotient = &scaled_numerator / denominator;
    let remainder = &scaled_numerator % denominator;
    let negative = scaled_numerator < zero;
    let remainder_magnitude = if remainder < zero {
        -remainder
    } else {
        remainder
    };
    let midpoint_ordering = (&remainder_magnitude * 2_u8).cmp(denominator);
    let has_fraction = remainder_magnitude != zero;
    let quotient_is_odd = (&quotient % 2_u8) != zero;
    let direction = if negative { -1_i8 } else { 1_i8 };
    let increment = match strategy {
        RoundingStrategy::MidpointNearestEven
        | RoundingStrategy::BankersRounding => {
            midpoint_ordering == Ordering::Greater
                || (midpoint_ordering == Ordering::Equal && quotient_is_odd)
        }
        RoundingStrategy::MidpointAwayFromZero
        | RoundingStrategy::RoundHalfUp => midpoint_ordering != Ordering::Less,
        RoundingStrategy::MidpointTowardZero
        | RoundingStrategy::RoundHalfDown => {
            midpoint_ordering == Ordering::Greater
        }
        RoundingStrategy::ToZero | RoundingStrategy::RoundDown => false,
        RoundingStrategy::AwayFromZero | RoundingStrategy::RoundUp => {
            has_fraction
        }
        RoundingStrategy::ToNegativeInfinity => negative && has_fraction,
        RoundingStrategy::ToPositiveInfinity => !negative && has_fraction,
    };
    quotient + BigInt::from(if increment { direction } else { 0 })
}

/// Constructs a Decimal from a rounded arbitrary-precision mantissa.
///
/// # Parameters
///
/// * `mantissa` - Signed integer mantissa to validate.
/// * `scale` - Decimal scale associated with `mantissa`.
///
/// # Returns
///
/// The Decimal when the absolute mantissa fits 96 bits, or `None` otherwise.
fn decimal_from_mantissa(mantissa: BigInt, scale: u32) -> Option<Decimal> {
    let zero = BigInt::from(0_u8);
    let magnitude = if mantissa < zero {
        -mantissa.clone()
    } else {
        mantissa.clone()
    };
    if magnitude > BigInt::from((1_u128 << 96) - 1) {
        return None;
    }
    Decimal::try_from_i128_with_scale(i128::try_from(mantissa).ok()?, scale)
        .ok()
}

/// Applies explicit final rounding and retains exactly the requested scale.
///
/// # Arguments
///
/// * `value` - Converted Decimal before final output rounding.
/// * `options` - Optional output scale and rounding strategy.
///
/// # Returns
///
/// The unchanged value for maximum precision, or a value with exactly the
/// requested scale.
///
/// # Errors
///
/// Returns [`MeasurementError::ArithmeticOverflow`] when Decimal cannot retain
/// the requested scale.
fn apply_output_scale(
    mut value: Decimal,
    options: ConversionOptions,
) -> Result<Decimal, MeasurementError> {
    let (scale, rounding) = match options.mode() {
        ConversionMode::MaximumPrecision => return Ok(value),
        ConversionMode::FixedScale { scale, rounding } => (scale, rounding),
    };
    value = value.round_dp_with_strategy(scale, rounding);
    value.rescale(scale);
    if value.scale() != scale {
        return Err(MeasurementError::ArithmeticOverflow {
            operation: "set output scale",
        });
    }
    Ok(value)
}
