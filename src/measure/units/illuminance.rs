// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted illuminance measurements.

use crate::define_unit_family;
use uom::si::f64::Illuminance as UomIlluminance;
use uom::si::illuminance::{
    footcandle,
    kilolux,
    lux,
};

define_unit_family! {
    /// Units for persisted `uom` illuminance quantities.
    pub enum Illuminance for "illuminance", uom = UomIlluminance {
        /// Lux (`lx`).
        Lux => { symbol: "lx"; coefficient: 1; uom: lux; }
        /// Kilolux (`klx`).
        Kilolux => { symbol: "klx"; coefficient: 1000; uom: kilolux; }
        /// Foot-candle (`fc`).
        Footcandle => { symbol: "fc"; coefficient: 1562500 / 145161; uom: footcandle; }
    }
}
