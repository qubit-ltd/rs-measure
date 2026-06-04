/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Units for persisted velocity measurements.

use super::define_measurement_unit;
use uom::si::f64::Velocity as UomVelocity;
use uom::si::velocity::{
    foot_per_second,
    kilometer_per_hour,
    knot,
    meter_per_second,
    mile_per_hour,
};

define_measurement_unit! {
    /// Units for persisted `uom` velocity quantities.
    pub enum Velocity for UomVelocity, "velocity" {
        /// Meter per second (`m/s`).
        MeterPerSecond => "m/s", meter_per_second;
        /// Kilometer per hour (`km/h`).
        KilometerPerHour => "km/h", kilometer_per_hour;
        /// Foot per second (`ft/s`).
        FootPerSecond => "ft/s", foot_per_second;
        /// Mile per hour (`mi/h`).
        MilePerHour => "mi/h", mile_per_hour;
        /// Knot (`kn`).
        Knot => "kn", knot;
    }
}
