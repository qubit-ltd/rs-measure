// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted amount of substance measurements.

use crate::define_unit_family;
use uom::si::amount_of_substance::{
    kilomole,
    micromole,
    millimole,
    mole,
    particle,
};
use uom::si::f64::AmountOfSubstance as UomAmountOfSubstance;

define_unit_family! {
    /// Units for persisted `uom` amount of substance quantities.
    pub enum AmountOfSubstance for "amount_of_substance", uom = UomAmountOfSubstance {
        /// Micromole (`µmol`).
        Micromole => { symbol: "µmol"; definition: crate::consts::amount_of_substance::MICROMOLE; aliases: ["umol", "μmol"]; uom: micromole; }
        /// Millimole (`mmol`).
        Millimole => { symbol: "mmol"; definition: crate::consts::amount_of_substance::MILLIMOLE; uom: millimole; }
        /// Mole (`mol`).
        Mole => { symbol: "mol"; definition: crate::consts::amount_of_substance::MOLE; uom: mole; }
        /// Kilomole (`kmol`).
        Kilomole => { symbol: "kmol"; definition: crate::consts::amount_of_substance::KILOMOLE; uom: kilomole; }
        /// Particle (`particle`).
        Particle => { symbol: "particle"; definition: crate::consts::amount_of_substance::PARTICLE; uom: particle; }
    }
}
