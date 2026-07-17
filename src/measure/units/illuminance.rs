// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted illuminance measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;
#[cfg(feature = "uom")]
use uom::si::f64::Illuminance as UomIlluminance;
#[cfg(feature = "uom")]
use uom::si::illuminance::lux;

define_unit_family! {
    /// Units for persisted illuminance measurements.
    pub enum Illuminance for "illuminance" {
        /// Lux (`lx`).
        Lux => { symbol: "lx"; definition: crate::consts::illuminance::LUX; }
        /// Kilolux (`klx`).
        Kilolux => { symbol: "klx"; definition: crate::consts::illuminance::KILOLUX; }
        /// Foot-candle (`fc`).
        Footcandle => { symbol: "fc"; definition: crate::consts::illuminance::FOOTCANDLE; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    Illuminance, UomIlluminance {
        base: lux;
    }
}
