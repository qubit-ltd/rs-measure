// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted voltage measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted voltage measurement.
///
/// This is an ergonomic alias for electric potential.
pub type Voltage = Measurement<unit::ElectricPotential>;
