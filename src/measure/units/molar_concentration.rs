// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted molar concentration measurements.

use super::define_measurement_unit;
use uom::si::f64::MolarConcentration as UomMolarConcentration;
use uom::si::molar_concentration::{
    micromole_per_liter,
    millimole_per_liter,
    mole_per_cubic_meter,
    mole_per_liter,
    nanomole_per_liter,
    particle_per_milliliter,
};

define_measurement_unit! {
    /// Units for persisted `uom` molar concentration quantities.
    pub enum MolarConcentration for UomMolarConcentration, "molar concentration" {
        /// Nanomole per liter (`nmol/L`).
        NanomolePerLiter => "nmol/L", nanomole_per_liter;
        /// Micromole per liter (`µmol/L`).
        MicromolePerLiter => "µmol/L" | "umol/L" | "μmol/L", micromole_per_liter;
        /// Millimole per liter (`mmol/L`).
        MillimolePerLiter => "mmol/L", millimole_per_liter;
        /// Mole per liter (`mol/L`).
        MolePerLiter => "mol/L" | "M", mole_per_liter;
        /// Mole per cubic meter (`mol/m³`).
        MolePerCubicMeter => "mol/m³" | "mol/m3" | "mol/m^3", mole_per_cubic_meter;
        /// Particle per milliliter (`particle/mL`).
        ParticlePerMilliliter => "particle/mL", particle_per_milliliter;
    }
}
