// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted heat capacity measurements.

use super::define_measurement_unit;
use uom::si::f64::HeatCapacity as UomHeatCapacity;
use uom::si::heat_capacity::{
    btu_per_degree_fahrenheit,
    calorie_per_kelvin,
    joule_per_degree_celsius,
    joule_per_kelvin,
    kilojoule_per_kelvin,
};

define_measurement_unit! {
    /// Units for persisted `uom` heat capacity quantities.
    pub enum HeatCapacity for UomHeatCapacity, "heat capacity" {
        /// Joule per kelvin (`J/K`).
        JoulePerKelvin => "J/K", joule_per_kelvin;
        /// Kilojoule per kelvin (`kJ/K`).
        KilojoulePerKelvin => "kJ/K", kilojoule_per_kelvin;
        /// Joule per degree Celsius (`J/°C`).
        JoulePerDegreeCelsius => "J/°C" | "J/degC", joule_per_degree_celsius;
        /// Calorie per kelvin (`cal/K`).
        CaloriePerKelvin => "cal/K", calorie_per_kelvin;
        /// British thermal unit per degree Fahrenheit (`Btu/°F`).
        BritishThermalUnitPerDegreeFahrenheit => "Btu/°F" | "Btu/degF", btu_per_degree_fahrenheit;
    }
}
