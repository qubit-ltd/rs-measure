// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted angular velocity measurements.

#[cfg(feature = "uom")]
use uom::si::angular_velocity::radian_per_second;
#[cfg(feature = "uom")]
use uom::si::f64::AngularVelocity as UomAngularVelocity;

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;

define_unit_family! {
    /// Units for persisted angular velocity measurements.
    pub enum AngularVelocity for "angular_velocity" {
        /// Radian per second (`rad/s`).
        RadianPerSecond => { symbol: "rad/s"; definition: crate::consts::angular_velocity::RADIAN_PER_SECOND; }
        /// Degree per second (`°/s`).
        DegreePerSecond => { symbol: "°/s"; definition: crate::consts::angular_velocity::DEGREE_PER_SECOND; aliases: ["deg/s"]; }
        /// Revolution per second (`rps`).
        RevolutionPerSecond => { symbol: "rps"; definition: crate::consts::angular_velocity::REVOLUTION_PER_SECOND; }
        /// Revolution per minute (`rpm`).
        RevolutionPerMinute => { symbol: "rpm"; definition: crate::consts::angular_velocity::REVOLUTION_PER_MINUTE; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    AngularVelocity, UomAngularVelocity {
        base: radian_per_second;
    }
}
