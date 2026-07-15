// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted heat capacity measurements.

use crate::define_unit_family;
use uom::si::f64::HeatCapacity as UomHeatCapacity;
use uom::si::heat_capacity::{
    btu_per_degree_fahrenheit,
    calorie_per_kelvin,
    joule_per_degree_celsius,
    joule_per_kelvin,
    kilojoule_per_kelvin,
};

define_unit_family! {
    /// Units for persisted `uom` heat capacity quantities.
    pub enum HeatCapacity for "heat_capacity", uom = UomHeatCapacity {
        /// Joule per kelvin (`J/K`).
        JoulePerKelvin => { symbol: "J/K"; coefficient: 1; uom: joule_per_kelvin; }
        /// Kilojoule per kelvin (`kJ/K`).
        KilojoulePerKelvin => { symbol: "kJ/K"; coefficient: 1000; uom: kilojoule_per_kelvin; }
        /// Joule per degree Celsius (`J/°C`).
        JoulePerDegreeCelsius => { symbol: "J/°C"; coefficient: 1; aliases: ["J/degC"]; uom: joule_per_degree_celsius; }
        /// Calorie per kelvin (`cal/K`).
        ThermochemicalCaloriePerKelvin => { symbol: "cal (th)/K"; coefficient: 523 / 125; aliases: ["cal/K"]; uom: calorie_per_kelvin; }
        /// British thermal unit per degree Fahrenheit (`Btu/°F`).
        BritishThermalUnitInternationalTablePerDegreeFahrenheit => { symbol: "Btu (IT)/°F"; coefficient: 1186938 / 625; aliases: ["Btu/°F", "Btu/degF"]; uom: btu_per_degree_fahrenheit; }
    }
}
