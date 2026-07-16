// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted acceleration measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted acceleration measurement.
pub type Acceleration = Measurement<unit::Acceleration>;
