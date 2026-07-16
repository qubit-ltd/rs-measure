// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted pressure measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted pressure measurement.
pub type Pressure = Measurement<unit::Pressure>;
