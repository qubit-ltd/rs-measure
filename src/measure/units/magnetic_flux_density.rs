// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted magnetic flux density measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;
#[cfg(feature = "uom")]
use uom::si::f64::MagneticFluxDensity as UomMagneticFluxDensity;
#[cfg(feature = "uom")]
use uom::si::magnetic_flux_density::tesla;

define_unit_family! {
    /// Units for persisted magnetic flux density measurements.
    pub enum MagneticFluxDensity for "magnetic_flux_density" {
        /// Nanotesla (`nT`).
        Nanotesla => { symbol: "nT"; definition: crate::consts::magnetic_flux_density::NANOTESLA; }
        /// Microtesla (`µT`).
        Microtesla => { symbol: "µT"; definition: crate::consts::magnetic_flux_density::MICROTESLA; aliases: ["uT", "μT"]; }
        /// Millitesla (`mT`).
        Millitesla => { symbol: "mT"; definition: crate::consts::magnetic_flux_density::MILLITESLA; }
        /// Tesla (`T`).
        Tesla => { symbol: "T"; definition: crate::consts::magnetic_flux_density::TESLA; }
        /// Gauss (`G`).
        Gauss => { symbol: "G"; definition: crate::consts::magnetic_flux_density::GAUSS; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    MagneticFluxDensity, UomMagneticFluxDensity {
        base: tesla;
    }
}
