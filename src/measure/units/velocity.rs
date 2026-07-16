// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted velocity measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use uom::si::f64::Velocity as UomVelocity;
#[cfg(feature = "uom")]
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
    /// Units for persisted velocity measurements.
    pub enum Velocity for "velocity", uom = UomVelocity {
        /// Micrometer per second (`µm/s`).
        MicrometerPerSecond => { symbol: "µm/s"; definition: crate::consts::velocity::MICROMETER_PER_SECOND; aliases: ["um/s", "μm/s"]; uom: micrometer_per_second; }
        /// Millimeter per second (`mm/s`).
        MillimeterPerSecond => { symbol: "mm/s"; definition: crate::consts::velocity::MILLIMETER_PER_SECOND; uom: millimeter_per_second; }
        /// Centimeter per second (`cm/s`).
        CentimeterPerSecond => { symbol: "cm/s"; definition: crate::consts::velocity::CENTIMETER_PER_SECOND; uom: centimeter_per_second; }
        /// Meter per second (`m/s`).
        MeterPerSecond => { symbol: "m/s"; definition: crate::consts::velocity::METER_PER_SECOND; uom: meter_per_second; }
        /// Kilometer per hour (`km/h`).
        KilometerPerHour => { symbol: "km/h"; definition: crate::consts::velocity::KILOMETER_PER_HOUR; aliases: ["km/hr", "kph"]; uom: kilometer_per_hour; }
        /// Foot per second (`ft/s`).
        FootPerSecond => { symbol: "ft/s"; definition: crate::consts::velocity::FOOT_PER_SECOND; uom: foot_per_second; }
        /// Mile per hour (`mi/h`).
        MilePerHour => { symbol: "mi/h"; definition: crate::consts::velocity::MILE_PER_HOUR; aliases: ["mi/hr", "mph"]; uom: mile_per_hour; }
        /// Knot (`kn`).
        Knot => { symbol: "kn"; definition: crate::consts::velocity::KNOT; uom: knot; }
    }
}
