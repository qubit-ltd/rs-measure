// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted acceleration measurements.

use super::define_measurement_unit;
use uom::si::acceleration::{
    foot_per_second_squared,
    meter_per_second_squared,
    millimeter_per_second_squared,
    standard_gravity,
};
use uom::si::f64::Acceleration as UomAcceleration;

define_measurement_unit! {
    /// Units for persisted `uom` acceleration quantities.
    pub enum Acceleration for UomAcceleration, "acceleration" {
        /// Millimeter per second squared (`mm/s²`).
        MillimeterPerSecondSquared => "mm/s²" | "mm/s2" | "mm/s^2", millimeter_per_second_squared;
        /// Meter per second squared (`m/s²`).
        MeterPerSecondSquared => "m/s²" | "m/s2" | "m/s^2", meter_per_second_squared;
        /// Foot per second squared (`ft/s²`).
        FootPerSecondSquared => "ft/s²" | "ft/s2" | "ft/s^2", foot_per_second_squared;
        /// Standard gravity (`g₀`).
        StandardGravity => "g₀" | "g0" | "g", standard_gravity;
    }
}
