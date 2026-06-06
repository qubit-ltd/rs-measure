// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted electric current density measurements.

use super::define_measurement_unit;
use uom::si::electric_current_density::{
    ampere_per_square_centimeter,
    ampere_per_square_meter,
    ampere_per_square_millimeter,
};
use uom::si::f64::ElectricCurrentDensity as UomElectricCurrentDensity;

define_measurement_unit! {
    /// Units for persisted `uom` electric current density quantities.
    pub enum ElectricCurrentDensity for UomElectricCurrentDensity, "electric current density" {
        /// Ampere per square meter (`A/m²`).
        AmperePerSquareMeter => "A/m²" | "A/m2" | "A/m^2", ampere_per_square_meter;
        /// Ampere per square centimeter (`A/cm²`).
        AmperePerSquareCentimeter => "A/cm²" | "A/cm2" | "A/cm^2", ampere_per_square_centimeter;
        /// Ampere per square millimeter (`A/mm²`).
        AmperePerSquareMillimeter => "A/mm²" | "A/mm2" | "A/mm^2", ampere_per_square_millimeter;
    }
}
