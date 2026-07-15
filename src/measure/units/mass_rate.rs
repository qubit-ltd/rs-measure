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
        MilligramPerSecond => { symbol: "mg/s"; definition: crate::consts::mass_rate::MILLIGRAM_PER_SECOND; uom: milligram_per_second; }
        /// Gram per second (`g/s`).
        GramPerSecond => { symbol: "g/s"; definition: crate::consts::mass_rate::GRAM_PER_SECOND; uom: gram_per_second; }
        /// Kilogram per second (`kg/s`).
        KilogramPerSecond => { symbol: "kg/s"; definition: crate::consts::mass_rate::KILOGRAM_PER_SECOND; uom: kilogram_per_second; }
        /// Kilogram per hour (`kg/h`).
        KilogramPerHour => { symbol: "kg/h"; definition: crate::consts::mass_rate::KILOGRAM_PER_HOUR; uom: kilogram_per_hour; }
        /// Tonne per hour (`t/h`).
        TonnePerHour => { symbol: "t/h"; definition: crate::consts::mass_rate::TONNE_PER_HOUR; uom: ton_per_hour; }
        /// Pound per hour (`lb/h`).
        PoundPerHour => { symbol: "lb/h"; definition: crate::consts::mass_rate::POUND_PER_HOUR; uom: pound_per_hour; }
    }
}
