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

#![deny(missing_docs)]
#![deny(unsafe_op_in_unsafe_fn)]

mod measure;
pub mod measurement;
pub mod unit;

pub use measure::{
    Measurement,
    MeasurementError,
    Unit,
};
