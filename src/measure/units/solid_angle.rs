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
        Steradian => { symbol: "sr"; definition: crate::consts::solid_angle::STERADIAN; uom: steradian; }
        /// Spat (`sp`).
        Spat => { symbol: "sp"; definition: crate::consts::solid_angle::SPAT; uom: spat; }
        /// Square degree (`°²`).
        SquareDegree => { symbol: "°²"; definition: crate::consts::solid_angle::SQUARE_DEGREE; aliases: ["deg2", "deg^2"]; uom: square_degree; }
    }
}
