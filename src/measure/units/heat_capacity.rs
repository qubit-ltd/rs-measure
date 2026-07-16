// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted heat capacity measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use uom::si::f64::HeatCapacity as UomHeatCapacity;
#[cfg(feature = "uom")]
use uom::si::heat_capacity::{
    btu_it_per_degree_fahrenheit,
    calorie_per_kelvin,
    joule_per_degree_celsius,
    joule_per_kelvin,
    kilojoule_per_kelvin,
};

define_unit_family! {
    /// Units for persisted heat capacity measurements.
    pub enum HeatCapacity for "heat_capacity", uom = UomHeatCapacity {
        /// Joule per kelvin (`J/K`).
        JoulePerKelvin => { symbol: "J/K"; definition: crate::consts::heat_capacity::JOULE_PER_KELVIN; uom: joule_per_kelvin; }
        /// Kilojoule per kelvin (`kJ/K`).
        KilojoulePerKelvin => { symbol: "kJ/K"; definition: crate::consts::heat_capacity::KILOJOULE_PER_KELVIN; uom: kilojoule_per_kelvin; }
        /// Joule per degree Celsius (`J/°C`).
        JoulePerDegreeCelsius => { symbol: "J/°C"; definition: crate::consts::heat_capacity::JOULE_PER_DEGREE_CELSIUS; aliases: ["J/degC"]; uom: joule_per_degree_celsius; }
        /// Thermochemical calorie per kelvin with canonical symbol
        /// `cal (th)/K`.
        ThermochemicalCaloriePerKelvin => { symbol: "cal (th)/K"; definition: crate::consts::heat_capacity::THERMOCHEMICAL_CALORIE_PER_KELVIN; aliases: ["cal/K"]; uom: calorie_per_kelvin; }
        /// International Table British thermal unit per degree Fahrenheit
        /// with canonical symbol `Btu (IT)/°F`.
        BritishThermalUnitInternationalTablePerDegreeFahrenheit => { symbol: "Btu (IT)/°F"; definition: crate::consts::heat_capacity::BRITISH_THERMAL_UNIT_INTERNATIONAL_TABLE_PER_DEGREE_FAHRENHEIT; aliases: ["Btu/°F", "Btu/degF"]; uom: btu_it_per_degree_fahrenheit; }
    }
}
