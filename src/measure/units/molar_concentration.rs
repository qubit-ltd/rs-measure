// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted molar concentration measurements.

#[cfg(feature = "uom")]
use uom::si::f64::MolarConcentration as UomMolarConcentration;
#[cfg(feature = "uom")]
use uom::si::molar_concentration::mole_per_cubic_meter;

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;

define_unit_family! {
    /// Units for persisted molar concentration measurements.
    pub enum MolarConcentration for "molar_concentration" {
        /// Nanomole per liter (`nmol/L`).
        NanomolePerLiter => { symbol: "nmol/L"; definition: crate::consts::molar_concentration::NANOMOLE_PER_LITER; }
        /// Micromole per liter (`µmol/L`).
        MicromolePerLiter => { symbol: "µmol/L"; definition: crate::consts::molar_concentration::MICROMOLE_PER_LITER; aliases: ["umol/L", "μmol/L"]; }
        /// Millimole per liter (`mmol/L`).
        MillimolePerLiter => { symbol: "mmol/L"; definition: crate::consts::molar_concentration::MILLIMOLE_PER_LITER; }
        /// Mole per liter (`mol/L`).
        MolePerLiter => { symbol: "mol/L"; definition: crate::consts::molar_concentration::MOLE_PER_LITER; aliases: ["M"]; }
        /// Mole per cubic meter (`mol/m³`).
        MolePerCubicMeter => { symbol: "mol/m³"; definition: crate::consts::molar_concentration::MOLE_PER_CUBIC_METER; aliases: ["mol/m3", "mol/m^3"]; }
        /// Particle per milliliter (`particle/mL`).
        ParticlePerMilliliter => { symbol: "particle/mL"; definition: crate::consts::molar_concentration::PARTICLE_PER_MILLILITER; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    MolarConcentration, UomMolarConcentration {
        base: mole_per_cubic_meter;
    }
}
