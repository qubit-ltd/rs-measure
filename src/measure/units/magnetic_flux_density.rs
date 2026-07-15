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
        Nanotesla => { symbol: "nT"; coefficient: 1 / 1000000000; uom: nanotesla; }
        /// Microtesla (`µT`).
        Microtesla => { symbol: "µT"; coefficient: 1 / 1000000; aliases: ["uT", "μT"]; uom: microtesla; }
        /// Millitesla (`mT`).
        Millitesla => { symbol: "mT"; coefficient: 1 / 1000; uom: millitesla; }
        /// Tesla (`T`).
        Tesla => { symbol: "T"; coefficient: 1; uom: tesla; }
        /// Gauss (`G`).
        Gauss => { symbol: "G"; coefficient: 1 / 10000; uom: gauss; }
    }
}
