// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted measurement values and `uom` adapters.

use crate::measure::MeasurementError;
use crate::measure::Unit;
use rust_decimal::Decimal;
use serde::{
    Deserialize,
    Serialize,
};
use std::fmt;
use std::str::FromStr;

/// A persisted measurement value for one concrete `uom` quantity.
///
/// `Measurement<U>` stores the decimal value exactly as it was supplied and
/// stores the unit family member alongside it. Calculations can cross into
/// `uom` with [`Measurement::to_uom`], while persistence keeps the original
/// user-facing unit instead of only the normalized base-unit value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound(
    serialize = "U: Serialize",
    deserialize = "U: Deserialize<'de>"
))]
pub struct Measurement<U>
where
    U: Unit,
{
    /// The numeric value expressed in [`Measurement::unit`].
    #[serde(with = "rust_decimal::serde::str")]
    pub value: Decimal,

    /// The typed unit used to interpret [`Measurement::value`].
    pub unit: U,
}

impl<U> Measurement<U>
where
    U: Unit,
{
    /// Creates a persisted measurement from a decimal value and typed unit.
    #[must_use]
    pub const fn new(value: Decimal, unit: U) -> Self {
        Self { value, unit }
    }

    /// Returns the lower-case `uom` quantity name represented by this value.
    #[must_use]
    pub const fn quantity_name(&self) -> &'static str {
        U::QUANTITY
    }

    /// Converts this measurement into its typed `uom` quantity.
    #[must_use]
    pub fn to_uom(self) -> U::Quantity {
        self.unit.to_uom(self.value)
    }

    /// Creates a persisted measurement from a typed `uom` quantity.
    ///
    /// The returned value is expressed in `unit`, preserving the requested
    /// storage or display unit instead of always using the `uom` base unit.
    pub fn from_uom(
        quantity: U::Quantity,
        unit: U,
    ) -> Result<Self, MeasurementError> {
        unit.value_from_uom(quantity)
            .map(|value| Self::new(value, unit))
    }

    /// Converts this measurement to another unit from the same quantity family.
    ///
    /// The conversion is delegated to `uom`: this crate does not maintain an
    /// independent conversion table.
    pub fn convert_to(self, target: U) -> Result<Self, MeasurementError> {
        if self.unit == target {
            return Ok(Self::new(self.value, target));
        }
        Self::from_uom(self.to_uom(), target)
    }
}

impl<U> fmt::Display for Measurement<U>
where
    U: Unit,
{
    /// Formats this measurement as `<value> <unit>`.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {}", self.value, self.unit)
    }
}

impl<U> FromStr for Measurement<U>
where
    U: Unit,
{
    type Err = MeasurementError;

    /// Parses a typed measurement written as `<decimal><unit>` or `<decimal>
    /// <unit>`.
    ///
    /// The unit is resolved only inside `U`'s quantity family, so parsing a
    /// mass unit as a length measurement returns
    /// [`MeasurementError::UnknownUnit`].
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (value_text, unit_text) = split_measurement_parts(input)
            .ok_or_else(|| {
                MeasurementError::InvalidMeasurement(input.to_owned())
            })?;
        let value = Decimal::from_str(value_text).map_err(|_| {
            MeasurementError::InvalidMeasurement(input.to_owned())
        })?;
        let unit = U::from_str(unit_text)?;
        Ok(Self::new(value, unit))
    }
}

/// Splits a measurement string into decimal value text and trimmed unit text.
fn split_measurement_parts(input: &str) -> Option<(&str, &str)> {
    let trimmed = input.trim();
    let value_len = decimal_prefix_len(trimmed)?;
    let (value_text, unit_text) = trimmed.split_at(value_len);
    let unit_text = unit_text.trim();
    if unit_text.is_empty() || unit_text.starts_with(['.', '+', '-']) {
        None
    } else {
        Some((value_text, unit_text))
    }
}

/// Returns the byte length of the leading decimal value.
fn decimal_prefix_len(input: &str) -> Option<usize> {
    let bytes = input.as_bytes();
    let mut index = 0;
    if matches!(bytes.first(), Some(b'+' | b'-')) {
        index += 1;
    }

    let mut has_digit = false;
    let mut has_dot = false;
    while let Some(byte) = bytes.get(index) {
        match byte {
            b'0'..=b'9' => {
                has_digit = true;
                index += 1;
            }
            b'.' if !has_dot => {
                has_dot = true;
                index += 1;
            }
            b'e' | b'E' if has_digit => {
                if let Some(end) = exponent_end(bytes, index + 1) {
                    return Some(end);
                }
                break;
            }
            b'.' | b'+' | b'-' => return None,
            _ => break,
        }
    }

    has_digit.then_some(index)
}

/// Returns the end offset of a valid exponent suffix.
fn exponent_end(bytes: &[u8], mut index: usize) -> Option<usize> {
    if matches!(bytes.get(index), Some(b'+' | b'-')) {
        index += 1;
    }

    let digits_start = index;
    while matches!(bytes.get(index), Some(b'0'..=b'9')) {
        index += 1;
    }
    (index > digits_start).then_some(index)
}
