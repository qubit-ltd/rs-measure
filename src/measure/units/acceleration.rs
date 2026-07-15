// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted acceleration measurements.

use crate::define_unit_family;
use uom::si::acceleration::{
    foot_per_second_squared,
    meter_per_second_squared,
    millimeter_per_second_squared,
    standard_gravity,
};
use uom::si::f64::Acceleration as UomAcceleration;

define_unit_family! {
    /// Units for persisted `uom` acceleration quantities.
    pub enum Acceleration for "acceleration", uom = UomAcceleration {
        /// Millimeter per second squared (`mm/s²`).
        MillimeterPerSecondSquared => { symbol: "mm/s²"; coefficient: 1 / 1000; aliases: ["mm/s2", "mm/s^2"]; uom: millimeter_per_second_squared; }
        /// Meter per second squared (`m/s²`).
        MeterPerSecondSquared => { symbol: "m/s²"; coefficient: 1; aliases: ["m/s2", "m/s^2"]; uom: meter_per_second_squared; }
        /// Foot per second squared (`ft/s²`).
        FootPerSecondSquared => { symbol: "ft/s²"; coefficient: 381 / 1250; aliases: ["ft/s2", "ft/s^2"]; uom: foot_per_second_squared; }
        /// Standard gravity (`g₀`).
        StandardGravity => { symbol: "g₀"; coefficient: 196133 / 20000; aliases: ["g0", "g"]; uom: standard_gravity; }
    }
}
