// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted solid angle measurements.

use super::define_measurement_unit;
use uom::si::f64::SolidAngle as UomSolidAngle;
use uom::si::solid_angle::{
    spat,
    square_degree,
    steradian,
};

define_measurement_unit! {
    /// Units for persisted `uom` solid angle quantities.
    pub enum SolidAngle for UomSolidAngle, "solid angle" {
        /// Steradian (`sr`).
        Steradian => "sr", steradian;
        /// Spat (`sp`).
        Spat => "sp", spat;
        /// Square degree (`°²`).
        SquareDegree => "°²" | "deg2" | "deg^2", square_degree;
    }
}
