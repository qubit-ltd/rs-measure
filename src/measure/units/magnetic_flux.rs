// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted magnetic flux measurements.

#[cfg(feature = "uom")]
use uom::si::f64::MagneticFlux as UomMagneticFlux;
#[cfg(feature = "uom")]
use uom::si::magnetic_flux::weber;

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;

define_unit_family! {
    /// Units for persisted magnetic flux measurements.
    pub enum MagneticFlux for "magnetic_flux" {
        /// Microweber (`µWb`).
        Microweber => { symbol: "µWb"; definition: crate::consts::magnetic_flux::MICROWEBER; aliases: ["uWb", "μWb"]; }
        /// Milliweber (`mWb`).
        Milliweber => { symbol: "mWb"; definition: crate::consts::magnetic_flux::MILLIWEBER; }
        /// Weber (`Wb`).
        Weber => { symbol: "Wb"; definition: crate::consts::magnetic_flux::WEBER; }
        /// Maxwell (`Mx`).
        Maxwell => { symbol: "Mx"; definition: crate::consts::magnetic_flux::MAXWELL; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    MagneticFlux, UomMagneticFlux {
        base: weber;
    }
}
