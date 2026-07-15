// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted magnetic flux measurements.

use crate::define_unit_family;
use uom::si::f64::MagneticFlux as UomMagneticFlux;
use uom::si::magnetic_flux::{
    maxwell,
    microweber,
    milliweber,
    weber,
};

define_unit_family! {
    /// Units for persisted `uom` magnetic flux quantities.
    pub enum MagneticFlux for "magnetic_flux", uom = UomMagneticFlux {
        /// Microweber (`µWb`).
        Microweber => { symbol: "µWb"; coefficient: 1 / 1000000; aliases: ["uWb", "μWb"]; uom: microweber; }
        /// Milliweber (`mWb`).
        Milliweber => { symbol: "mWb"; coefficient: 1 / 1000; uom: milliweber; }
        /// Weber (`Wb`).
        Weber => { symbol: "Wb"; coefficient: 1; uom: weber; }
        /// Maxwell (`Mx`).
        Maxwell => { symbol: "Mx"; coefficient: 1 / 100000000; uom: maxwell; }
    }
}
