/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Units for persisted magnetic flux measurements.

use super::define_measurement_unit;
use uom::si::f64::MagneticFlux as UomMagneticFlux;
use uom::si::magnetic_flux::{
    maxwell,
    microweber,
    milliweber,
    weber,
};

define_measurement_unit! {
    /// Units for persisted `uom` magnetic flux quantities.
    pub enum MagneticFlux for UomMagneticFlux, "magnetic flux" {
        /// Microweber (`µWb`).
        Microweber => "µWb" | "uWb" | "μWb", microweber;
        /// Milliweber (`mWb`).
        Milliweber => "mWb", milliweber;
        /// Weber (`Wb`).
        Weber => "Wb", weber;
        /// Maxwell (`Mx`).
        Maxwell => "Mx", maxwell;
    }
}
