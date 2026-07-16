// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted mass concentration measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use uom::si::f64::MassConcentration as UomMassConcentration;
#[cfg(feature = "uom")]
use uom::si::mass_concentration::{
    gram_per_deciliter,
    gram_per_liter,
    kilogram_per_cubic_meter,
    microgram_per_liter,
    milligram_per_deciliter,
    milligram_per_liter,
};

define_unit_family! {
    /// Units for persisted mass concentration measurements.
    pub enum MassConcentration for "mass_concentration", uom = UomMassConcentration {
        /// Microgram per liter (`µg/L`).
        MicrogramPerLiter => { symbol: "µg/L"; definition: crate::consts::mass_concentration::MICROGRAM_PER_LITER; aliases: ["ug/L", "μg/L"]; uom: microgram_per_liter; }
        /// Milligram per liter (`mg/L`).
        MilligramPerLiter => { symbol: "mg/L"; definition: crate::consts::mass_concentration::MILLIGRAM_PER_LITER; uom: milligram_per_liter; }
        /// Gram per liter (`g/L`).
        GramPerLiter => { symbol: "g/L"; definition: crate::consts::mass_concentration::GRAM_PER_LITER; uom: gram_per_liter; }
        /// Kilogram per cubic meter (`kg/m³`).
        KilogramPerCubicMeter => { symbol: "kg/m³"; definition: crate::consts::mass_concentration::KILOGRAM_PER_CUBIC_METER; aliases: ["kg/m3", "kg/m^3"]; uom: kilogram_per_cubic_meter; }
        /// Milligram per deciliter (`mg/dL`).
        MilligramPerDeciliter => { symbol: "mg/dL"; definition: crate::consts::mass_concentration::MILLIGRAM_PER_DECILITER; uom: milligram_per_deciliter; }
        /// Gram per deciliter (`g/dL`).
        GramPerDeciliter => { symbol: "g/dL"; definition: crate::consts::mass_concentration::GRAM_PER_DECILITER; uom: gram_per_deciliter; }
    }
}
