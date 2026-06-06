// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted volume rate measurements.

use super::define_measurement_unit;
use uom::si::f64::VolumeRate as UomVolumeRate;
use uom::si::volume_rate::{
    cubic_meter_per_hour,
    cubic_meter_per_second,
    gallon_per_minute,
    liter_per_minute,
    liter_per_second,
    milliliter_per_second,
};

define_measurement_unit! {
    /// Units for persisted `uom` volume rate quantities.
    pub enum VolumeRate for UomVolumeRate, "volume rate" {
        /// Cubic meter per second (`m³/s`).
        CubicMeterPerSecond => "m³/s" | "m3/s" | "m^3/s", cubic_meter_per_second;
        /// Cubic meter per hour (`m³/h`).
        CubicMeterPerHour => "m³/h" | "m3/h" | "m^3/h", cubic_meter_per_hour;
        /// Milliliter per second (`mL/s`).
        MilliliterPerSecond => "mL/s", milliliter_per_second;
        /// Liter per second (`L/s`).
        LiterPerSecond => "L/s", liter_per_second;
        /// Liter per minute (`L/min`).
        LiterPerMinute => "L/min", liter_per_minute;
        /// US gallon per minute (`gal/min`).
        GallonPerMinute => "gal/min", gallon_per_minute;
    }
}
