// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted thermodynamic temperature measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use uom::si::f64::ThermodynamicTemperature as UomTemperature;
#[cfg(feature = "uom")]
use uom::si::thermodynamic_temperature::{
    degree_celsius,
    degree_fahrenheit,
    degree_rankine,
    kelvin,
};

define_unit_family! {
    /// Units for persisted thermodynamic temperature measurements.
    pub enum Temperature for "temperature", uom = UomTemperature {
        /// Kelvin (`K`).
        Kelvin => { symbol: "K"; definition: crate::consts::temperature::KELVIN; uom: kelvin; }
        /// Degree Celsius (`°C`).
        Celsius => { symbol: "°C"; definition: crate::consts::temperature::CELSIUS; aliases: ["degC"]; uom: degree_celsius; }
        /// Degree Fahrenheit (`°F`).
        Fahrenheit => { symbol: "°F"; definition: crate::consts::temperature::FAHRENHEIT; aliases: ["degF"]; uom: degree_fahrenheit; }
        /// Degree Rankine (`°R`).
        Rankine => { symbol: "°R"; definition: crate::consts::temperature::RANKINE; aliases: ["degR"]; uom: degree_rankine; }
    }
}
