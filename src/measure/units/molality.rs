// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted molality measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use uom::si::f64::Molality as UomMolality;
#[cfg(feature = "uom")]
use uom::si::molality::mole_per_kilogram;

define_unit_family! {
    /// Units for persisted molality measurements.
    pub enum Molality for "molality", uom = UomMolality {
        /// Mole per kilogram (`mol/kg`).
        MolePerKilogram => { symbol: "mol/kg"; definition: crate::consts::molality::MOLE_PER_KILOGRAM; uom: mole_per_kilogram; }
    }
}
