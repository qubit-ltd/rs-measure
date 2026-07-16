// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted electrical resistivity measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted electrical resistivity measurement.
pub type ElectricalResistivity = Measurement<unit::ElectricalResistivity>;
