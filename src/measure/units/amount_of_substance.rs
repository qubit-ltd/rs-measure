// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted amount of substance measurements.

use super::define_measurement_unit;
use uom::si::amount_of_substance::{
    kilomole,
    micromole,
    millimole,
    mole,
    particle,
};
use uom::si::f64::AmountOfSubstance as UomAmountOfSubstance;

define_measurement_unit! {
    /// Units for persisted `uom` amount of substance quantities.
    pub enum AmountOfSubstance for UomAmountOfSubstance, "amount of substance" {
        /// Micromole (`µmol`).
        Micromole => "µmol" | "umol" | "μmol", micromole;
        /// Millimole (`mmol`).
        Millimole => "mmol", millimole;
        /// Mole (`mol`).
        Mole => "mol", mole;
        /// Kilomole (`kmol`).
        Kilomole => "kmol", kilomole;
        /// Particle (`particle`).
        Particle => "particle", particle;
    }
}
