// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted frequency measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted frequency measurement.
pub type Frequency = Measurement<unit::Frequency>;
