/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Units for persisted thermodynamic temperature measurements.

use super::define_measurement_unit;
use uom::si::f64::ThermodynamicTemperature as UomTemperature;
use uom::si::thermodynamic_temperature::{
    degree_celsius,
    degree_fahrenheit,
    degree_rankine,
    kelvin,
};

define_measurement_unit! {
    /// Units for persisted `uom` thermodynamic temperature quantities.
    pub enum Temperature for UomTemperature, "temperature" {
        /// Kelvin (`K`).
        Kelvin => "K", kelvin;
        /// Degree Celsius (`°C`).
        Celsius => "°C" | "degC", degree_celsius;
        /// Degree Fahrenheit (`°F`).
        Fahrenheit => "°F" | "degF", degree_fahrenheit;
        /// Degree Rankine (`°R`).
        Rankine => "°R" | "degR", degree_rankine;
    }
}
