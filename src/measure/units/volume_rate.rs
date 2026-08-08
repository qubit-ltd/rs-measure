// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted volume rate measurements.

#[cfg(feature = "uom")]
use uom::si::f64::VolumeRate as UomVolumeRate;
#[cfg(feature = "uom")]
use uom::si::volume_rate::cubic_meter_per_second;

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;

define_unit_family! {
    /// Units for persisted volume rate measurements.
    pub enum VolumeRate for "volume_rate" {
        /// Cubic meter per second (`m³/s`).
        CubicMeterPerSecond => { symbol: "m³/s"; definition: crate::consts::volume_rate::CUBIC_METER_PER_SECOND; aliases: ["m3/s", "m^3/s"]; }
        /// Cubic meter per hour (`m³/h`).
        CubicMeterPerHour => { symbol: "m³/h"; definition: crate::consts::volume_rate::CUBIC_METER_PER_HOUR; aliases: ["m3/h", "m^3/h"]; }
        /// Milliliter per second (`mL/s`).
        MilliliterPerSecond => { symbol: "mL/s"; definition: crate::consts::volume_rate::MILLILITER_PER_SECOND; }
        /// Liter per second (`L/s`).
        LiterPerSecond => { symbol: "L/s"; definition: crate::consts::volume_rate::LITER_PER_SECOND; }
        /// Liter per minute (`L/min`).
        LiterPerMinute => { symbol: "L/min"; definition: crate::consts::volume_rate::LITER_PER_MINUTE; }
        /// US liquid gallon per minute with canonical symbol `gal (US)/min`.
        UsGallonPerMinute => { symbol: "gal (US)/min"; definition: crate::consts::volume_rate::US_GALLON_PER_MINUTE; aliases: ["gal/min"]; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    VolumeRate, UomVolumeRate {
        base: cubic_meter_per_second;
    }
}
