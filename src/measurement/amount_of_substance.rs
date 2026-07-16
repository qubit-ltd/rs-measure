// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted amount of substance measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted amount of substance measurement.
pub type AmountOfSubstance = Measurement<unit::AmountOfSubstance>;
