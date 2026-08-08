// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted mass rate measurements.

#[cfg(feature = "uom")]
use uom::si::f64::MassRate as UomMassRate;
#[cfg(feature = "uom")]
use uom::si::mass_rate::kilogram_per_second;

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;

define_unit_family! {
    /// Units for persisted mass rate measurements.
    pub enum MassRate for "mass_rate" {
        /// Milligram per second (`mg/s`).
        MilligramPerSecond => { symbol: "mg/s"; definition: crate::consts::mass_rate::MILLIGRAM_PER_SECOND; }
        /// Gram per second (`g/s`).
        GramPerSecond => { symbol: "g/s"; definition: crate::consts::mass_rate::GRAM_PER_SECOND; }
        /// Kilogram per second (`kg/s`).
        KilogramPerSecond => { symbol: "kg/s"; definition: crate::consts::mass_rate::KILOGRAM_PER_SECOND; }
        /// Kilogram per hour (`kg/h`).
        KilogramPerHour => { symbol: "kg/h"; definition: crate::consts::mass_rate::KILOGRAM_PER_HOUR; }
        /// Tonne per hour (`t/h`).
        TonnePerHour => { symbol: "t/h"; definition: crate::consts::mass_rate::TONNE_PER_HOUR; }
        /// Pound per hour (`lb/h`).
        PoundPerHour => { symbol: "lb/h"; definition: crate::consts::mass_rate::POUND_PER_HOUR; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    MassRate, UomMassRate {
        base: kilogram_per_second;
    }
}
