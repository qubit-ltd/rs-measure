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
        AmperePerSquareMeter => { symbol: "A/m²"; coefficient: 1; aliases: ["A/m2", "A/m^2"]; uom: ampere_per_square_meter; }
        /// Ampere per square centimeter (`A/cm²`).
        AmperePerSquareCentimeter => { symbol: "A/cm²"; coefficient: 10000; aliases: ["A/cm2", "A/cm^2"]; uom: ampere_per_square_centimeter; }
        /// Ampere per square millimeter (`A/mm²`).
        AmperePerSquareMillimeter => { symbol: "A/mm²"; coefficient: 1000000; aliases: ["A/mm2", "A/mm^2"]; uom: ampere_per_square_millimeter; }
    }
}
