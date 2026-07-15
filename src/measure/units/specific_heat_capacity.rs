// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted specific heat capacity measurements.

use crate::define_unit_family;
use uom::si::f64::SpecificHeatCapacity as UomSpecificHeatCapacity;
use uom::si::specific_heat_capacity::{
    btu_per_pound_degree_fahrenheit,
    calorie_per_gram_kelvin,
    joule_per_gram_degree_celsius,
    joule_per_kilogram_kelvin,
    kilojoule_per_kilogram_kelvin,
};

define_unit_family! {
    /// Units for persisted `uom` specific heat capacity quantities.
    pub enum SpecificHeatCapacity for "specific_heat_capacity", uom = UomSpecificHeatCapacity {
        /// Joule per kilogram kelvin (`J/(kg · K)`).
        JoulePerKilogramKelvin => { symbol: "J/(kg · K)"; coefficient: 1; aliases: ["J/(kg*K)"]; uom: joule_per_kilogram_kelvin; }
        /// Kilojoule per kilogram kelvin (`kJ/(kg · K)`).
        KilojoulePerKilogramKelvin => { symbol: "kJ/(kg · K)"; coefficient: 1000; aliases: ["kJ/(kg*K)"]; uom: kilojoule_per_kilogram_kelvin; }
        /// Joule per gram degree Celsius (`J/(g · °C)`).
        JoulePerGramDegreeCelsius => { symbol: "J/(g · °C)"; coefficient: 1000; aliases: ["J/(g*degC)"]; uom: joule_per_gram_degree_celsius; }
        /// Calorie per gram kelvin (`cal/(g · K)`).
        ThermochemicalCaloriePerGramKelvin => { symbol: "cal (th)/(g · K)"; coefficient: 4184; aliases: ["cal/(g · K)", "cal/(g*K)"]; uom: calorie_per_gram_kelvin; }
        /// British thermal unit per pound degree Fahrenheit (`Btu/(lb · °F)`).
        BritishThermalUnitInternationalTablePerPoundDegreeFahrenheit => { symbol: "Btu (IT)/(lb · °F)"; coefficient: 189910080000 / 45359237; aliases: ["Btu/(lb · °F)", "Btu/(lb*degF)"]; uom: btu_per_pound_degree_fahrenheit; }
    }
}
