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
use crate::impl_uom_unit;
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
    pub enum MassConcentration for "mass_concentration" {
        /// Microgram per liter (`µg/L`).
        MicrogramPerLiter => { symbol: "µg/L"; definition: crate::consts::mass_concentration::MICROGRAM_PER_LITER; aliases: ["ug/L", "μg/L"]; }
        /// Milligram per liter (`mg/L`).
        MilligramPerLiter => { symbol: "mg/L"; definition: crate::consts::mass_concentration::MILLIGRAM_PER_LITER; }
        /// Gram per liter (`g/L`).
        GramPerLiter => { symbol: "g/L"; definition: crate::consts::mass_concentration::GRAM_PER_LITER; }
        /// Kilogram per cubic meter (`kg/m³`).
        KilogramPerCubicMeter => { symbol: "kg/m³"; definition: crate::consts::mass_concentration::KILOGRAM_PER_CUBIC_METER; aliases: ["kg/m3", "kg/m^3"]; }
        /// Milligram per deciliter (`mg/dL`).
        MilligramPerDeciliter => { symbol: "mg/dL"; definition: crate::consts::mass_concentration::MILLIGRAM_PER_DECILITER; }
        /// Gram per deciliter (`g/dL`).
        GramPerDeciliter => { symbol: "g/dL"; definition: crate::consts::mass_concentration::GRAM_PER_DECILITER; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    MassConcentration, UomMassConcentration {
        MicrogramPerLiter => microgram_per_liter;
        MilligramPerLiter => milligram_per_liter;
        GramPerLiter => gram_per_liter;
        KilogramPerCubicMeter => kilogram_per_cubic_meter;
        MilligramPerDeciliter => milligram_per_deciliter;
        GramPerDeciliter => gram_per_deciliter;
    }
}
