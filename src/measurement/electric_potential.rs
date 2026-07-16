// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted electric potential measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted electric potential measurement.
///
/// This is the SI quantity commonly called voltage.
pub type ElectricPotential = Measurement<unit::ElectricPotential>;
