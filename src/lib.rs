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
