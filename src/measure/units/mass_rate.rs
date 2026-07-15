// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted mass rate measurements.

use crate::define_unit_family;
use uom::si::f64::MassRate as UomMassRate;
use uom::si::mass_rate::{
    gram_per_second,
    kilogram_per_hour,
    kilogram_per_second,
    milligram_per_second,
    pound_per_hour,
    ton_per_hour,
};

define_unit_family! {
    /// Units for persisted `uom` mass rate quantities.
    pub enum MassRate for "mass_rate", uom = UomMassRate {
        /// Milligram per second (`mg/s`).
        MilligramPerSecond => { symbol: "mg/s"; coefficient: 1 / 1000000; uom: milligram_per_second; }
        /// Gram per second (`g/s`).
        GramPerSecond => { symbol: "g/s"; coefficient: 1 / 1000; uom: gram_per_second; }
        /// Kilogram per second (`kg/s`).
        KilogramPerSecond => { symbol: "kg/s"; coefficient: 1; uom: kilogram_per_second; }
        /// Kilogram per hour (`kg/h`).
        KilogramPerHour => { symbol: "kg/h"; coefficient: 1 / 3600; uom: kilogram_per_hour; }
        /// Tonne per hour (`t/h`).
        TonnePerHour => { symbol: "t/h"; coefficient: 5 / 18; uom: ton_per_hour; }
        /// Pound per hour (`lb/h`).
        PoundPerHour => { symbol: "lb/h"; coefficient: 45359237 / 360000000000; uom: pound_per_hour; }
    }
}
