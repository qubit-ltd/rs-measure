// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Decimal-only conversion arithmetic.

use rust_decimal::Decimal;

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
/// Returns [`MeasurementError::ArithmeticOverflow`] if any intermediate or
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

    let adjusted = value.checked_add(source.offset()).ok_or(
        MeasurementError::ArithmeticOverflow {
            operation: "add source offset",
        },
    )?;
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
    let converted = match (
        checked_mul_exact(source_numerator, target_denominator),
        checked_mul_exact(source_denominator, target_numerator),
    ) {
        (Some(numerator), Some(denominator)) => {
            apply_ratio(adjusted, numerator, denominator)?
        }
        _ => {
            let base =
                apply_ratio(adjusted, source_numerator, source_denominator)?;
            apply_ratio(base, target_denominator, target_numerator)?
        }
    };
    let result = converted.checked_sub(target.offset()).ok_or(
        MeasurementError::ArithmeticOverflow {
            operation: "subtract target offset",
        },
    )?;
    apply_output_scale(result, options)
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

/// Applies a positive ratio while preferring multiplication before division.
///
/// If multiplication overflows, division is attempted first so a
/// mathematically representable result can still succeed.
///
/// # Arguments
///
/// * `value` - Decimal value to scale.
/// * `numerator` - Positive reduced numerator.
/// * `denominator` - Positive reduced denominator.
///
/// # Returns
///
/// `value * numerator / denominator` when representable.
///
/// # Errors
///
/// Returns [`MeasurementError::ArithmeticOverflow`] if neither checked
/// operation order can represent the result.
fn apply_ratio(
    value: Decimal,
    numerator: Decimal,
    denominator: Decimal,
) -> Result<Decimal, MeasurementError> {
    if let Some(product) = value.checked_mul(numerator) {
        return product.checked_div(denominator).ok_or(
            MeasurementError::ArithmeticOverflow {
                operation: "divide conversion ratio",
            },
        );
    }

    let quotient = value.checked_div(denominator).ok_or(
        MeasurementError::ArithmeticOverflow {
            operation: "divide conversion ratio",
        },
    )?;
    quotient.checked_mul(numerator).ok_or(
        MeasurementError::ArithmeticOverflow {
            operation: "multiply conversion ratio",
        },
    )
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
