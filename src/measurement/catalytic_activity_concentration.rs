// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted catalytic activity concentration measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted catalytic activity concentration measurement.
pub type CatalyticActivityConcentration = Measurement<unit::CatalyticActivityConcentration>;
