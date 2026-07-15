// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Decimal-only conversion arithmetic.

use rust_decimal::Decimal;

use crate::measure::{
    ConversionOptions,
    MeasurementError,
    UnitDefinition,
};

/// Converts a value between validated unit definitions without using floats.
///
/// Returns an arithmetic error if any intermediate or requested final scale is
/// outside the Decimal representation.
pub(crate) fn convert_decimal(
    value: Decimal,
    source: UnitDefinition,
    target: UnitDefinition,
    options: ConversionOptions,
) -> Result<Decimal, MeasurementError> {
    if source == target && options.scale().is_none() {
        return Ok(value);
    }

    let adjusted = value.checked_add(source.offset()).ok_or(
        MeasurementError::ArithmeticOverflow {
            operation: "add source offset",
        },
    )?;
    let source_factor = source.factor();
    let target_factor = target.factor();
    let converted = match (
        source_factor
            .numerator()
            .checked_mul(target_factor.denominator()),
        source_factor
            .denominator()
            .checked_mul(target_factor.numerator()),
    ) {
        (Some(numerator), Some(denominator)) => {
            apply_ratio(adjusted, numerator, denominator)?
        }
        _ => {
            let base = apply_ratio(
                adjusted,
                source_factor.numerator(),
                source_factor.denominator(),
            )?;
            apply_ratio(
                base,
                target_factor.denominator(),
                target_factor.numerator(),
            )?
        }
    };
    let result = converted.checked_sub(target.offset()).ok_or(
        MeasurementError::ArithmeticOverflow {
            operation: "subtract target offset",
        },
    )?;
    apply_output_scale(result, options)
}

/// Applies a positive ratio while preferring multiplication before division.
///
/// If multiplication overflows, division is attempted first so a
/// mathematically representable result can still succeed.
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
fn apply_output_scale(
    mut value: Decimal,
    options: ConversionOptions,
) -> Result<Decimal, MeasurementError> {
    let Some(scale) = options.scale() else {
        return Ok(value);
    };
    value = value.round_dp_with_strategy(scale, options.rounding());
    value.rescale(scale);
    if value.scale() != scale {
        return Err(MeasurementError::ArithmeticOverflow {
            operation: "set output scale",
        });
    }
    Ok(value)
}
