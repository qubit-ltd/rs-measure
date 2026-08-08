// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted temperature interval measurements.

#[cfg(feature = "uom")]
use uom::si::f64::TemperatureInterval as UomTemperatureInterval;
#[cfg(feature = "uom")]
use uom::si::temperature_interval::kelvin;

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;

define_unit_family! {
    /// Units for persisted temperature interval measurements.
    pub enum TemperatureInterval for "temperature_interval" {
        /// Kelvin (`K`).
        Kelvin => { symbol: "K"; definition: crate::consts::temperature_interval::KELVIN; }
        /// Degree Celsius (`°C`).
        Celsius => { symbol: "°C"; definition: crate::consts::temperature_interval::CELSIUS; aliases: ["degC"]; }
        /// Degree Fahrenheit (`°F`).
        Fahrenheit => { symbol: "°F"; definition: crate::consts::temperature_interval::FAHRENHEIT; aliases: ["degF"]; }
        /// Degree Rankine (`°R`).
        Rankine => { symbol: "°R"; definition: crate::consts::temperature_interval::RANKINE; aliases: ["degR"]; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    TemperatureInterval, UomTemperatureInterval {
        base: kelvin;
    }
}
