// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted electrical conductivity measurements.

use crate::define_unit_family;
use uom::si::electrical_conductivity::{
    siemens_per_centimeter,
    siemens_per_meter,
};
use uom::si::f64::ElectricalConductivity as UomElectricalConductivity;

define_unit_family! {
    /// Units for persisted `uom` electrical conductivity quantities.
    pub enum ElectricalConductivity for "electrical_conductivity", uom = UomElectricalConductivity {
        /// Siemens per meter (`S/m`).
        SiemensPerMeter => { symbol: "S/m"; definition: crate::consts::electrical_conductivity::SIEMENS_PER_METER; uom: siemens_per_meter; }
        /// Siemens per centimeter (`S/cm`).
        SiemensPerCentimeter => { symbol: "S/cm"; definition: crate::consts::electrical_conductivity::SIEMENS_PER_CENTIMETER; uom: siemens_per_centimeter; }
    }
}
