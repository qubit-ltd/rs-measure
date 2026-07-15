// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted solid angle measurements.

use crate::define_unit_family;
use uom::si::f64::SolidAngle as UomSolidAngle;
use uom::si::solid_angle::{
    spat,
    square_degree,
    steradian,
};

define_unit_family! {
    /// Units for persisted `uom` solid angle quantities.
    pub enum SolidAngle for "solid_angle", uom = UomSolidAngle {
        /// Steradian (`sr`).
        Steradian => { symbol: "sr"; coefficient: 1; uom: steradian; }
        /// Spat (`sp`).
        Spat => { symbol: "sp"; coefficient: 12566370614359173 / 1000000000000000; uom: spat; }
        /// Square degree (`°²`).
        SquareDegree => { symbol: "°²"; coefficient: 1523087098933543 / 5000000000000000000; aliases: ["deg2", "deg^2"]; uom: square_degree; }
    }
}
