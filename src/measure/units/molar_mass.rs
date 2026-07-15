// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted molar mass measurements.

use crate::define_unit_family;
use uom::si::f64::MolarMass as UomMolarMass;
use uom::si::molar_mass::{
    gram_per_mole,
    kilogram_per_mole,
    milligram_per_mole,
};

define_unit_family! {
    /// Units for persisted `uom` molar mass quantities.
    pub enum MolarMass for "molar_mass", uom = UomMolarMass {
        /// Milligram per mole (`mg/mol`).
        MilligramPerMole => { symbol: "mg/mol"; definition: crate::consts::molar_mass::MILLIGRAM_PER_MOLE; uom: milligram_per_mole; }
        /// Gram per mole (`g/mol`).
        GramPerMole => { symbol: "g/mol"; definition: crate::consts::molar_mass::GRAM_PER_MOLE; uom: gram_per_mole; }
        /// Kilogram per mole (`kg/mol`).
        KilogramPerMole => { symbol: "kg/mol"; definition: crate::consts::molar_mass::KILOGRAM_PER_MOLE; uom: kilogram_per_mole; }
    }
}
