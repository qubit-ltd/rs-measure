// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted molar mass measurements.

use super::define_measurement_unit;
use uom::si::f64::MolarMass as UomMolarMass;
use uom::si::molar_mass::{
    gram_per_mole,
    kilogram_per_mole,
    milligram_per_mole,
};

define_measurement_unit! {
    /// Units for persisted `uom` molar mass quantities.
    pub enum MolarMass for UomMolarMass, "molar mass" {
        /// Milligram per mole (`mg/mol`).
        MilligramPerMole => "mg/mol", milligram_per_mole;
        /// Gram per mole (`g/mol`).
        GramPerMole => "g/mol", gram_per_mole;
        /// Kilogram per mole (`kg/mol`).
        KilogramPerMole => "kg/mol", kilogram_per_mole;
    }
}
