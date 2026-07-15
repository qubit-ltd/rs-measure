// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted electric current density measurements.

use crate::define_unit_family;
use uom::si::electric_current_density::{
    ampere_per_square_centimeter,
    ampere_per_square_meter,
    ampere_per_square_millimeter,
};
use uom::si::f64::ElectricCurrentDensity as UomElectricCurrentDensity;

define_unit_family! {
    /// Units for persisted `uom` electric current density quantities.
    pub enum ElectricCurrentDensity for "electric_current_density", uom = UomElectricCurrentDensity {
        /// Ampere per square meter (`A/m²`).
        AmperePerSquareMeter => { symbol: "A/m²"; definition: crate::consts::electric_current_density::AMPERE_PER_SQUARE_METER; aliases: ["A/m2", "A/m^2"]; uom: ampere_per_square_meter; }
        /// Ampere per square centimeter (`A/cm²`).
        AmperePerSquareCentimeter => { symbol: "A/cm²"; definition: crate::consts::electric_current_density::AMPERE_PER_SQUARE_CENTIMETER; aliases: ["A/cm2", "A/cm^2"]; uom: ampere_per_square_centimeter; }
        /// Ampere per square millimeter (`A/mm²`).
        AmperePerSquareMillimeter => { symbol: "A/mm²"; definition: crate::consts::electric_current_density::AMPERE_PER_SQUARE_MILLIMETER; aliases: ["A/mm2", "A/mm^2"]; uom: ampere_per_square_millimeter; }
    }
}
