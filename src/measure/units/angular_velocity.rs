// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted angular velocity measurements.

use crate::define_unit_family;
use uom::si::angular_velocity::{
    degree_per_second,
    radian_per_second,
    revolution_per_minute,
    revolution_per_second,
};
use uom::si::f64::AngularVelocity as UomAngularVelocity;

define_unit_family! {
    /// Units for persisted `uom` angular velocity quantities.
    pub enum AngularVelocity for "angular_velocity", uom = UomAngularVelocity {
        /// Radian per second (`rad/s`).
        RadianPerSecond => { symbol: "rad/s"; coefficient: 1; uom: radian_per_second; }
        /// Degree per second (`°/s`).
        DegreePerSecond => { symbol: "°/s"; coefficient: 3490658503988659 / 200000000000000000; aliases: ["deg/s"]; uom: degree_per_second; }
        /// Revolution per second (`rps`).
        RevolutionPerSecond => { symbol: "rps"; coefficient: 3141592653589793 / 500000000000000; uom: revolution_per_second; }
        /// Revolution per minute (`rpm`).
        RevolutionPerMinute => { symbol: "rpm"; coefficient: 10471975511965977 / 100000000000000000; uom: revolution_per_minute; }
    }
}
