// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact integer-byte adapters for information measurements.

use num_bigint_04::BigInt;
use rust_decimal::Decimal;

use crate::measure::Information;
use crate::measure::Measurement;
use crate::measure::MeasurementError;
use crate::measure::Unit;
use crate::measure::decimal_conversion::convert_decimal_to_rational;

/// Converts an information measurement into a non-negative whole-byte value.
///
/// # Parameters
///
/// * `measurement` - Information measurement to convert without rounding.
///
/// # Returns
///
/// The exact non-negative number of bytes as an arbitrary-precision integer.
///
/// # Errors
///
/// Returns [`MeasurementError::NegativeInformation`] for negative values or
/// [`MeasurementError::FractionalByteInformation`] when the value is not a
/// whole number of bytes.
fn exact_nonnegative_bytes(
    measurement: Measurement<Information>,
) -> Result<BigInt, MeasurementError> {
    let original_value = measurement.value;
    let original_unit = measurement.unit.symbol().to_owned();
    if original_value < Decimal::ZERO {
        return Err(MeasurementError::NegativeInformation {
            value: original_value,
            unit: original_unit,
        });
    }
    let source = measurement
        .unit
        .definition()
        .expect("built-in Information definitions are valid");
    let target = Information::Byte
        .definition()
        .expect("built-in Information definitions are valid");
    let bytes = convert_decimal_to_rational(original_value, source, target);
    if !bytes.is_integer() {
        return Err(MeasurementError::FractionalByteInformation {
            value: original_value,
            unit: original_unit,
        });
    }
    Ok(bytes.to_integer())
}

impl From<u64> for Measurement<Information> {
    /// Creates an information measurement from an exact byte count.
    ///
    /// # Parameters
    ///
    /// * `bytes` - Non-negative number of bytes to store.
    ///
    /// # Returns
    ///
    /// An information measurement expressed in [`Information::Byte`].
    #[inline(always)]
    fn from(bytes: u64) -> Self {
        Self::new(Decimal::from(bytes), Information::Byte)
    }
}

impl From<usize> for Measurement<Information> {
    /// Creates an information measurement from an exact byte count.
    ///
    /// # Parameters
    ///
    /// * `bytes` - Non-negative number of bytes to store.
    ///
    /// # Returns
    ///
    /// An information measurement expressed in [`Information::Byte`].
    #[inline(always)]
    fn from(bytes: usize) -> Self {
        Self::new(Decimal::from(bytes), Information::Byte)
    }
}

impl TryFrom<Measurement<Information>> for u64 {
    type Error = MeasurementError;

    /// Converts an information measurement into an exact `u64` byte count.
    ///
    /// # Parameters
    ///
    /// * `measurement` - Information measurement to convert without rounding.
    ///
    /// # Returns
    ///
    /// The exact non-negative number of bytes.
    ///
    /// # Errors
    ///
    /// Returns a classified negative-value, fractional-byte, or `u64`
    /// out-of-range error.
    fn try_from(measurement: Measurement<Information>) -> Result<Self, Self::Error> {
        let value = measurement.value;
        let unit = measurement.unit.symbol().to_owned();
        let bytes = exact_nonnegative_bytes(measurement)?;
        Self::try_from(bytes).map_err(|_| MeasurementError::InformationOutOfRange {
            value,
            unit,
            target: "u64",
        })
    }
}

impl TryFrom<Measurement<Information>> for usize {
    type Error = MeasurementError;

    /// Converts an information measurement into an exact `usize` byte count.
    ///
    /// # Parameters
    ///
    /// * `measurement` - Information measurement to convert without rounding.
    ///
    /// # Returns
    ///
    /// The exact non-negative number of bytes.
    ///
    /// # Errors
    ///
    /// Returns a classified negative-value, fractional-byte, or `usize`
    /// out-of-range error.
    fn try_from(measurement: Measurement<Information>) -> Result<Self, Self::Error> {
        let value = measurement.value;
        let unit = measurement.unit.symbol().to_owned();
        let bytes = exact_nonnegative_bytes(measurement)?;
        Self::try_from(bytes).map_err(|_| MeasurementError::InformationOutOfRange {
            value,
            unit,
            target: "usize",
        })
    }
}
