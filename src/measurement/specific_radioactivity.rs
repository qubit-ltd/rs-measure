// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted specific radioactivity measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted specific radioactivity measurement.
pub type SpecificRadioactivity = Measurement<unit::SpecificRadioactivity>;
