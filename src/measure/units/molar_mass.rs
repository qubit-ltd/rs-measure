// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted molar mass measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;
#[cfg(feature = "uom")]
use uom::si::f64::MolarMass as UomMolarMass;
#[cfg(feature = "uom")]
use uom::si::molar_mass::{
    gram_per_mole,
    kilogram_per_mole,
    milligram_per_mole,
};

define_unit_family! {
    /// Units for persisted molar mass measurements.
    pub enum MolarMass for "molar_mass" {
        /// Milligram per mole (`mg/mol`).
        MilligramPerMole => { symbol: "mg/mol"; definition: crate::consts::molar_mass::MILLIGRAM_PER_MOLE; }
        /// Gram per mole (`g/mol`).
        GramPerMole => { symbol: "g/mol"; definition: crate::consts::molar_mass::GRAM_PER_MOLE; }
        /// Kilogram per mole (`kg/mol`).
        KilogramPerMole => { symbol: "kg/mol"; definition: crate::consts::molar_mass::KILOGRAM_PER_MOLE; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    MolarMass, UomMolarMass {
        MilligramPerMole => milligram_per_mole;
        GramPerMole => gram_per_mole;
        KilogramPerMole => kilogram_per_mole;
    }
}
