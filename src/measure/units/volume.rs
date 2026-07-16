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
    pub enum Volume for "volume", uom = UomVolume {
        /// Cubic millimeter (`mm³`).
        CubicMillimeter => { symbol: "mm³"; definition: crate::consts::volume::CUBIC_MILLIMETER; aliases: ["mm3", "mm^3"]; uom: cubic_millimeter; }
        /// Cubic centimeter (`cm³`).
        CubicCentimeter => { symbol: "cm³"; definition: crate::consts::volume::CUBIC_CENTIMETER; aliases: ["cm3", "cm^3"]; uom: cubic_centimeter; }
        /// Cubic meter (`m³`).
        CubicMeter => { symbol: "m³"; definition: crate::consts::volume::CUBIC_METER; aliases: ["m3", "m^3"]; uom: cubic_meter; }
        /// Microliter (`µL`).
        Microliter => { symbol: "µL"; definition: crate::consts::volume::MICROLITER; aliases: ["uL", "μL"]; uom: microliter; }
        /// Milliliter (`mL`).
        Milliliter => { symbol: "mL"; definition: crate::consts::volume::MILLILITER; uom: milliliter; }
        /// Liter (`L`).
        Liter => { symbol: "L"; definition: crate::consts::volume::LITER; uom: liter; }
        /// Cubic inch (`in³`).
        CubicInch => { symbol: "in³"; definition: crate::consts::volume::CUBIC_INCH; aliases: ["in3", "in^3"]; uom: cubic_inch; }
        /// Cubic foot (`ft³`).
        CubicFoot => { symbol: "ft³"; definition: crate::consts::volume::CUBIC_FOOT; aliases: ["ft3", "ft^3"]; uom: cubic_foot; }
        /// Cubic yard (`yd³`).
        CubicYard => { symbol: "yd³"; definition: crate::consts::volume::CUBIC_YARD; aliases: ["yd3", "yd^3"]; uom: cubic_yard; }
        /// US fluid ounce with canonical symbol `fl oz (US)`.
        UsFluidOunce => { symbol: "fl oz (US)"; definition: crate::consts::volume::US_FLUID_OUNCE; aliases: ["fl oz"]; uom: fluid_ounce; }
        /// US customary cup with canonical symbol `cup (US customary)`.
        UsCustomaryCup => { symbol: "cup (US customary)"; definition: crate::consts::volume::US_CUSTOMARY_CUP; aliases: ["cup"]; uom: cup; }
        /// US liquid pint with canonical symbol `pt (US liq)`.
        UsLiquidPint => { symbol: "pt (US liq)"; definition: crate::consts::volume::US_LIQUID_PINT; aliases: ["liq pt"]; uom: pint_liquid; }
        /// US liquid quart with canonical symbol `qt (US liq)`.
        UsLiquidQuart => { symbol: "qt (US liq)"; definition: crate::consts::volume::US_LIQUID_QUART; aliases: ["liq qt"]; uom: quart_liquid; }
        /// US liquid gallon with canonical symbol `gal (US)`.
        UsLiquidGallon => { symbol: "gal (US)"; definition: crate::consts::volume::US_LIQUID_GALLON; aliases: ["gal"]; uom: gallon; }
    }
}
