// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted heat flux density measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted heat flux density measurement.
pub type HeatFluxDensity = Measurement<unit::HeatFluxDensity>;
