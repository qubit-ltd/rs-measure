// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Opt-in Serde adapter for canonical compact measurement text.
//!
//! Apply this module with `#[serde(with = "qubit_measure::measurement_text")]`
//! when a field should use a string such as `"2 MiB"` instead of the default
//! three-field measurement record. Deserialization uses strict unit parsing.
//! Optional fields can use
//! `#[serde(default, with = "qubit_measure::measurement_text::option")]`.

use serde::Deserialize;
use serde::Deserializer;
use serde::Serializer;
use serde::de::Error as _;

use crate::Measurement;
use crate::Unit;

/// Serializes a measurement as canonical `<value> <unit>` text.
///
/// # Parameters
///
/// * `measurement` - Typed measurement to format with its canonical unit.
/// * `serializer` - Serde serializer receiving the formatted string.
///
/// # Returns
///
/// The serializer's successful output.
///
/// # Errors
///
/// Returns the serializer's error when it cannot emit the string.
#[inline]
pub fn serialize<S, U>(measurement: &Measurement<U>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    U: Unit,
{
    serializer.collect_str(measurement)
}

/// Deserializes canonical `<value> <unit>` text into a typed measurement.
///
/// The default measurement text byte limit applies after Serde constructs the
/// string. Transport and pre-allocation limits remain the caller's
/// responsibility.
///
/// # Parameters
///
/// * `deserializer` - Serde deserializer providing the compact string.
///
/// # Returns
///
/// The exactly parsed Decimal measurement and canonical typed unit.
///
/// # Errors
///
/// Returns the deserializer's error for a non-string value, oversized input,
/// invalid or unrepresentable Decimal text, a non-canonical alias, or an
/// unknown unit.
#[inline]
pub fn deserialize<'de, D, U>(deserializer: D) -> Result<Measurement<U>, D::Error>
where
    D: Deserializer<'de>,
    U: Unit,
{
    let text = String::deserialize(deserializer)?;
    Measurement::parse_strict(&text).map_err(D::Error::custom)
}

/// Serde adapter for optional canonical compact measurement text.
///
/// Apply this module with
/// `#[serde(default, with = "qubit_measure::measurement_text::option")]`.
/// Present measurements use canonical `<value> <unit>` strings, while absent
/// measurements use Serde's `null` representation.
pub mod option {
    use serde::Deserialize;
    use serde::Deserializer;
    use serde::Serialize;
    use serde::Serializer;
    use serde::de::Error as _;

    use crate::Measurement;
    use crate::Unit;

    /// Serializes an optional measurement as canonical text or `null`.
    ///
    /// # Parameters
    ///
    /// * `measurement` - Optional typed measurement to serialize.
    /// * `serializer` - Serde serializer receiving the string or `null`.
    ///
    /// # Returns
    ///
    /// The serializer's successful output.
    ///
    /// # Errors
    ///
    /// Returns the serializer's error when it cannot emit the value.
    #[inline]
    pub fn serialize<S, U>(measurement: &Option<Measurement<U>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        U: Unit,
    {
        measurement.as_ref().map(ToString::to_string).serialize(serializer)
    }

    /// Deserializes optional canonical text from a string or `null`.
    ///
    /// # Parameters
    ///
    /// * `deserializer` - Serde deserializer providing a string or `null`.
    ///
    /// # Returns
    ///
    /// `Some` containing the strictly parsed measurement for a string, or
    /// `None` for `null`.
    ///
    /// # Errors
    ///
    /// Returns the deserializer's error for a non-string value, oversized
    /// input, invalid or unrepresentable Decimal text, a non-canonical alias,
    /// or an unknown unit.
    #[inline]
    pub fn deserialize<'de, D, U>(deserializer: D) -> Result<Option<Measurement<U>>, D::Error>
    where
        D: Deserializer<'de>,
        U: Unit,
    {
        Option::<String>::deserialize(deserializer)?
            .map(|text| Measurement::parse_strict(&text).map_err(D::Error::custom))
            .transpose()
    }
}
