// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted specific heat capacity measurements.

#[cfg(feature = "uom")]
use uom::si::f64::SpecificHeatCapacity as UomSpecificHeatCapacity;
#[cfg(feature = "uom")]
use uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;

define_unit_family! {
    /// Units for persisted specific heat capacity measurements.
    pub enum SpecificHeatCapacity for "specific_heat_capacity" {
        /// Joule per kilogram kelvin (`J/(kg · K)`).
        JoulePerKilogramKelvin => { symbol: "J/(kg · K)"; definition: crate::consts::specific_heat_capacity::JOULE_PER_KILOGRAM_KELVIN; aliases: ["J/(kg*K)"]; }
        /// Kilojoule per kilogram kelvin (`kJ/(kg · K)`).
        KilojoulePerKilogramKelvin => { symbol: "kJ/(kg · K)"; definition: crate::consts::specific_heat_capacity::KILOJOULE_PER_KILOGRAM_KELVIN; aliases: ["kJ/(kg*K)"]; }
        /// Joule per gram degree Celsius (`J/(g · °C)`).
        JoulePerGramDegreeCelsius => { symbol: "J/(g · °C)"; definition: crate::consts::specific_heat_capacity::JOULE_PER_GRAM_DEGREE_CELSIUS; aliases: ["J/(g*degC)"]; }
        /// Thermochemical calorie per gram kelvin with canonical symbol
        /// `cal (th)/(g · K)`.
        ThermochemicalCaloriePerGramKelvin => { symbol: "cal (th)/(g · K)"; definition: crate::consts::specific_heat_capacity::THERMOCHEMICAL_CALORIE_PER_GRAM_KELVIN; aliases: ["cal/(g · K)", "cal/(g*K)"]; }
        /// International Table British thermal unit per pound degree
        /// Fahrenheit with canonical symbol `Btu (IT)/(lb · °F)`.
        BritishThermalUnitInternationalTablePerPoundDegreeFahrenheit => { symbol: "Btu (IT)/(lb · °F)"; definition: crate::consts::specific_heat_capacity::BRITISH_THERMAL_UNIT_INTERNATIONAL_TABLE_PER_POUND_DEGREE_FAHRENHEIT; aliases: ["Btu/(lb · °F)", "Btu/(lb*degF)"]; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    SpecificHeatCapacity, UomSpecificHeatCapacity {
        base: joule_per_kilogram_kelvin;
    }
}
