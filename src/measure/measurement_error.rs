/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
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
}
