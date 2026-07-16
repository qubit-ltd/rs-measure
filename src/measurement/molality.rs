// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted molality measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted molality measurement.
pub type Molality = Measurement<unit::Molality>;
