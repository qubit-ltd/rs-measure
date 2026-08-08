// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted measurement values and optional approximate adapters.

use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::de::Error;
use serde::ser::SerializeStruct;

use crate::measure::ConversionOptions;
use crate::measure::MeasurementError;
use crate::measure::MeasurementParseOptions;
use crate::measure::Unit;
#[cfg(feature = "uom")]
use crate::measure::UomUnit;
use crate::measure::decimal_conversion::compare_decimal_values;
use crate::measure::internal::MeasurementWire;
use crate::measure::internal::parse_measurement_text;

/// A persisted measurement value for one concrete quantity.
///
/// `Measurement<U>` stores the decimal value exactly as it was supplied and
/// stores the unit family member alongside it. With the `uom` Cargo feature,
/// calculations can cross into an approximate `uom/f64` quantity, while
/// persistence keeps the original user-facing unit instead of only the
/// normalized base-unit value.
/// Its Serde contract encodes units through [`Unit::symbol`] and decodes them
/// through [`Unit::parse_strict`], without requiring unit-specific Serde. Each
/// decoded wire string is limited to
/// [`MeasurementParseOptions::DEFAULT_MAX_TEXT_BYTES`] bytes. This is an
/// acceptance and parsing-work limit applied after Serde has constructed the
/// string; callers needing payload or pre-allocation limits must configure
/// them at the transport or deserializer boundary.
///
/// Derived [`PartialEq`] and [`Eq`] compare the stored Decimal and unit fields,
/// not their converted physical values. Use [`Measurement::equivalent_to`] or
/// [`Measurement::try_cmp_exact`] for exact cross-unit semantics.
///
/// # Examples
///
/// Discarding a measurement is diagnosed when unused results are denied:
///
/// ```compile_fail
/// #![deny(unused_must_use)]
/// use qubit_measure::{Measurement, unit};
/// use rust_decimal::Decimal;
///
/// Measurement::new(Decimal::ONE, unit::Length::Meter);
/// ```
#[must_use]
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
    /// # Parameters
    ///
    /// * `value` - Exact Decimal value expressed in `unit`.
    /// * `unit` - Typed unit used to interpret and persist `value`.
    ///
    /// # Returns
    ///
    /// A measurement that preserves both supplied fields.
    #[inline(always)]
    pub const fn new(value: Decimal, unit: U) -> Self {
        Self { value, unit }
    }

    /// Parses a measurement whose unit must use its canonical symbol.
    ///
    /// # Parameters
    ///
    /// * `input` - Measurement text in `<decimal><unit>` or `<decimal> <unit>`
    ///   form.
    ///
    /// Unit symbols beginning with `.`, `+`, or `-` require whitespace before
    /// the unit, for example `"1.25 +cu"`; their compact forms are rejected as
    /// ambiguous Decimal boundaries.
    /// Scientific notation is accepted only when the final value is exactly
    /// representable; no parsing path rounds. The parser retains as much input
    /// scale as Decimal can hold and applies the default 1 MiB text limit.
    ///
    /// # Returns
    ///
    /// A typed measurement containing the parsed Decimal and canonical unit.
    ///
    /// # Errors
    ///
    /// Returns [`MeasurementError::MeasurementTextLimitExceeded`] for
    /// oversized input, [`MeasurementError::InvalidMeasurementSyntax`] for
    /// malformed text,
    /// [`MeasurementError::UnrepresentableMeasurementValue`] when Decimal
    /// cannot hold the value exactly, or a classified unit or ambiguity error.
    #[inline(always)]
    pub fn parse_strict(input: &str) -> Result<Self, MeasurementError> {
        Self::parse_strict_with_options(
            input,
            &MeasurementParseOptions::default(),
        )
    }

    /// Parses a canonical measurement using explicit resource limits.
    ///
    /// # Parameters
    ///
    /// * `input` - Measurement text in compact or space-separated form.
    /// * `options` - Resource limits applied before parsing.
    ///
    /// # Returns
    ///
    /// A typed measurement containing the parsed Decimal and canonical unit.
    ///
    /// # Errors
    ///
    /// Returns a classified measurement syntax, representation, size, unit,
    /// or ambiguity error.
    #[inline(always)]
    pub fn parse_strict_with_options(
        input: &str,
        options: &MeasurementParseOptions,
    ) -> Result<Self, MeasurementError> {
        let (value, unit) = parse_measurement_text::<U>(input, true, options)?;
        Ok(Self::new(value, unit))
    }

    /// Parses a measurement accepting canonical symbols and documented aliases.
    ///
    /// # Parameters
    ///
    /// * `input` - Measurement text in `<decimal><unit>` or `<decimal> <unit>`
    ///   form.
    ///
    /// This convenience method applies the default 1 MiB text limit and never
    /// rounds an unrepresentable Decimal value.
    ///
    /// # Returns
    ///
    /// A typed measurement containing the parsed Decimal and resolved unit.
    ///
    /// # Errors
    ///
    /// Returns [`MeasurementError::MeasurementTextLimitExceeded`] for
    /// oversized input, [`MeasurementError::InvalidMeasurementSyntax`] for
    /// malformed text,
    /// [`MeasurementError::UnrepresentableMeasurementValue`] when Decimal
    /// cannot hold the value exactly, or a classified unit or ambiguity error.
    ///
    /// # Examples
    ///
    /// ```
    /// use qubit_measure::{Measurement, unit};
    ///
    /// let value = Measurement::<unit::Time>::parse_lenient("1 year")?;
    /// assert_eq!(value.unit, unit::Time::CommonYear365);
    /// # Ok::<(), qubit_measure::MeasurementError>(())
    /// ```
    #[inline(always)]
    pub fn parse_lenient(input: &str) -> Result<Self, MeasurementError> {
        Self::parse_lenient_with_options(
            input,
            &MeasurementParseOptions::default(),
        )
    }

    /// Parses a lenient measurement using explicit resource limits.
    ///
    /// # Parameters
    ///
    /// * `input` - Measurement text in compact or space-separated form.
    /// * `options` - Resource limits applied before parsing.
    ///
    /// # Returns
    ///
    /// A typed measurement containing the parsed Decimal and resolved unit.
    ///
    /// # Errors
    ///
    /// Returns a classified measurement syntax, representation, size, unit,
    /// or ambiguity error.
    #[inline(always)]
    pub fn parse_lenient_with_options(
        input: &str,
        options: &MeasurementParseOptions,
    ) -> Result<Self, MeasurementError> {
        let (value, unit) = parse_measurement_text::<U>(input, false, options)?;
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

    /// Compares this measurement with another value of the same quantity.
    ///
    /// The comparison converts through exact rational arithmetic. It does not
    /// apply a Decimal output scale or rounding policy and does not cross
    /// floating point.
    ///
    /// # Parameters
    ///
    /// * `other` - Measurement of the same unit family to compare against.
    ///
    /// # Returns
    ///
    /// The exact physical ordering of `self` and `other`.
    ///
    /// # Errors
    ///
    /// Returns [`MeasurementError::InvalidUnitDefinition`] when either unit
    /// cannot provide a valid exact definition.
    #[inline]
    pub fn try_cmp_exact(
        &self,
        other: &Self,
    ) -> Result<Ordering, MeasurementError> {
        let left_definition = self.unit.definition()?;
        let right_definition = other.unit.definition()?;
        Ok(compare_decimal_values(
            self.value,
            left_definition,
            other.value,
            right_definition,
        ))
    }

    /// Tests whether another measurement represents the same physical value.
    ///
    /// Unlike derived [`PartialEq`], this method compares across units through
    /// exact rational arithmetic and performs no Decimal rounding.
    ///
    /// # Parameters
    ///
    /// * `other` - Measurement of the same unit family to compare against.
    ///
    /// # Returns
    ///
    /// `true` when both measurements represent exactly the same physical
    /// value; otherwise, `false`.
    ///
    /// # Errors
    ///
    /// Returns [`MeasurementError::InvalidUnitDefinition`] when either unit
    /// cannot provide a valid exact definition.
    #[inline]
    pub fn equivalent_to(
        &self,
        other: &Self,
    ) -> Result<bool, MeasurementError> {
        self.try_cmp_exact(other)
            .map(|ordering| ordering == Ordering::Equal)
    }

    /// Converts this measurement using [`ConversionOptions::DEFAULT`].
    ///
    /// # Parameters
    ///
    /// * `target` - Unit in which the returned value is expressed.
    ///
    /// # Returns
    ///
    /// A measurement converted to `target` through exact rational factors and
    /// the default maximum-precision Decimal output policy.
    /// When the source and target definitions are mathematically equivalent,
    /// this policy preserves the original Decimal representation, including
    /// its scale and trailing zeroes.
    ///
    /// # Errors
    ///
    /// Returns unit-definition errors or
    /// [`MeasurementError::ValueOutOfRange`] from the conversion engine.
    #[inline(always)]
    pub fn convert_to(self, target: U) -> Result<Self, MeasurementError> {
        self.convert_to_with_options(target, ConversionOptions::DEFAULT)
    }

    /// Converts this measurement using explicit Decimal options.
    ///
    /// # Parameters
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
    /// Returns unit-definition errors,
    /// [`MeasurementError::ValueOutOfRange`], or
    /// [`MeasurementError::OutputScaleUnrepresentable`] from the conversion
    /// engine.
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
    /// # Parameters
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
    /// Returns [`MeasurementError::InvalidUnitDefinition`] when an external
    /// unit family cannot provide a valid exact definition. Returns
    /// [`MeasurementError::DecimalConversion`] when the approximate
    /// floating-point value cannot be represented as Decimal.
    #[inline(always)]
    pub fn from_uom_approx(
        quantity: U::Quantity,
        unit: U,
    ) -> Result<Self, MeasurementError> {
        unit.value_from_uom_approx(quantity)
            .map(|value| Self::new(value, unit))
    }

    /// Tries to convert this measurement into its approximate typed `uom`
    /// quantity.
    ///
    /// This bridge crosses `f64` and may lose Decimal precision.
    ///
    /// # Returns
    ///
    /// The corresponding strongly typed `uom/f64` quantity.
    ///
    /// # Errors
    ///
    /// Returns [`MeasurementError::InvalidUnitDefinition`] when an external
    /// unit family cannot provide a valid exact definition.
    #[inline(always)]
    pub fn try_to_uom_approx(self) -> Result<U::Quantity, MeasurementError> {
        self.unit.try_to_uom_approx(self.value)
    }

    /// Converts this measurement into its approximate typed `uom` quantity.
    ///
    /// This bridge crosses `f64` and may lose Decimal precision.
    ///
    /// # Returns
    ///
    /// The corresponding strongly typed `uom/f64` quantity.
    ///
    /// # Panics
    ///
    /// Panics when [`UomUnit::to_uom_approx`] panics. Its default
    /// implementation panics if [`UomUnit::try_to_uom_approx`] returns an
    /// error. Use [`Measurement::try_to_uom_approx`] for a fallible
    /// conversion.
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
    /// # Parameters
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
    /// # Parameters
    ///
    /// * `deserializer` - Serde deserializer providing the persisted record.
    ///
    /// # Returns
    ///
    /// A measurement after quantity validation and strict unit parsing.
    ///
    /// # Errors
    ///
    /// Returns a deserializer error for an oversized wire string, malformed or
    /// unrepresentable Decimal text, mismatched quantity metadata, a documented
    /// but non-canonical unit alias, or an unknown unit.
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = MeasurementWire::deserialize(deserializer)?;
        if wire.quantity != U::QUANTITY {
            return Err(Error::custom(MeasurementError::QuantityMismatch {
                expected: U::QUANTITY.to_owned(),
                actual: wire.quantity,
            }));
        }
        let unit = U::parse_strict(&wire.unit).map_err(Error::custom)?;
        Ok(Self::new(wire.value, unit))
    }
}

impl<U> fmt::Display for Measurement<U>
where
    U: Unit,
{
    /// Formats this measurement as `<value> <unit>`.
    ///
    /// # Parameters
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
    /// Unit symbols beginning with `.`, `+`, or `-` require
    /// whitespace before the unit, for example `"1.25 +cu"`; their compact
    /// forms are rejected as ambiguous Decimal boundaries.
    /// This default entry point applies a 1 MiB input limit and requires the
    /// final Decimal value to be exactly representable without rounding.
    ///
    /// # Parameters
    ///
    /// * `input` - Measurement text in compact or space-separated form.
    ///
    /// # Returns
    ///
    /// A typed measurement parsed from a canonical unit symbol.
    ///
    /// # Errors
    ///
    /// Returns [`MeasurementError::MeasurementTextLimitExceeded`] for
    /// oversized input, [`MeasurementError::InvalidMeasurementSyntax`] for
    /// malformed text,
    /// [`MeasurementError::UnrepresentableMeasurementValue`] when Decimal
    /// cannot hold the value exactly, or a classified unit or ambiguity error.
    #[inline(always)]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::parse_strict(input)
    }
}
