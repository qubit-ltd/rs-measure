// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted magnetic flux density measurements.

use crate::define_unit_family;
use uom::si::f64::MagneticFluxDensity as UomMagneticFluxDensity;
use uom::si::magnetic_flux_density::{
    gauss,
    microtesla,
    millitesla,
    nanotesla,
    tesla,
};

define_unit_family! {
    /// Units for persisted `uom` magnetic flux density quantities.
    pub enum MagneticFluxDensity for "magnetic_flux_density", uom = UomMagneticFluxDensity {
        /// Nanotesla (`nT`).
        Nanotesla => { symbol: "nT"; definition: crate::consts::magnetic_flux_density::NANOTESLA; uom: nanotesla; }
        /// Microtesla (`µT`).
        Microtesla => { symbol: "µT"; definition: crate::consts::magnetic_flux_density::MICROTESLA; aliases: ["uT", "μT"]; uom: microtesla; }
        /// Millitesla (`mT`).
        Millitesla => { symbol: "mT"; definition: crate::consts::magnetic_flux_density::MILLITESLA; uom: millitesla; }
        /// Tesla (`T`).
        Tesla => { symbol: "T"; definition: crate::consts::magnetic_flux_density::TESLA; uom: tesla; }
        /// Gauss (`G`).
        Gauss => { symbol: "G"; definition: crate::consts::magnetic_flux_density::GAUSS; uom: gauss; }
    }
}
