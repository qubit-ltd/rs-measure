// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted electrical conductivity measurements.

use super::define_measurement_unit;
use uom::si::electrical_conductivity::{
    siemens_per_centimeter,
    siemens_per_meter,
};
use uom::si::f64::ElectricalConductivity as UomElectricalConductivity;

define_measurement_unit! {
    /// Units for persisted `uom` electrical conductivity quantities.
    pub enum ElectricalConductivity for UomElectricalConductivity, "electrical conductivity" {
        /// Siemens per meter (`S/m`).
        SiemensPerMeter => "S/m", siemens_per_meter;
        /// Siemens per centimeter (`S/cm`).
        SiemensPerCentimeter => "S/cm", siemens_per_centimeter;
    }
}
