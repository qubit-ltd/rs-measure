// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted temperature interval measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted temperature interval measurement.
pub type TemperatureInterval = Measurement<unit::TemperatureInterval>;
