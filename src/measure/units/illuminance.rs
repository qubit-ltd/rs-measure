/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Units for persisted illuminance measurements.

use super::define_measurement_unit;
use uom::si::f64::Illuminance as UomIlluminance;
use uom::si::illuminance::{
    footcandle,
    kilolux,
    lux,
};

define_measurement_unit! {
    /// Units for persisted `uom` illuminance quantities.
    pub enum Illuminance for UomIlluminance, "illuminance" {
        /// Lux (`lx`).
        Lux => "lx", lux;
        /// Kilolux (`klx`).
        Kilolux => "klx", kilolux;
        /// Foot-candle (`fc`).
        Footcandle => "fc", footcandle;
    }
}
