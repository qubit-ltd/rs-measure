// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted magnetic flux density measurements.

use super::define_measurement_unit;
use uom::si::f64::MagneticFluxDensity as UomMagneticFluxDensity;
use uom::si::magnetic_flux_density::{
    gauss,
    microtesla,
    millitesla,
    nanotesla,
    tesla,
};

define_measurement_unit! {
    /// Units for persisted `uom` magnetic flux density quantities.
    pub enum MagneticFluxDensity for UomMagneticFluxDensity, "magnetic flux density" {
        /// Nanotesla (`nT`).
        Nanotesla => "nT", nanotesla;
        /// Microtesla (`µT`).
        Microtesla => "µT" | "uT" | "μT", microtesla;
        /// Millitesla (`mT`).
        Millitesla => "mT", millitesla;
        /// Tesla (`T`).
        Tesla => "T", tesla;
        /// Gauss (`G`).
        Gauss => "G", gauss;
    }
}
