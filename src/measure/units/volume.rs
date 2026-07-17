// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted volume measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;
#[cfg(feature = "uom")]
use uom::si::f64::Volume as UomVolume;
#[cfg(feature = "uom")]
use uom::si::volume::{
    cubic_centimeter,
    cubic_foot,
    cubic_inch,
    cubic_meter,
    cubic_millimeter,
    cubic_yard,
    cup,
    fluid_ounce,
    gallon,
    liter,
    microliter,
    milliliter,
    pint_liquid,
    quart_liquid,
};

define_unit_family! {
    /// Units for persisted volume measurements.
    pub enum Volume for "volume" {
        /// Cubic millimeter (`mm³`).
        CubicMillimeter => { symbol: "mm³"; definition: crate::consts::volume::CUBIC_MILLIMETER; aliases: ["mm3", "mm^3"]; }
        /// Cubic centimeter (`cm³`).
        CubicCentimeter => { symbol: "cm³"; definition: crate::consts::volume::CUBIC_CENTIMETER; aliases: ["cm3", "cm^3"]; }
        /// Cubic meter (`m³`).
        CubicMeter => { symbol: "m³"; definition: crate::consts::volume::CUBIC_METER; aliases: ["m3", "m^3"]; }
        /// Microliter (`µL`).
        Microliter => { symbol: "µL"; definition: crate::consts::volume::MICROLITER; aliases: ["uL", "μL"]; }
        /// Milliliter (`mL`).
        Milliliter => { symbol: "mL"; definition: crate::consts::volume::MILLILITER; }
        /// Liter (`L`).
        Liter => { symbol: "L"; definition: crate::consts::volume::LITER; }
        /// Cubic inch (`in³`).
        CubicInch => { symbol: "in³"; definition: crate::consts::volume::CUBIC_INCH; aliases: ["in3", "in^3"]; }
        /// Cubic foot (`ft³`).
        CubicFoot => { symbol: "ft³"; definition: crate::consts::volume::CUBIC_FOOT; aliases: ["ft3", "ft^3"]; }
        /// Cubic yard (`yd³`).
        CubicYard => { symbol: "yd³"; definition: crate::consts::volume::CUBIC_YARD; aliases: ["yd3", "yd^3"]; }
        /// US fluid ounce with canonical symbol `fl oz (US)`.
        UsFluidOunce => { symbol: "fl oz (US)"; definition: crate::consts::volume::US_FLUID_OUNCE; aliases: ["fl oz"]; }
        /// US customary cup with canonical symbol `cup (US customary)`.
        UsCustomaryCup => { symbol: "cup (US customary)"; definition: crate::consts::volume::US_CUSTOMARY_CUP; aliases: ["cup"]; }
        /// US liquid pint with canonical symbol `pt (US liq)`.
        UsLiquidPint => { symbol: "pt (US liq)"; definition: crate::consts::volume::US_LIQUID_PINT; aliases: ["liq pt"]; }
        /// US liquid quart with canonical symbol `qt (US liq)`.
        UsLiquidQuart => { symbol: "qt (US liq)"; definition: crate::consts::volume::US_LIQUID_QUART; aliases: ["liq qt"]; }
        /// US liquid gallon with canonical symbol `gal (US)`.
        UsLiquidGallon => { symbol: "gal (US)"; definition: crate::consts::volume::US_LIQUID_GALLON; aliases: ["gal"]; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    Volume, UomVolume {
        CubicMillimeter => cubic_millimeter;
        CubicCentimeter => cubic_centimeter;
        CubicMeter => cubic_meter;
        Microliter => microliter;
        Milliliter => milliliter;
        Liter => liter;
        CubicInch => cubic_inch;
        CubicFoot => cubic_foot;
        CubicYard => cubic_yard;
        UsFluidOunce => fluid_ounce;
        UsCustomaryCup => cup;
        UsLiquidPint => pint_liquid;
        UsLiquidQuart => quart_liquid;
        UsLiquidGallon => gallon;
    }
}
