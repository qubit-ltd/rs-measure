// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted measurement values and optional approximate adapters.

#[cfg(feature = "uom")]
use crate::measure::UomUnit;
use crate::measure::internal::MeasurementWire;
use crate::measure::{
    ConversionOptions,
    MeasurementError,
    Unit,
};
use rust_decimal::Decimal;
use serde::ser::SerializeStruct;
use serde::{
    Deserialize,
    Deserializer,
    Serialize,
    Serializer,
};
use std::fmt;
use std::str::FromStr;

/// A persisted measurement value for one concrete quantity.
///
/// `Measurement<U>` stores the decimal value exactly as it was supplied and
/// stores the unit family member alongside it. With the `uom` Cargo feature,
/// calculations can cross into an approximate `uom/f64` quantity, while
/// persistence keeps the original user-facing unit instead of only the
/// normalized base-unit value.
/// Its Serde contract encodes units through [`Unit::symbol`] and decodes them
/// through [`Unit::parse_lenient`], without requiring unit-specific Serde.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Measurement<U>
where
    U: Unit,
{
    /// The numeric value expressed in [`Measurement::unit`].
    pub value: Decimal,

    /// The typed unit used to interpret [`Measurement::value`].
    pub unit: U,
}

impl<U> Measurement<U>
where
    U: Unit,
{
    /// Creates a persisted measurement from a decimal value and typed unit.
    ///
    /// # Arguments
    ///
    /// * `value` - Exact Decimal value expressed in `unit`.
    /// * `unit` - Typed unit used to interpret and persist `value`.
    ///
    /// # Returns
    ///
    /// A measurement that preserves both supplied fields.
    #[must_use]
    #[inline(always)]
    pub const fn new(value: Decimal, unit: U) -> Self {
        Self { value, unit }
    }

    /// Parses a measurement whose unit must use its canonical symbol.
    ///
    /// # Arguments
    ///
    /// * `input` - Measurement text in `<decimal><unit>` or `<decimal> <unit>`
    ///   form.
    ///
    /// # Returns
    ///
    /// A typed measurement containing the parsed Decimal and canonical unit.
    ///
    /// # Errors
    ///
    /// Returns [`MeasurementError::InvalidMeasurement`] for malformed numeric
    /// text, [`MeasurementError::NonCanonicalUnit`] for a known alias, or
    /// [`MeasurementError::UnknownUnit`] for an unknown unit.
    pub fn parse_strict(input: &str) -> Result<Self, MeasurementError> {
        let (value_text, unit_text) = split_measurement_parts(input)
            .ok_or_else(|| {
                MeasurementError::InvalidMeasurement(input.to_owned())
            })?;
        let value = Decimal::from_str(value_text).map_err(|_| {
            MeasurementError::InvalidMeasurement(input.to_owned())
        })?;
        let unit = U::parse_strict(unit_text)?;
        Ok(Self::new(value, unit))
    }

    /// Returns the stable persisted quantity identifier represented here.
    ///
    /// # Returns
    ///
    /// [`Unit::QUANTITY`] for `U`.
    #[must_use]
    #[inline(always)]
    pub const fn quantity_name(&self) -> &'static str {
        U::QUANTITY
    }

    /// Converts this measurement using [`ConversionOptions::DEFAULT`].
    ///
    /// # Arguments
    ///
    /// * `target` - Unit in which the returned value is expressed.
    ///
    /// # Returns
    ///
    /// A measurement converted to `target` through exact Decimal factors.
    ///
    /// # Errors
    ///
    /// Returns unit-definition or Decimal arithmetic errors from the exact
    /// conversion engine.
    #[inline(always)]
    pub fn convert_to(self, target: U) -> Result<Self, MeasurementError> {
        self.convert_to_with_options(target, ConversionOptions::DEFAULT)
    }

    /// Converts this measurement using explicit Decimal options.
    ///
    /// # Arguments
    ///
    /// * `target` - Unit in which the returned value is expressed.
    /// * `options` - Final Decimal scale and rounding configuration.
    ///
    /// # Returns
    ///
    /// A measurement converted to `target` with the requested output policy.
    ///
    /// # Errors
    ///
    /// Returns unit-definition or Decimal arithmetic errors from the exact
    /// conversion engine, including an unrepresentable requested scale.
    #[inline]
    pub fn convert_to_with_options(
        self,
        target: U,
        options: ConversionOptions,
    ) -> Result<Self, MeasurementError> {
        let source = self.unit.definition()?;
        let target_definition = target.definition()?;
        let value =
            source.convert_value_to(self.value, target_definition, options)?;
        Ok(Self::new(value, target))
    }
}

