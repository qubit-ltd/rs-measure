// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Errors returned by measurement parsing, conversion, and adapters.

use thiserror::Error;

/// Errors returned by parsing, converting, or adapting measurements.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum MeasurementError {
    /// The unit symbol is not known for the requested quantity.
    #[error("unknown {quantity} unit: {unit}")]
    UnknownUnit {
        /// The quantity whose unit table was used.
        quantity: String,

        /// The unknown unit symbol.
        unit: String,
    },

    /// The measurement text exceeds the configured byte limit.
    #[error("measurement text exceeds the {maximum}-byte limit")]
    MeasurementTextLimitExceeded {
        /// Inclusive maximum accepted number of UTF-8 bytes.
        maximum: usize,
    },

    /// The measurement text does not follow the supported grammar.
    #[error("invalid measurement syntax")]
    InvalidMeasurementSyntax,

    /// The numeric value cannot be represented exactly as Decimal.
    #[error("measurement value cannot be represented exactly as Decimal")]
    UnrepresentableMeasurementValue,

    /// Compact text admits more than one valid numeric/unit split.
    #[error(
        "ambiguous measurement {input}; matching units: {units}",
        units = .units.join(", "),
    )]
    AmbiguousMeasurement {
        /// Original measurement text.
        input: String,

        /// Unit suffixes that each leave a valid Decimal prefix.
        units: Vec<String>,
    },

    /// A negative time measurement cannot be represented by `Duration`.
    #[error("negative duration: {value} {unit}")]
    NegativeDuration {
        /// Original Decimal measurement value.
        value: rust_decimal::Decimal,

        /// Canonical symbol of the original time unit.
        unit: String,
    },

    /// A time measurement contains a fractional nanosecond.
    #[error("duration has subnanosecond precision: {value} {unit}")]
    SubnanosecondDuration {
        /// Original Decimal measurement value.
        value: rust_decimal::Decimal,

        /// Canonical symbol of the original time unit.
        unit: String,
    },

    /// A non-negative time measurement exceeds `Duration::MAX`.
    #[error("duration is out of range: {value} {unit}")]
    DurationOutOfRange {
        /// Original Decimal measurement value.
        value: rust_decimal::Decimal,

        /// Canonical symbol of the original time unit.
        unit: String,
    },

    /// A negative information measurement cannot represent a byte count.
    #[error("negative information size: {value} {unit}")]
    NegativeInformation {
        /// Original Decimal measurement value.
        value: rust_decimal::Decimal,

        /// Canonical symbol of the original information unit.
        unit: String,
    },

    /// An information measurement is not an exact whole number of bytes.
    #[error("information size is not a whole number of bytes: {value} {unit}")]
    FractionalByteInformation {
        /// Original Decimal measurement value.
        value: rust_decimal::Decimal,

        /// Canonical symbol of the original information unit.
        unit: String,
    },

    /// An exact byte count cannot be represented by the requested integer.
    #[error("information size is out of range for {target}: {value} {unit}")]
    InformationOutOfRange {
        /// Original Decimal measurement value.
        value: rust_decimal::Decimal,

        /// Canonical symbol of the original information unit.
        unit: String,

        /// Name of the requested integer target type.
        target: &'static str,
    },

    /// Floating-point-to-decimal conversion failed.
    #[error("f64 value cannot be represented as Decimal: {0}")]
    DecimalConversion(String),

    /// The requested Decimal scale exceeds the supported limit.
    #[error("invalid Decimal scale {scale}; maximum is {max}")]
    InvalidScale {
        /// The requested scale.
        scale: u32,

        /// The largest supported scale.
        max: u32,
    },

    /// The converted value cannot be represented by Decimal.
    #[error("converted value is outside the Decimal range")]
    ValueOutOfRange,

    /// The converted value cannot retain the requested Decimal scale.
    #[error("converted value cannot retain Decimal scale {scale}")]
    OutputScaleUnrepresentable {
        /// Requested Decimal scale that could not be retained.
        scale: u32,
    },

    /// A unit definition contains an invalid conversion factor or offset.
    #[error("invalid unit definition: {reason}")]
    InvalidUnitDefinition {
        /// The validation failure.
        reason: String,
    },

    /// Strict parsing recognized an alias instead of a canonical unit symbol.
    #[error("non-canonical {quantity} unit {unit}; use {canonical}")]
    NonCanonicalUnit {
        /// The quantity whose unit table was used.
        quantity: String,

        /// The recognized non-canonical alias.
        unit: String,

        /// The canonical symbol accepted by strict parsing.
        canonical: String,
    },

    /// Serialized quantity metadata does not match the target unit family.
    #[error("quantity mismatch: expected {expected}, got {actual}")]
    QuantityMismatch {
        /// The target unit family's quantity identifier.
        expected: String,

        /// The quantity identifier found in serialized data.
        actual: String,
    },
}
