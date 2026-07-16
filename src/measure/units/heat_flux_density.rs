// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted heat flux density measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use uom::si::f64::HeatFluxDensity as UomHeatFluxDensity;
#[cfg(feature = "uom")]
use uom::si::heat_flux_density::{
    kilowatt_per_square_meter,
    milliwatt_per_square_meter,
    watt_per_square_centimeter,
    watt_per_square_meter,
};

define_unit_family! {
    /// Units for persisted heat flux density measurements.
    pub enum HeatFluxDensity for "heat_flux_density", uom = UomHeatFluxDensity {
        /// Milliwatt per square meter (`mW/m²`).
        MilliwattPerSquareMeter => { symbol: "mW/m²"; definition: crate::consts::heat_flux_density::MILLIWATT_PER_SQUARE_METER; aliases: ["mW/m2", "mW/m^2"]; uom: milliwatt_per_square_meter; }
        /// Watt per square meter (`W/m²`).
        WattPerSquareMeter => { symbol: "W/m²"; definition: crate::consts::heat_flux_density::WATT_PER_SQUARE_METER; aliases: ["W/m2", "W/m^2"]; uom: watt_per_square_meter; }
        /// Kilowatt per square meter (`kW/m²`).
        KilowattPerSquareMeter => { symbol: "kW/m²"; definition: crate::consts::heat_flux_density::KILOWATT_PER_SQUARE_METER; aliases: ["kW/m2", "kW/m^2"]; uom: kilowatt_per_square_meter; }
        /// Watt per square centimeter (`W/cm²`).
        WattPerSquareCentimeter => { symbol: "W/cm²"; definition: crate::consts::heat_flux_density::WATT_PER_SQUARE_CENTIMETER; aliases: ["W/cm2", "W/cm^2"]; uom: watt_per_square_centimeter; }
    }
}
