/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Units for persisted mass concentration measurements.

use super::define_measurement_unit;
use uom::si::f64::MassConcentration as UomMassConcentration;
use uom::si::mass_concentration::{
    gram_per_deciliter,
    gram_per_liter,
    kilogram_per_cubic_meter,
    microgram_per_liter,
    milligram_per_deciliter,
    milligram_per_liter,
};

define_measurement_unit! {
    /// Units for persisted `uom` mass concentration quantities.
    pub enum MassConcentration for UomMassConcentration, "mass concentration" {
        /// Microgram per liter (`µg/L`).
        MicrogramPerLiter => "µg/L" | "ug/L" | "μg/L", microgram_per_liter;
        /// Milligram per liter (`mg/L`).
        MilligramPerLiter => "mg/L", milligram_per_liter;
        /// Gram per liter (`g/L`).
        GramPerLiter => "g/L", gram_per_liter;
        /// Kilogram per cubic meter (`kg/m³`).
        KilogramPerCubicMeter => "kg/m³" | "kg/m3" | "kg/m^3", kilogram_per_cubic_meter;
        /// Milligram per deciliter (`mg/dL`).
        MilligramPerDeciliter => "mg/dL", milligram_per_deciliter;
        /// Gram per deciliter (`g/dL`).
        GramPerDeciliter => "g/dL", gram_per_deciliter;
    }
}
