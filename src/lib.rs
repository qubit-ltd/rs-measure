// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # qubit-measure
//!
//! Persistent typed Decimal measurement values with explicit units.
//!
//! The default build contains the exact Decimal core only. Enable the
//! default-off `uom` Cargo feature to expose approximate `uom/f64` adapters.
//! Built-in exact conversion definitions share crate-wide constants grouped by
//! quantity.
//!
//! # Examples
//!
//! Parse a measurement, derive a rounded conversion, and persist the original
//! value with its unit. The converted measurement is a derived value; JSON
//! serialization retains the original value and unit, including the Decimal
//! scale supplied by the input.
//!
//! ```
//! use qubit_measure::{ConversionOptions, measurement, unit};
//! use rust_decimal::RoundingStrategy;
//! use serde_json::json;
//!
//! let original = measurement::Length::parse_strict("50.0 cm")?;
//! let meters = original.convert_to_with_options(
//!     unit::Length::Meter,
//!     ConversionOptions::fixed_scale(4, RoundingStrategy::MidpointNearestEven)?,
//! )?;
//! assert_eq!(meters.to_string(), "0.5000 m");
//!
//! let stored = serde_json::to_value(original)?;
//! assert_eq!(stored, json!({"quantity":"length","value":"50.0","unit":"cm"}));
//!
//! let restored: measurement::Length = serde_json::from_value(stored)?;
//! assert_eq!(restored.value, original.value);
//! assert_eq!(restored.unit, unit::Length::Centimeter);
//! assert_eq!(restored.value.scale(), 1);
//! assert_eq!(restored.to_string(), "50.0 cm");
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # Further reading
//!
//! Explore [`Measurement`], [`ConversionOptions`], [`Unit`], and
//! [`measurement_text`] for the core value model, conversion policies, unit
//! definitions, and text parsing and formatting APIs. The English and Chinese
//! guides are available in the [README](https://github.com/qubit-ltd/rs-measure/blob/main/README.md)
//! and [中文 README](https://github.com/qubit-ltd/rs-measure/blob/main/README.zh_CN.md).

#![deny(missing_docs)]
#![deny(unsafe_op_in_unsafe_fn)]

#[doc(hidden)]
#[path = "private.rs"]
pub mod __private;
mod consts;
mod measure;
pub mod measurement;
pub mod measurement_text;
pub mod unit;

pub use measure::ConversionFactor;
pub use measure::ConversionOptions;
pub use measure::Measurement;
pub use measure::MeasurementError;
pub use measure::MeasurementParseOptions;
pub use measure::Unit;
pub use measure::UnitDefinition;
#[cfg(feature = "uom")]
pub use measure::UomUnit;
pub use measure::assert_unit_family_valid;
