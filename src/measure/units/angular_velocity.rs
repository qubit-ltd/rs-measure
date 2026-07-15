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
        RadianPerSecond => { symbol: "rad/s"; definition: crate::consts::angular_velocity::RADIAN_PER_SECOND; uom: radian_per_second; }
        /// Degree per second (`°/s`).
        DegreePerSecond => { symbol: "°/s"; definition: crate::consts::angular_velocity::DEGREE_PER_SECOND; aliases: ["deg/s"]; uom: degree_per_second; }
        /// Revolution per second (`rps`).
        RevolutionPerSecond => { symbol: "rps"; definition: crate::consts::angular_velocity::REVOLUTION_PER_SECOND; uom: revolution_per_second; }
        /// Revolution per minute (`rpm`).
        RevolutionPerMinute => { symbol: "rpm"; definition: crate::consts::angular_velocity::REVOLUTION_PER_MINUTE; uom: revolution_per_minute; }
    }
}
