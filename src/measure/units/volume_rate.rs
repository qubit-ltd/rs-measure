// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted volume rate measurements.

use crate::define_unit_family;
use uom::si::f64::VolumeRate as UomVolumeRate;
use uom::si::volume_rate::{
    cubic_meter_per_hour,
    cubic_meter_per_second,
    gallon_per_minute,
    liter_per_minute,
    liter_per_second,
    milliliter_per_second,
};

define_unit_family! {
    /// Units for persisted `uom` volume rate quantities.
    pub enum VolumeRate for "volume_rate", uom = UomVolumeRate {
        /// Cubic meter per second (`m³/s`).
        CubicMeterPerSecond => { symbol: "m³/s"; coefficient: 1; aliases: ["m3/s", "m^3/s"]; uom: cubic_meter_per_second; }
        /// Cubic meter per hour (`m³/h`).
        CubicMeterPerHour => { symbol: "m³/h"; coefficient: 1 / 3600; aliases: ["m3/h", "m^3/h"]; uom: cubic_meter_per_hour; }
        /// Milliliter per second (`mL/s`).
        MilliliterPerSecond => { symbol: "mL/s"; coefficient: 1 / 1000000; uom: milliliter_per_second; }
        /// Liter per second (`L/s`).
        LiterPerSecond => { symbol: "L/s"; coefficient: 1 / 1000; uom: liter_per_second; }
        /// Liter per minute (`L/min`).
        LiterPerMinute => { symbol: "L/min"; coefficient: 1 / 60000; uom: liter_per_minute; }
        /// US gallon per minute (`gal/min`).
        UsGallonPerMinute => { symbol: "gal (US)/min"; coefficient: 157725491 / 2500000000000; aliases: ["gal/min"]; uom: gallon_per_minute; }
    }
}
