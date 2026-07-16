// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted molar concentration measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted molar concentration measurement.
pub type MolarConcentration = Measurement<unit::MolarConcentration>;
