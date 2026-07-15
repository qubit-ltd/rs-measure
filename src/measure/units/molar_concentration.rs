// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted molar concentration measurements.

use crate::define_unit_family;
use uom::si::f64::MolarConcentration as UomMolarConcentration;
use uom::si::molar_concentration::{
    micromole_per_liter,
    millimole_per_liter,
    mole_per_cubic_meter,
    mole_per_liter,
    nanomole_per_liter,
    particle_per_milliliter,
};

define_unit_family! {
    /// Units for persisted `uom` molar concentration quantities.
    pub enum MolarConcentration for "molar_concentration", uom = UomMolarConcentration {
        /// Nanomole per liter (`nmol/L`).
        NanomolePerLiter => { symbol: "nmol/L"; coefficient: 1 / 1000000; uom: nanomole_per_liter; }
        /// Micromole per liter (`µmol/L`).
        MicromolePerLiter => { symbol: "µmol/L"; coefficient: 1 / 1000; aliases: ["umol/L", "μmol/L"]; uom: micromole_per_liter; }
        /// Millimole per liter (`mmol/L`).
        MillimolePerLiter => { symbol: "mmol/L"; coefficient: 1; uom: millimole_per_liter; }
        /// Mole per liter (`mol/L`).
        MolePerLiter => { symbol: "mol/L"; coefficient: 1000; aliases: ["M"]; uom: mole_per_liter; }
        /// Mole per cubic meter (`mol/m³`).
        MolePerCubicMeter => { symbol: "mol/m³"; coefficient: 1; aliases: ["mol/m3", "mol/m^3"]; uom: mole_per_cubic_meter; }
        /// Particle per milliliter (`particle/mL`).
        ParticlePerMilliliter => { symbol: "particle/mL"; coefficient: 1 / 602214076000000000; uom: particle_per_milliliter; }
    }
}
