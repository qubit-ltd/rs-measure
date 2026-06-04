/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Units for persisted specific heat capacity measurements.

use super::define_measurement_unit;
use uom::si::f64::SpecificHeatCapacity as UomSpecificHeatCapacity;
use uom::si::specific_heat_capacity::{
    btu_per_pound_degree_fahrenheit,
    calorie_per_gram_kelvin,
    joule_per_gram_degree_celsius,
    joule_per_kilogram_kelvin,
    kilojoule_per_kilogram_kelvin,
};

define_measurement_unit! {
    /// Units for persisted `uom` specific heat capacity quantities.
    pub enum SpecificHeatCapacity for UomSpecificHeatCapacity, "specific heat capacity" {
        /// Joule per kilogram kelvin (`J/(kg · K)`).
        JoulePerKilogramKelvin => "J/(kg · K)" | "J/(kg*K)", joule_per_kilogram_kelvin;
        /// Kilojoule per kilogram kelvin (`kJ/(kg · K)`).
        KilojoulePerKilogramKelvin => "kJ/(kg · K)" | "kJ/(kg*K)", kilojoule_per_kilogram_kelvin;
        /// Joule per gram degree Celsius (`J/(g · °C)`).
        JoulePerGramDegreeCelsius => "J/(g · °C)" | "J/(g*degC)", joule_per_gram_degree_celsius;
        /// Calorie per gram kelvin (`cal/(g · K)`).
        CaloriePerGramKelvin => "cal/(g · K)" | "cal/(g*K)", calorie_per_gram_kelvin;
        /// British thermal unit per pound degree Fahrenheit (`Btu/(lb · °F)`).
        BritishThermalUnitPerPoundDegreeFahrenheit => "Btu/(lb · °F)" | "Btu/(lb*degF)", btu_per_pound_degree_fahrenheit;
    }
}
