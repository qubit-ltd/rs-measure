// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted magnetic flux measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use uom::si::f64::MagneticFlux as UomMagneticFlux;
#[cfg(feature = "uom")]
use uom::si::magnetic_flux::{
    maxwell,
    microweber,
    milliweber,
    weber,
};

define_unit_family! {
    /// Units for persisted magnetic flux measurements.
    pub enum MagneticFlux for "magnetic_flux", uom = UomMagneticFlux {
        /// Microweber (`µWb`).
        Microweber => { symbol: "µWb"; definition: crate::consts::magnetic_flux::MICROWEBER; aliases: ["uWb", "μWb"]; uom: microweber; }
        /// Milliweber (`mWb`).
        Milliweber => { symbol: "mWb"; definition: crate::consts::magnetic_flux::MILLIWEBER; uom: milliweber; }
        /// Weber (`Wb`).
        Weber => { symbol: "Wb"; definition: crate::consts::magnetic_flux::WEBER; uom: weber; }
        /// Maxwell (`Mx`).
        Maxwell => { symbol: "Mx"; definition: crate::consts::magnetic_flux::MAXWELL; uom: maxwell; }
    }
}
