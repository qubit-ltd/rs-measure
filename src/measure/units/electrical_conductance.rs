// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted electrical conductance measurements.

#[cfg(feature = "uom")]
use uom::si::electrical_conductance::siemens;
#[cfg(feature = "uom")]
use uom::si::f64::ElectricalConductance as UomElectricalConductance;

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;

define_unit_family! {
    /// Units for persisted electrical conductance measurements.
    pub enum ElectricalConductance for "electrical_conductance" {
        /// Microsiemens (`µS`).
        Microsiemens => { symbol: "µS"; definition: crate::consts::electrical_conductance::MICROSIEMENS; aliases: ["uS", "μS"]; }
        /// Millisiemens (`mS`).
        Millisiemens => { symbol: "mS"; definition: crate::consts::electrical_conductance::MILLISIEMENS; }
        /// Siemens (`S`).
        Siemens => { symbol: "S"; definition: crate::consts::electrical_conductance::SIEMENS; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    ElectricalConductance, UomElectricalConductance {
        base: siemens;
    }
}
