/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Units for persisted heat flux density measurements.

use super::define_measurement_unit;
use uom::si::f64::HeatFluxDensity as UomHeatFluxDensity;
use uom::si::heat_flux_density::{
    kilowatt_per_square_meter,
    milliwatt_per_square_meter,
    watt_per_square_centimeter,
    watt_per_square_meter,
};

define_measurement_unit! {
    /// Units for persisted `uom` heat flux density quantities.
    pub enum HeatFluxDensity for UomHeatFluxDensity, "heat flux density" {
        /// Milliwatt per square meter (`mW/m²`).
        MilliwattPerSquareMeter => "mW/m²" | "mW/m2" | "mW/m^2", milliwatt_per_square_meter;
        /// Watt per square meter (`W/m²`).
        WattPerSquareMeter => "W/m²" | "W/m2" | "W/m^2", watt_per_square_meter;
        /// Kilowatt per square meter (`kW/m²`).
        KilowattPerSquareMeter => "kW/m²" | "kW/m2" | "kW/m^2", kilowatt_per_square_meter;
        /// Watt per square centimeter (`W/cm²`).
        WattPerSquareCentimeter => "W/cm²" | "W/cm2" | "W/cm^2", watt_per_square_centimeter;
    }
}
