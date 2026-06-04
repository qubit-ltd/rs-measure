/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Units for persisted mass rate measurements.

use super::define_measurement_unit;
use uom::si::f64::MassRate as UomMassRate;
use uom::si::mass_rate::{
    gram_per_second,
    kilogram_per_hour,
    kilogram_per_second,
    milligram_per_second,
    pound_per_hour,
    ton_per_hour,
};

define_measurement_unit! {
    /// Units for persisted `uom` mass rate quantities.
    pub enum MassRate for UomMassRate, "mass rate" {
        /// Milligram per second (`mg/s`).
        MilligramPerSecond => "mg/s", milligram_per_second;
        /// Gram per second (`g/s`).
        GramPerSecond => "g/s", gram_per_second;
        /// Kilogram per second (`kg/s`).
        KilogramPerSecond => "kg/s", kilogram_per_second;
        /// Kilogram per hour (`kg/h`).
        KilogramPerHour => "kg/h", kilogram_per_hour;
        /// Tonne per hour (`t/h`).
        TonnePerHour => "t/h", ton_per_hour;
        /// Pound per hour (`lb/h`).
        PoundPerHour => "lb/h", pound_per_hour;
    }
}
