// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted luminous intensity measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted luminous intensity measurement.
pub type LuminousIntensity = Measurement<unit::LuminousIntensity>;
