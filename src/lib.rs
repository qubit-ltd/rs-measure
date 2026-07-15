// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # qubit-measure
//!
//! Persistent typed measurement values with explicit units and `uom` adapters.
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
pub mod unit;

pub use measure::{
    ConversionFactor,
    ConversionOptions,
    Measurement,
    MeasurementError,
    Unit,
    UnitDefinition,
    UomUnit,
    default_conversion_options,
    set_default_conversion_options,
};
pub use rust_decimal::{
    Decimal,
    RoundingStrategy,
};
