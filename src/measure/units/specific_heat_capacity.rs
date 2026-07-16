// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted specific heat capacity measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use uom::si::f64::SpecificHeatCapacity as UomSpecificHeatCapacity;
#[cfg(feature = "uom")]
use uom::si::specific_heat_capacity::{
    btu_it_per_pound_degree_fahrenheit,
    calorie_per_gram_kelvin,
    joule_per_gram_degree_celsius,
    joule_per_kilogram_kelvin,
    kilojoule_per_kilogram_kelvin,
};

define_unit_family! {
    /// Units for persisted specific heat capacity measurements.
    pub enum SpecificHeatCapacity for "specific_heat_capacity", uom = UomSpecificHeatCapacity {
        /// Joule per kilogram kelvin (`J/(kg · K)`).
        JoulePerKilogramKelvin => { symbol: "J/(kg · K)"; definition: crate::consts::specific_heat_capacity::JOULE_PER_KILOGRAM_KELVIN; aliases: ["J/(kg*K)"]; uom: joule_per_kilogram_kelvin; }
        /// Kilojoule per kilogram kelvin (`kJ/(kg · K)`).
        KilojoulePerKilogramKelvin => { symbol: "kJ/(kg · K)"; definition: crate::consts::specific_heat_capacity::KILOJOULE_PER_KILOGRAM_KELVIN; aliases: ["kJ/(kg*K)"]; uom: kilojoule_per_kilogram_kelvin; }
        /// Joule per gram degree Celsius (`J/(g · °C)`).
        JoulePerGramDegreeCelsius => { symbol: "J/(g · °C)"; definition: crate::consts::specific_heat_capacity::JOULE_PER_GRAM_DEGREE_CELSIUS; aliases: ["J/(g*degC)"]; uom: joule_per_gram_degree_celsius; }
        /// Thermochemical calorie per gram kelvin with canonical symbol
        /// `cal (th)/(g · K)`.
        ThermochemicalCaloriePerGramKelvin => { symbol: "cal (th)/(g · K)"; definition: crate::consts::specific_heat_capacity::THERMOCHEMICAL_CALORIE_PER_GRAM_KELVIN; aliases: ["cal/(g · K)", "cal/(g*K)"]; uom: calorie_per_gram_kelvin; }
        /// International Table British thermal unit per pound degree
        /// Fahrenheit with canonical symbol `Btu (IT)/(lb · °F)`.
        BritishThermalUnitInternationalTablePerPoundDegreeFahrenheit => { symbol: "Btu (IT)/(lb · °F)"; definition: crate::consts::specific_heat_capacity::BRITISH_THERMAL_UNIT_INTERNATIONAL_TABLE_PER_POUND_DEGREE_FAHRENHEIT; aliases: ["Btu/(lb · °F)", "Btu/(lb*degF)"]; uom: btu_it_per_pound_degree_fahrenheit; }
    }
}
