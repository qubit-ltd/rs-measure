// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted electrical conductivity measurements.

#[cfg(feature = "uom")]
use uom::si::electrical_conductivity::siemens_per_meter;
#[cfg(feature = "uom")]
use uom::si::f64::ElectricalConductivity as UomElectricalConductivity;

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;

define_unit_family! {
    /// Units for persisted electrical conductivity measurements.
    pub enum ElectricalConductivity for "electrical_conductivity" {
        /// Siemens per meter (`S/m`).
        SiemensPerMeter => { symbol: "S/m"; definition: crate::consts::electrical_conductivity::SIEMENS_PER_METER; }
        /// Siemens per centimeter (`S/cm`).
        SiemensPerCentimeter => { symbol: "S/cm"; definition: crate::consts::electrical_conductivity::SIEMENS_PER_CENTIMETER; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    ElectricalConductivity, UomElectricalConductivity {
        base: siemens_per_meter;
    }
}
