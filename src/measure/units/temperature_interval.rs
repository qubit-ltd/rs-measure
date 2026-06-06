// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted temperature interval measurements.

use super::define_measurement_unit;
use uom::si::f64::TemperatureInterval as UomTemperatureInterval;
use uom::si::temperature_interval::{
    degree_celsius,
    degree_fahrenheit,
    degree_rankine,
    kelvin,
};

define_measurement_unit! {
    /// Units for persisted `uom` temperature interval quantities.
    pub enum TemperatureInterval for UomTemperatureInterval, "temperature interval" {
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
