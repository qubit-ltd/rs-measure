/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Units for persisted thermal conductivity measurements.

use super::define_measurement_unit;
use uom::si::f64::ThermalConductivity as UomThermalConductivity;
use uom::si::thermal_conductivity::{
    kilowatt_per_meter_kelvin,
    milliwatt_per_meter_kelvin,
    watt_per_meter_degree_celsius,
    watt_per_meter_kelvin,
};

define_measurement_unit! {
    /// Units for persisted `uom` thermal conductivity quantities.
    pub enum ThermalConductivity for UomThermalConductivity, "thermal conductivity" {
        /// Milliwatt per meter kelvin (`mW/(m · K)`).
        MilliwattPerMeterKelvin => "mW/(m · K)" | "mW/(m*K)", milliwatt_per_meter_kelvin;
        /// Watt per meter kelvin (`W/(m · K)`).
        WattPerMeterKelvin => "W/(m · K)" | "W/(m*K)", watt_per_meter_kelvin;
        /// Kilowatt per meter kelvin (`kW/(m · K)`).
        KilowattPerMeterKelvin => "kW/(m · K)" | "kW/(m*K)", kilowatt_per_meter_kelvin;
        /// Watt per meter degree Celsius (`W/(m · °C)`).
        WattPerMeterDegreeCelsius => "W/(m · °C)" | "W/(m*degC)", watt_per_meter_degree_celsius;
    }
}
