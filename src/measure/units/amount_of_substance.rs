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
        Micromole => { symbol: "µmol"; coefficient: 1 / 1000000; aliases: ["umol", "μmol"]; uom: micromole; }
        /// Millimole (`mmol`).
        Millimole => { symbol: "mmol"; coefficient: 1 / 1000; uom: millimole; }
        /// Mole (`mol`).
        Mole => { symbol: "mol"; coefficient: 1; uom: mole; }
        /// Kilomole (`kmol`).
        Kilomole => { symbol: "kmol"; coefficient: 1000; uom: kilomole; }
        /// Particle (`particle`).
        Particle => { symbol: "particle"; coefficient: 1 / 602214076000000000000000; uom: particle; }
    }
}
