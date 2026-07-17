// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted area measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;
#[cfg(feature = "uom")]
use uom::si::area::{
    acre,
    hectare,
    square_centimeter,
    square_foot,
    square_inch,
    square_kilometer,
    square_meter,
    square_mile,
    square_millimeter,
    square_yard,
};
#[cfg(feature = "uom")]
use uom::si::f64::Area as UomArea;

define_unit_family! {
    /// Units for persisted area measurements.
    pub enum Area for "area" {
        /// Square millimeter (`mm²`).
        SquareMillimeter => { symbol: "mm²"; definition: crate::consts::area::SQUARE_MILLIMETER; aliases: ["mm2", "mm^2"]; }
        /// Square centimeter (`cm²`).
        SquareCentimeter => { symbol: "cm²"; definition: crate::consts::area::SQUARE_CENTIMETER; aliases: ["cm2", "cm^2"]; }
        /// Square meter (`m²`).
        SquareMeter => { symbol: "m²"; definition: crate::consts::area::SQUARE_METER; aliases: ["m2", "m^2"]; }
        /// Square kilometer (`km²`).
        SquareKilometer => { symbol: "km²"; definition: crate::consts::area::SQUARE_KILOMETER; aliases: ["km2", "km^2"]; }
        /// Hectare (`ha`).
        Hectare => { symbol: "ha"; definition: crate::consts::area::HECTARE; }
        /// Acre (`ac`).
        Acre => { symbol: "ac"; definition: crate::consts::area::ACRE; }
        /// Square inch (`in²`).
        SquareInch => { symbol: "in²"; definition: crate::consts::area::SQUARE_INCH; aliases: ["in2", "in^2"]; }
        /// Square foot (`ft²`).
        SquareFoot => { symbol: "ft²"; definition: crate::consts::area::SQUARE_FOOT; aliases: ["ft2", "ft^2"]; }
        /// Square yard (`yd²`).
        SquareYard => { symbol: "yd²"; definition: crate::consts::area::SQUARE_YARD; aliases: ["yd2", "yd^2"]; }
        /// Square mile (`mi²`).
        SquareMile => { symbol: "mi²"; definition: crate::consts::area::SQUARE_MILE; aliases: ["mi2", "mi^2"]; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    Area, UomArea {
        SquareMillimeter => square_millimeter;
        SquareCentimeter => square_centimeter;
        SquareMeter => square_meter;
        SquareKilometer => square_kilometer;
        Hectare => hectare;
        Acre => acre;
        SquareInch => square_inch;
        SquareFoot => square_foot;
        SquareYard => square_yard;
        SquareMile => square_mile;
    }
}
