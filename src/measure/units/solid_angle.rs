// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted solid angle measurements.

#[cfg(feature = "uom")]
use uom::si::f64::SolidAngle as UomSolidAngle;
#[cfg(feature = "uom")]
use uom::si::solid_angle::steradian;

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;

define_unit_family! {
    /// Units for persisted solid angle measurements.
    pub enum SolidAngle for "solid_angle" {
        /// Steradian (`sr`).
        Steradian => { symbol: "sr"; definition: crate::consts::solid_angle::STERADIAN; }
        /// Spat (`sp`).
        Spat => { symbol: "sp"; definition: crate::consts::solid_angle::SPAT; }
        /// Square degree (`°²`).
        SquareDegree => { symbol: "°²"; definition: crate::consts::solid_angle::SQUARE_DEGREE; aliases: ["deg2", "deg^2"]; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    SolidAngle, UomSolidAngle {
        base: steradian;
    }
}
