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

    /// The measurement text cannot be parsed.
    #[error("invalid measurement: {0}")]
    InvalidMeasurement(String),

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

    /// Checked Decimal arithmetic could not represent an intermediate value.
    #[error("Decimal arithmetic overflow while {operation}")]
    ArithmeticOverflow {
        /// The operation that exceeded the Decimal range.
        operation: &'static str,
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
