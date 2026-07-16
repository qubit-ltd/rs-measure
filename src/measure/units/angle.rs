// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted angle measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use uom::si::angle::{
    degree,
    minute,
    radian,
    revolution,
    second,
};
#[cfg(feature = "uom")]
use uom::si::f64::Angle as UomAngle;

define_unit_family! {
    /// Units for persisted angle measurements.
    pub enum Angle for "angle", uom = UomAngle {
        /// Radian (`rad`).
        Radian => { symbol: "rad"; definition: crate::consts::angle::RADIAN; uom: radian; }
        /// Degree (`°`).
        Degree => { symbol: "°"; definition: crate::consts::angle::DEGREE; aliases: ["deg"]; uom: degree; }
        /// Revolution (`r`).
        Revolution => { symbol: "r"; definition: crate::consts::angle::REVOLUTION; aliases: ["rev"]; uom: revolution; }
        /// Arcminute (`′`).
        Minute => { symbol: "′"; definition: crate::consts::angle::MINUTE; aliases: ["arcmin"]; uom: minute; }
        /// Arcsecond (`″`).
        Second => { symbol: "″"; definition: crate::consts::angle::SECOND; aliases: ["arcsec"]; uom: second; }
    }
}
