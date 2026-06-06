// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted luminance measurements.

use super::define_measurement_unit;
use uom::si::f64::Luminance as UomLuminance;
use uom::si::luminance::{
    candela_per_square_centimeter,
    candela_per_square_foot,
    candela_per_square_meter,
    footlambert,
    stilb,
};

define_measurement_unit! {
    /// Units for persisted `uom` luminance quantities.
    pub enum Luminance for UomLuminance, "luminance" {
        /// Candela per square meter (`cd/m²`).
        CandelaPerSquareMeter => "cd/m²" | "cd/m2" | "cd/m^2", candela_per_square_meter;
        /// Candela per square centimeter (`cd/cm²`).
        CandelaPerSquareCentimeter => "cd/cm²" | "cd/cm2" | "cd/cm^2", candela_per_square_centimeter;
        /// Candela per square foot (`cd/ft²`).
        CandelaPerSquareFoot => "cd/ft²" | "cd/ft2" | "cd/ft^2", candela_per_square_foot;
        /// Foot-lambert (`fl`).
        Footlambert => "fl", footlambert;
        /// Stilb (`sb`).
        Stilb => "sb", stilb;
    }
}
