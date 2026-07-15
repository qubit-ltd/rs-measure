// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted temperature interval measurements.

use crate::define_unit_family;
use uom::si::f64::TemperatureInterval as UomTemperatureInterval;
use uom::si::temperature_interval::{
    degree_celsius,
    degree_fahrenheit,
    degree_rankine,
    kelvin,
};

define_unit_family! {
    /// Units for persisted `uom` temperature interval quantities.
    pub enum TemperatureInterval for "temperature_interval", uom = UomTemperatureInterval {
        /// Kelvin (`K`).
        Kelvin => { symbol: "K"; coefficient: 1; uom: kelvin; }
        /// Degree Celsius (`°C`).
        Celsius => { symbol: "°C"; coefficient: 1; aliases: ["degC"]; uom: degree_celsius; }
        /// Degree Fahrenheit (`°F`).
        Fahrenheit => { symbol: "°F"; coefficient: 5 / 9; aliases: ["degF"]; uom: degree_fahrenheit; }
        /// Degree Rankine (`°R`).
        Rankine => { symbol: "°R"; coefficient: 5 / 9; aliases: ["degR"]; uom: degree_rankine; }
    }
}
