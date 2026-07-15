// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted velocity measurements.

use crate::define_unit_family;
use uom::si::f64::Velocity as UomVelocity;
use uom::si::velocity::{
    centimeter_per_second,
    foot_per_second,
    kilometer_per_hour,
    knot,
    meter_per_second,
    micrometer_per_second,
    mile_per_hour,
    millimeter_per_second,
};

define_unit_family! {
    /// Units for persisted `uom` velocity quantities.
    pub enum Velocity for "velocity", uom = UomVelocity {
        /// Micrometer per second (`µm/s`).
        MicrometerPerSecond => { symbol: "µm/s"; coefficient: 1 / 1000000; aliases: ["um/s", "μm/s"]; uom: micrometer_per_second; }
        /// Millimeter per second (`mm/s`).
        MillimeterPerSecond => { symbol: "mm/s"; coefficient: 1 / 1000; uom: millimeter_per_second; }
        /// Centimeter per second (`cm/s`).
        CentimeterPerSecond => { symbol: "cm/s"; coefficient: 1 / 100; uom: centimeter_per_second; }
        /// Meter per second (`m/s`).
        MeterPerSecond => { symbol: "m/s"; coefficient: 1; uom: meter_per_second; }
        /// Kilometer per hour (`km/h`).
        KilometerPerHour => { symbol: "km/h"; coefficient: 5 / 18; aliases: ["km/hr", "kph"]; uom: kilometer_per_hour; }
        /// Foot per second (`ft/s`).
        FootPerSecond => { symbol: "ft/s"; coefficient: 381 / 1250; uom: foot_per_second; }
        /// Mile per hour (`mi/h`).
        MilePerHour => { symbol: "mi/h"; coefficient: 1397 / 3125; aliases: ["mi/hr", "mph"]; uom: mile_per_hour; }
        /// Knot (`kn`).
        Knot => { symbol: "kn"; coefficient: 463 / 900; uom: knot; }
    }
}
