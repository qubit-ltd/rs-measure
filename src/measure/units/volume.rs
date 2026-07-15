// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted volume measurements.

use crate::define_unit_family;
use uom::si::f64::Volume as UomVolume;
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
    /// Units for persisted `uom` volume quantities.
    pub enum Volume for "volume", uom = UomVolume {
        /// Cubic millimeter (`mm³`).
        CubicMillimeter => { symbol: "mm³"; coefficient: 1 / 1000000000; aliases: ["mm3", "mm^3"]; uom: cubic_millimeter; }
        /// Cubic centimeter (`cm³`).
        CubicCentimeter => { symbol: "cm³"; coefficient: 1 / 1000000; aliases: ["cm3", "cm^3"]; uom: cubic_centimeter; }
        /// Cubic meter (`m³`).
        CubicMeter => { symbol: "m³"; coefficient: 1; aliases: ["m3", "m^3"]; uom: cubic_meter; }
        /// Microliter (`µL`).
        Microliter => { symbol: "µL"; coefficient: 1 / 1000000000; aliases: ["uL", "μL"]; uom: microliter; }
        /// Milliliter (`mL`).
        Milliliter => { symbol: "mL"; coefficient: 1 / 1000000; uom: milliliter; }
        /// Liter (`L`).
        Liter => { symbol: "L"; coefficient: 1 / 1000; uom: liter; }
        /// Cubic inch (`in³`).
        CubicInch => { symbol: "in³"; coefficient: 2048383 / 125000000000; aliases: ["in3", "in^3"]; uom: cubic_inch; }
        /// Cubic foot (`ft³`).
        CubicFoot => { symbol: "ft³"; coefficient: 55306341 / 1953125000; aliases: ["ft3", "ft^3"]; uom: cubic_foot; }
        /// Cubic yard (`yd³`).
        CubicYard => { symbol: "yd³"; coefficient: 1493271207 / 1953125000; aliases: ["yd3", "yd^3"]; uom: cubic_yard; }
        /// US fluid ounce (`fl oz`).
        UsFluidOunce => { symbol: "fl oz (US)"; coefficient: 473176473 / 16000000000000; aliases: ["fl oz"]; uom: fluid_ounce; }
        /// US cup (`cup`).
        UsCustomaryCup => { symbol: "cup (US customary)"; coefficient: 473176473 / 2000000000000; aliases: ["cup"]; uom: cup; }
        /// US liquid pint (`liq pt`).
        UsLiquidPint => { symbol: "pt (US liq)"; coefficient: 473176473 / 1000000000000; aliases: ["liq pt"]; uom: pint_liquid; }
        /// US liquid quart (`liq qt`).
        UsLiquidQuart => { symbol: "qt (US liq)"; coefficient: 473176473 / 500000000000; aliases: ["liq qt"]; uom: quart_liquid; }
        /// Gallon (`gal`).
        UsLiquidGallon => { symbol: "gal (US)"; coefficient: 473176473 / 125000000000; aliases: ["gal"]; uom: gallon; }
    }
}
