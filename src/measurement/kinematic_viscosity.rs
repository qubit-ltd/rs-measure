// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted kinematic viscosity measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted kinematic viscosity measurement.
pub type KinematicViscosity = Measurement<unit::KinematicViscosity>;
