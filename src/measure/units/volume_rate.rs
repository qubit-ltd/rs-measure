// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted volume rate measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use uom::si::f64::VolumeRate as UomVolumeRate;
#[cfg(feature = "uom")]
use uom::si::volume_rate::{
    cubic_meter_per_hour,
    cubic_meter_per_second,
    gallon_per_minute,
    liter_per_minute,
    liter_per_second,
    milliliter_per_second,
};

define_unit_family! {
    /// Units for persisted volume rate measurements.
    pub enum VolumeRate for "volume_rate", uom = UomVolumeRate {
        /// Cubic meter per second (`m³/s`).
        CubicMeterPerSecond => { symbol: "m³/s"; definition: crate::consts::volume_rate::CUBIC_METER_PER_SECOND; aliases: ["m3/s", "m^3/s"]; uom: cubic_meter_per_second; }
        /// Cubic meter per hour (`m³/h`).
        CubicMeterPerHour => { symbol: "m³/h"; definition: crate::consts::volume_rate::CUBIC_METER_PER_HOUR; aliases: ["m3/h", "m^3/h"]; uom: cubic_meter_per_hour; }
        /// Milliliter per second (`mL/s`).
        MilliliterPerSecond => { symbol: "mL/s"; definition: crate::consts::volume_rate::MILLILITER_PER_SECOND; uom: milliliter_per_second; }
        /// Liter per second (`L/s`).
        LiterPerSecond => { symbol: "L/s"; definition: crate::consts::volume_rate::LITER_PER_SECOND; uom: liter_per_second; }
        /// Liter per minute (`L/min`).
        LiterPerMinute => { symbol: "L/min"; definition: crate::consts::volume_rate::LITER_PER_MINUTE; uom: liter_per_minute; }
        /// US liquid gallon per minute with canonical symbol `gal (US)/min`.
        UsGallonPerMinute => { symbol: "gal (US)/min"; definition: crate::consts::volume_rate::US_GALLON_PER_MINUTE; aliases: ["gal/min"]; uom: gallon_per_minute; }
    }
}
