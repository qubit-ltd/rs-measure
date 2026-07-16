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
    pub enum Area for "area", uom = UomArea {
        /// Square millimeter (`mm²`).
        SquareMillimeter => { symbol: "mm²"; definition: crate::consts::area::SQUARE_MILLIMETER; aliases: ["mm2", "mm^2"]; uom: square_millimeter; }
        /// Square centimeter (`cm²`).
        SquareCentimeter => { symbol: "cm²"; definition: crate::consts::area::SQUARE_CENTIMETER; aliases: ["cm2", "cm^2"]; uom: square_centimeter; }
        /// Square meter (`m²`).
        SquareMeter => { symbol: "m²"; definition: crate::consts::area::SQUARE_METER; aliases: ["m2", "m^2"]; uom: square_meter; }
        /// Square kilometer (`km²`).
        SquareKilometer => { symbol: "km²"; definition: crate::consts::area::SQUARE_KILOMETER; aliases: ["km2", "km^2"]; uom: square_kilometer; }
        /// Hectare (`ha`).
        Hectare => { symbol: "ha"; definition: crate::consts::area::HECTARE; uom: hectare; }
        /// Acre (`ac`).
        Acre => { symbol: "ac"; definition: crate::consts::area::ACRE; uom: acre; }
        /// Square inch (`in²`).
        SquareInch => { symbol: "in²"; definition: crate::consts::area::SQUARE_INCH; aliases: ["in2", "in^2"]; uom: square_inch; }
        /// Square foot (`ft²`).
        SquareFoot => { symbol: "ft²"; definition: crate::consts::area::SQUARE_FOOT; aliases: ["ft2", "ft^2"]; uom: square_foot; }
        /// Square yard (`yd²`).
        SquareYard => { symbol: "yd²"; definition: crate::consts::area::SQUARE_YARD; aliases: ["yd2", "yd^2"]; uom: square_yard; }
        /// Square mile (`mi²`).
        SquareMile => { symbol: "mi²"; definition: crate::consts::area::SQUARE_MILE; aliases: ["mi2", "mi^2"]; uom: square_mile; }
    }
}
