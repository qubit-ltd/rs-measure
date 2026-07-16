// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted mass rate measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted mass rate measurement.
pub type MassRate = Measurement<unit::MassRate>;