#[cfg(feature = "uom")]
impl<U> Measurement<U>
where
    U: UomUnit,
{
    /// Creates a persisted measurement from a typed `uom` quantity.
    ///
    /// The returned value is expressed in `unit`, preserving the requested
    /// storage or display unit instead of always using the `uom` base unit.
    /// The bridge crosses `f64` and may lose Decimal precision.
    ///
    /// # Arguments
    ///
    /// * `quantity` - Approximate typed `uom/f64` quantity to adapt.
    /// * `unit` - Unit in which the persisted Decimal value is expressed.
    ///
    /// # Returns
    ///
    /// A persisted measurement expressed in `unit`.
    ///
    /// # Errors
    ///
    /// Returns [`MeasurementError::DecimalConversion`] when the approximate
    /// floating-point value cannot be represented as Decimal.
    #[inline(always)]
    pub fn from_uom_approx(
        quantity: U::Quantity,
        unit: U,
    ) -> Result<Self, MeasurementError> {
        unit.value_from_uom_approx(quantity)
            .map(|value| Self::new(value, unit))
    }

    /// Converts this measurement into its approximate typed `uom` quantity.
    ///
    /// This bridge crosses `f64` and may lose Decimal precision.
    ///
    /// # Returns
    ///
    /// The corresponding strongly typed `uom/f64` quantity.
    #[must_use]
    #[inline(always)]
    pub fn to_uom_approx(self) -> U::Quantity {
        self.unit.to_uom_approx(self.value)
    }
}

impl<U> Serialize for Measurement<U>
where
    U: Unit,
{
    /// Serializes the stable quantity, Decimal string, and canonical unit.
    ///
    /// # Arguments
    ///
    /// * `serializer` - Serde serializer receiving the three-field record.
    ///
    /// # Returns
    ///
    /// The serializer's successful output.
    ///
    /// # Errors
    ///
    /// Returns the serializer's error if any field cannot be written.
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Measurement", 3)?;
        state.serialize_field("quantity", U::QUANTITY)?;
        state.serialize_field("value", &self.value.to_string())?;
        state.serialize_field("unit", self.unit.symbol())?;
        state.end()
    }
}

impl<'de, U> Deserialize<'de> for Measurement<U>
where
    U: Unit,
{
    /// Deserializes and validates the three-field persistence representation.
    ///
    /// # Arguments
    ///
    /// * `deserializer` - Serde deserializer providing the persisted record.
    ///
    /// # Returns
    ///
    /// A measurement after quantity validation and lenient unit parsing.
    ///
    /// # Errors
    ///
    /// Returns a deserializer error for malformed Decimal text, mismatched
    /// quantity metadata, or an unknown unit.
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = MeasurementWire::deserialize(deserializer)?;
        if wire.quantity != U::QUANTITY {
            return Err(serde::de::Error::custom(
                MeasurementError::QuantityMismatch {
                    expected: U::QUANTITY.to_owned(),
                    actual: wire.quantity,
                },
            ));
        }
        let unit =
            U::parse_lenient(&wire.unit).map_err(serde::de::Error::custom)?;
        Ok(Self::new(wire.value, unit))
    }
}

impl<U> fmt::Display for Measurement<U>
where
    U: Unit,
{
    /// Formats this measurement as `<value> <unit>`.
    ///
    /// # Arguments
    ///
    /// * `formatter` - Destination formatter.
    ///
    /// # Returns
    ///
    /// The formatter result.
    #[inline]
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
    ///
    /// # Arguments
    ///
    /// * `input` - Measurement text in compact or space-separated form.
    ///
    /// # Returns
    ///
    /// A typed measurement parsed with lenient unit aliases.
    ///
    /// # Errors
    ///
    /// Returns [`MeasurementError::InvalidMeasurement`] for malformed value
    /// text or [`MeasurementError::UnknownUnit`] for an unknown unit.
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (value_text, unit_text) = split_measurement_parts(input)
            .ok_or_else(|| {
                MeasurementError::InvalidMeasurement(input.to_owned())
            })?;
        let value = Decimal::from_str(value_text).map_err(|_| {
            MeasurementError::InvalidMeasurement(input.to_owned())
        })?;
        let unit = U::parse_lenient(unit_text)?;
        Ok(Self::new(value, unit))
    }
}

/// Splits a measurement string into decimal value text and trimmed unit text.
///
/// # Arguments
///
/// * `input` - Candidate measurement text.
///
/// # Returns
///
/// `Some((value, unit))` when a syntactically valid Decimal prefix and a
/// non-empty plausible unit suffix are present. Space-separated suffixes may
/// start with `.`, `+`, or `-`; compact suffixes starting with those reserved
/// characters return `None` to avoid accepting malformed Decimal text.
#[inline]
fn split_measurement_parts(input: &str) -> Option<(&str, &str)> {
    let trimmed = input.trim();
    let value_len = decimal_prefix_len(trimmed)?;
    let (value_text, unit_suffix) = trimmed.split_at(value_len);
    let is_separated = unit_suffix.trim_start().len() != unit_suffix.len();
    let unit_text = unit_suffix.trim();
    if unit_text.is_empty()
        || (!is_separated && unit_text.starts_with(['.', '+', '-']))
    {
        None
    } else {
        Some((value_text, unit_text))
    }
}

/// Returns the byte length of the leading decimal value.
///
/// # Arguments
///
/// * `input` - Text beginning with an optional signed Decimal.
///
/// # Returns
///
/// `Some(length)` for a valid Decimal prefix, including a valid exponent when
/// present; otherwise, `None`.
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
///
/// # Arguments
///
/// * `bytes` - Complete measurement input as bytes.
/// * `index` - Offset immediately after the exponent marker.
///
/// # Returns
///
/// `Some(end)` after at least one exponent digit, or `None` for an invalid
/// suffix.
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
