// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Decimal-only conversion arithmetic.

use std::cmp::Ordering;

use num_bigint_04::BigInt;
use num_rational::BigRational;
use rust_decimal::{
    Decimal,
    RoundingStrategy,
};

use crate::measure::internal::ConversionMode;
use crate::measure::{
    ConversionOptions,
    MeasurementError,
    UnitDefinition,
};

/// Converts a value between validated unit definitions without using floats.
///
/// # Parameters
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
/// Returns [`MeasurementError::ValueOutOfRange`] if the converted value is
/// outside the Decimal representation. Returns
/// [`MeasurementError::OutputScaleUnrepresentable`] if the value fits Decimal
/// but cannot retain the requested output scale.
pub(crate) fn convert_decimal(
    value: Decimal,
    source: UnitDefinition,
    target: UnitDefinition,
    options: ConversionOptions,
) -> Result<Decimal, MeasurementError> {
    if source == target
        || definitions_are_mathematically_equivalent(source, target)
    {
        return apply_output_scale(value, options);
    }

    let exact = convert_decimal_to_rational(value, source, target);
    if !fits_decimal_range(&exact) {
        return Err(MeasurementError::ValueOutOfRange);
    }

    match options.mode() {
        ConversionMode::MaximumPrecision => maximum_precision_decimal(&exact)
            .ok_or(MeasurementError::ValueOutOfRange),
        ConversionMode::FixedScale { scale, rounding } => {
            fixed_scale_decimal(&exact, scale, rounding)
                .ok_or(MeasurementError::OutputScaleUnrepresentable { scale })
        }
    }
}

/// Tests whether two unit definitions apply the same affine conversion.
///
/// # Parameters
///
/// * `source` - First validated unit definition.
/// * `target` - Second validated unit definition.
///
/// # Returns
///
/// `true` when both definitions have the same offset and mathematically equal
/// conversion factors, even if their stored Decimal ratio terms differ.
fn definitions_are_mathematically_equivalent(
    source: UnitDefinition,
    target: UnitDefinition,
) -> bool {
    if source.offset() != target.offset() {
        return false;
    }
    let source_factor = source.factor();
    let target_factor = target.factor();
    decimal_as_rational(source_factor.numerator())
        * decimal_as_rational(target_factor.denominator())
        == decimal_as_rational(target_factor.numerator())
            * decimal_as_rational(source_factor.denominator())
}

/// Converts a Decimal value exactly between validated unit definitions.
///
/// # Parameters
///
/// * `value` - Decimal value expressed by `source`.
/// * `source` - Definition of the input unit.
/// * `target` - Definition of the requested output unit.
///
/// # Returns
///
/// The exact rational value expressed by `target`, without applying a Decimal
/// representation boundary or rounding policy.
pub(super) fn convert_decimal_to_rational(
    value: Decimal,
    source: UnitDefinition,
    target: UnitDefinition,
) -> BigRational {
    let adjusted =
        decimal_as_rational(value) + decimal_as_rational(source.offset());
    let source_factor = source.factor();
    let target_factor = target.factor();
    let scaled = adjusted * decimal_as_rational(source_factor.numerator())
        / decimal_as_rational(source_factor.denominator())
        * decimal_as_rational(target_factor.denominator())
        / decimal_as_rational(target_factor.numerator());
    scaled - decimal_as_rational(target.offset())
}

/// Compares two Decimal values through exact unit conversion.
///
/// # Parameters
///
/// * `left_value` - Decimal value expressed by `left_definition`.
/// * `left_definition` - Validated definition of the left unit.
/// * `right_value` - Decimal value expressed by `right_definition`.
/// * `right_definition` - Validated definition of the right unit.
///
/// # Returns
///
/// The physical ordering of the two values without Decimal rounding or
/// floating-point conversion.
#[inline]
pub(super) fn compare_decimal_values(
    left_value: Decimal,
    left_definition: UnitDefinition,
    right_value: Decimal,
    right_definition: UnitDefinition,
) -> Ordering {
    let left_in_right_units = convert_decimal_to_rational(
        left_value,
        left_definition,
        right_definition,
    );
    left_in_right_units.cmp(&decimal_as_rational(right_value))
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
#[inline]
fn decimal_as_rational(value: Decimal) -> BigRational {
    BigRational::new(
        BigInt::from(value.mantissa()),
        BigInt::from(10_u8).pow(value.scale()),
    )
}

/// Tests whether an exact rational lies within Decimal's closed value range.
///
/// This check must precede Decimal rounding so an out-of-range rational cannot
/// round back to Decimal::MIN or Decimal::MAX.
fn fits_decimal_range(value: &BigRational) -> bool {
    let minimum = decimal_as_rational(Decimal::MIN);
    let maximum = decimal_as_rational(Decimal::MAX);
    value >= &minimum && value <= &maximum
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
#[inline(always)]
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
/// # Parameters
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
/// Returns [`MeasurementError::OutputScaleUnrepresentable`] when Decimal cannot
/// retain the requested scale.
#[inline]
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
        return Err(MeasurementError::OutputScaleUnrepresentable { scale });
    }
    Ok(value)
}
