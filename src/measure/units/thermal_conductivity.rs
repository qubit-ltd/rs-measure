// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted thermal conductivity measurements.

use crate::define_unit_family;
use uom::si::f64::ThermalConductivity as UomThermalConductivity;
use uom::si::thermal_conductivity::{
    kilowatt_per_meter_kelvin,
    milliwatt_per_meter_kelvin,
    watt_per_meter_degree_celsius,
    watt_per_meter_kelvin,
};

define_unit_family! {
    /// Units for persisted `uom` thermal conductivity quantities.
    pub enum ThermalConductivity for "thermal_conductivity", uom = UomThermalConductivity {
        /// Milliwatt per meter kelvin (`mW/(m · K)`).
        MilliwattPerMeterKelvin => { symbol: "mW/(m · K)"; coefficient: 1 / 1000; aliases: ["mW/(m*K)"]; uom: milliwatt_per_meter_kelvin; }
        /// Watt per meter kelvin (`W/(m · K)`).
        WattPerMeterKelvin => { symbol: "W/(m · K)"; coefficient: 1; aliases: ["W/(m*K)"]; uom: watt_per_meter_kelvin; }
        /// Kilowatt per meter kelvin (`kW/(m · K)`).
        KilowattPerMeterKelvin => { symbol: "kW/(m · K)"; coefficient: 1000; aliases: ["kW/(m*K)"]; uom: kilowatt_per_meter_kelvin; }
        /// Watt per meter degree Celsius (`W/(m · °C)`).
        WattPerMeterDegreeCelsius => { symbol: "W/(m · °C)"; coefficient: 1; aliases: ["W/(m*degC)"]; uom: watt_per_meter_degree_celsius; }
    }
}
