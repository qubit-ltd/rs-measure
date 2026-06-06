// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted area measurements.

use super::define_measurement_unit;
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

define_measurement_unit! {
    /// Units for persisted `uom` area quantities.
    pub enum Area for UomArea, "area" {
        /// Square millimeter (`mm²`).
        SquareMillimeter => "mm²" | "mm2" | "mm^2", square_millimeter;
        /// Square centimeter (`cm²`).
        SquareCentimeter => "cm²" | "cm2" | "cm^2", square_centimeter;
        /// Square meter (`m²`).
        SquareMeter => "m²" | "m2" | "m^2", square_meter;
        /// Square kilometer (`km²`).
        SquareKilometer => "km²" | "km2" | "km^2", square_kilometer;
        /// Hectare (`ha`).
        Hectare => "ha", hectare;
        /// Acre (`ac`).
        Acre => "ac", acre;
        /// Square inch (`in²`).
        SquareInch => "in²" | "in2" | "in^2", square_inch;
        /// Square foot (`ft²`).
        SquareFoot => "ft²" | "ft2" | "ft^2", square_foot;
        /// Square yard (`yd²`).
        SquareYard => "yd²" | "yd2" | "yd^2", square_yard;
        /// Square mile (`mi²`).
        SquareMile => "mi²" | "mi2" | "mi^2", square_mile;
    }
}
