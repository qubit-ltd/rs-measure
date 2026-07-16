// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted luminance measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use uom::si::f64::Luminance as UomLuminance;
#[cfg(feature = "uom")]
use uom::si::luminance::{
    candela_per_square_centimeter,
    candela_per_square_foot,
    candela_per_square_meter,
    footlambert,
    stilb,
};

define_unit_family! {
    /// Units for persisted luminance measurements.
    pub enum Luminance for "luminance", uom = UomLuminance {
        /// Candela per square meter (`cd/m²`).
        CandelaPerSquareMeter => { symbol: "cd/m²"; definition: crate::consts::luminance::CANDELA_PER_SQUARE_METER; aliases: ["cd/m2", "cd/m^2"]; uom: candela_per_square_meter; }
        /// Candela per square centimeter (`cd/cm²`).
        CandelaPerSquareCentimeter => { symbol: "cd/cm²"; definition: crate::consts::luminance::CANDELA_PER_SQUARE_CENTIMETER; aliases: ["cd/cm2", "cd/cm^2"]; uom: candela_per_square_centimeter; }
        /// Candela per square foot (`cd/ft²`).
        CandelaPerSquareFoot => { symbol: "cd/ft²"; definition: crate::consts::luminance::CANDELA_PER_SQUARE_FOOT; aliases: ["cd/ft2", "cd/ft^2"]; uom: candela_per_square_foot; }
        /// Foot-lambert (`fl`).
        Footlambert => { symbol: "fl"; definition: crate::consts::luminance::FOOTLAMBERT; uom: footlambert; }
        /// Stilb (`sb`).
        Stilb => { symbol: "sb"; definition: crate::consts::luminance::STILB; uom: stilb; }
    }
}
