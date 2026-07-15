// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted area measurements.

use crate::define_unit_family;
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
use uom::si::f64::Area as UomArea;

define_unit_family! {
    /// Units for persisted `uom` area quantities.
    pub enum Area for "area", uom = UomArea {
        /// Square millimeter (`mm²`).
        SquareMillimeter => { symbol: "mm²"; coefficient: 1 / 1000000; aliases: ["mm2", "mm^2"]; uom: square_millimeter; }
        /// Square centimeter (`cm²`).
        SquareCentimeter => { symbol: "cm²"; coefficient: 1 / 10000; aliases: ["cm2", "cm^2"]; uom: square_centimeter; }
        /// Square meter (`m²`).
        SquareMeter => { symbol: "m²"; coefficient: 1; aliases: ["m2", "m^2"]; uom: square_meter; }
        /// Square kilometer (`km²`).
        SquareKilometer => { symbol: "km²"; coefficient: 1000000; aliases: ["km2", "km^2"]; uom: square_kilometer; }
        /// Hectare (`ha`).
        Hectare => { symbol: "ha"; coefficient: 10000; uom: hectare; }
        /// Acre (`ac`).
        Acre => { symbol: "ac"; coefficient: 316160658 / 78125; uom: acre; }
        /// Square inch (`in²`).
        SquareInch => { symbol: "in²"; coefficient: 16129 / 25000000; aliases: ["in2", "in^2"]; uom: square_inch; }
        /// Square foot (`ft²`).
        SquareFoot => { symbol: "ft²"; coefficient: 145161 / 1562500; aliases: ["ft2", "ft^2"]; uom: square_foot; }
        /// Square yard (`yd²`).
        SquareYard => { symbol: "yd²"; coefficient: 1306449 / 1562500; aliases: ["yd2", "yd^2"]; uom: square_yard; }
        /// Square mile (`mi²`).
        SquareMile => { symbol: "mi²"; coefficient: 40468564224 / 15625; aliases: ["mi2", "mi^2"]; uom: square_mile; }
    }
}
