// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted magnetic flux measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted magnetic flux measurement.
pub type MagneticFlux = Measurement<unit::MagneticFlux>;
