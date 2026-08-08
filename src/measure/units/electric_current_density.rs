// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted electric current density measurements.

#[cfg(feature = "uom")]
use uom::si::electric_current_density::ampere_per_square_meter;
#[cfg(feature = "uom")]
use uom::si::f64::ElectricCurrentDensity as UomElectricCurrentDensity;

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;

define_unit_family! {
    /// Units for persisted electric current density measurements.
    pub enum ElectricCurrentDensity for "electric_current_density" {
        /// Ampere per square meter (`A/m²`).
        AmperePerSquareMeter => { symbol: "A/m²"; definition: crate::consts::electric_current_density::AMPERE_PER_SQUARE_METER; aliases: ["A/m2", "A/m^2"]; }
        /// Ampere per square centimeter (`A/cm²`).
        AmperePerSquareCentimeter => { symbol: "A/cm²"; definition: crate::consts::electric_current_density::AMPERE_PER_SQUARE_CENTIMETER; aliases: ["A/cm2", "A/cm^2"]; }
        /// Ampere per square millimeter (`A/mm²`).
        AmperePerSquareMillimeter => { symbol: "A/mm²"; definition: crate::consts::electric_current_density::AMPERE_PER_SQUARE_MILLIMETER; aliases: ["A/mm2", "A/mm^2"]; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    ElectricCurrentDensity, UomElectricCurrentDensity {
        base: ampere_per_square_meter;
    }
}
