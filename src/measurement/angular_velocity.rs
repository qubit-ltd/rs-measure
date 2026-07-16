// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted angular velocity measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted angular velocity measurement.
pub type AngularVelocity = Measurement<unit::AngularVelocity>;
