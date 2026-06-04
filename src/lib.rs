/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # qubit-measure
//!
//! Persistent typed measurement values with explicit units and `uom` adapters.

#![deny(missing_docs)]
#![deny(unsafe_op_in_unsafe_fn)]

/// Measurement values, typed unit families, and conversion types.
pub mod measure;

pub use measure::{
    AreaMeasurement,
    AreaUnit,
    LengthMeasurement,
    LengthUnit,
    MassMeasurement,
    MassUnit,
    Measurement,
    MeasurementError,
    MeasurementUnit,
    TimeMeasurement,
    TimeUnit,
    VolumeMeasurement,
    VolumeUnit,
};
