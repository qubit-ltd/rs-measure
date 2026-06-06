// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted angular velocity measurements.

use super::define_measurement_unit;
use uom::si::angular_velocity::{
    degree_per_second,
    radian_per_second,
    revolution_per_minute,
    revolution_per_second,
};
use uom::si::f64::AngularVelocity as UomAngularVelocity;

define_measurement_unit! {
    /// Units for persisted `uom` angular velocity quantities.
    pub enum AngularVelocity for UomAngularVelocity, "angular velocity" {
        /// Radian per second (`rad/s`).
        RadianPerSecond => "rad/s", radian_per_second;
        /// Degree per second (`°/s`).
        DegreePerSecond => "°/s" | "deg/s", degree_per_second;
        /// Revolution per second (`rps`).
        RevolutionPerSecond => "rps", revolution_per_second;
        /// Revolution per minute (`rpm`).
        RevolutionPerMinute => "rpm", revolution_per_minute;
    }
}
