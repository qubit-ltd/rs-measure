// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted mass concentration measurements.

use crate::define_unit_family;
use uom::si::f64::MassConcentration as UomMassConcentration;
use uom::si::mass_concentration::{
    gram_per_deciliter,
    gram_per_liter,
    kilogram_per_cubic_meter,
    microgram_per_liter,
    milligram_per_deciliter,
    milligram_per_liter,
};

define_unit_family! {
    /// Units for persisted `uom` mass concentration quantities.
    pub enum MassConcentration for "mass_concentration", uom = UomMassConcentration {
        /// Microgram per liter (`µg/L`).
        MicrogramPerLiter => { symbol: "µg/L"; coefficient: 1 / 1000000; aliases: ["ug/L", "μg/L"]; uom: microgram_per_liter; }
        /// Milligram per liter (`mg/L`).
        MilligramPerLiter => { symbol: "mg/L"; coefficient: 1 / 1000; uom: milligram_per_liter; }
        /// Gram per liter (`g/L`).
        GramPerLiter => { symbol: "g/L"; coefficient: 1; uom: gram_per_liter; }
        /// Kilogram per cubic meter (`kg/m³`).
        KilogramPerCubicMeter => { symbol: "kg/m³"; coefficient: 1; aliases: ["kg/m3", "kg/m^3"]; uom: kilogram_per_cubic_meter; }
        /// Milligram per deciliter (`mg/dL`).
        MilligramPerDeciliter => { symbol: "mg/dL"; coefficient: 1 / 100; uom: milligram_per_deciliter; }
        /// Gram per deciliter (`g/dL`).
        GramPerDeciliter => { symbol: "g/dL"; coefficient: 10; uom: gram_per_deciliter; }
    }
}
