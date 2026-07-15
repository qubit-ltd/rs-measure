// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted angle measurements.

use crate::define_unit_family;
use uom::si::angle::{
    degree,
    minute,
    radian,
    revolution,
    second,
};
use uom::si::f64::Angle as UomAngle;

define_unit_family! {
    /// Units for persisted `uom` angle quantities.
    pub enum Angle for "angle", uom = UomAngle {
        /// Radian (`rad`).
        Radian => { symbol: "rad"; coefficient: 1; uom: radian; }
        /// Degree (`°`).
        Degree => { symbol: "°"; coefficient: 3490658503988659 / 200000000000000000; aliases: ["deg"]; uom: degree; }
        /// Revolution (`r`).
        Revolution => { symbol: "r"; coefficient: 3141592653589793 / 500000000000000; aliases: ["rev"]; uom: revolution; }
        /// Arcminute (`′`).
        Minute => { symbol: "′"; coefficient: 45451282604019 / 156250000000000000; aliases: ["arcmin"]; uom: minute; }
        /// Arcsecond (`″`).
        Second => { symbol: "″"; coefficient: 15150427534673 / 3125000000000000000; aliases: ["arcsec"]; uom: second; }
    }
}
