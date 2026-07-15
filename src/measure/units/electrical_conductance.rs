// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted electrical conductance measurements.

use crate::define_unit_family;
use uom::si::electrical_conductance::{
    microsiemens,
    millisiemens,
    siemens,
};
use uom::si::f64::ElectricalConductance as UomElectricalConductance;

define_unit_family! {
    /// Units for persisted `uom` electrical conductance quantities.
    pub enum ElectricalConductance for "electrical_conductance", uom = UomElectricalConductance {
        /// Microsiemens (`µS`).
        Microsiemens => { symbol: "µS"; definition: crate::consts::electrical_conductance::MICROSIEMENS; aliases: ["uS", "μS"]; uom: microsiemens; }
        /// Millisiemens (`mS`).
        Millisiemens => { symbol: "mS"; definition: crate::consts::electrical_conductance::MILLISIEMENS; uom: millisiemens; }
        /// Siemens (`S`).
        Siemens => { symbol: "S"; definition: crate::consts::electrical_conductance::SIEMENS; uom: siemens; }
    }
}
