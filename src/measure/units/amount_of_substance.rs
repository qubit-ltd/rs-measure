// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted amount of substance measurements.

#[cfg(feature = "uom")]
use uom::si::amount_of_substance::mole;
#[cfg(feature = "uom")]
use uom::si::f64::AmountOfSubstance as UomAmountOfSubstance;

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;

define_unit_family! {
    /// Units for persisted amount of substance measurements.
    pub enum AmountOfSubstance for "amount_of_substance" {
        /// Micromole (`µmol`).
        Micromole => { symbol: "µmol"; definition: crate::consts::amount_of_substance::MICROMOLE; aliases: ["umol", "μmol"]; }
        /// Millimole (`mmol`).
        Millimole => { symbol: "mmol"; definition: crate::consts::amount_of_substance::MILLIMOLE; }
        /// Mole (`mol`).
        Mole => { symbol: "mol"; definition: crate::consts::amount_of_substance::MOLE; }
        /// Kilomole (`kmol`).
        Kilomole => { symbol: "kmol"; definition: crate::consts::amount_of_substance::KILOMOLE; }
        /// Particle (`particle`).
        Particle => { symbol: "particle"; definition: crate::consts::amount_of_substance::PARTICLE; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    AmountOfSubstance, UomAmountOfSubstance {
        base: mole;
    }
}
