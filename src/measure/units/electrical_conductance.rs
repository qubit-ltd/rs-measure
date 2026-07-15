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
        Microsiemens => { symbol: "µS"; coefficient: 1 / 1000000; aliases: ["uS", "μS"]; uom: microsiemens; }
        /// Millisiemens (`mS`).
        Millisiemens => { symbol: "mS"; coefficient: 1 / 1000; uom: millisiemens; }
        /// Siemens (`S`).
        Siemens => { symbol: "S"; coefficient: 1; uom: siemens; }
    }
}
