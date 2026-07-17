// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted electric current measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;
#[cfg(feature = "uom")]
use uom::si::electric_current::ampere;
#[cfg(feature = "uom")]
use uom::si::f64::ElectricCurrent as UomElectricCurrent;

define_unit_family! {
    /// Units for persisted electric current measurements.
    pub enum ElectricCurrent for "electric_current" {
        /// Picoampere (`pA`).
        Picoampere => { symbol: "pA"; definition: crate::consts::electric_current::PICOAMPERE; }
        /// Nanoampere (`nA`).
        Nanoampere => { symbol: "nA"; definition: crate::consts::electric_current::NANOAMPERE; }
        /// Microampere (`µA`).
        Microampere => { symbol: "µA"; definition: crate::consts::electric_current::MICROAMPERE; aliases: ["uA", "μA"]; }
        /// Milliampere (`mA`).
        Milliampere => { symbol: "mA"; definition: crate::consts::electric_current::MILLIAMPERE; }
        /// Ampere (`A`).
        Ampere => { symbol: "A"; definition: crate::consts::electric_current::AMPERE; }
        /// Kiloampere (`kA`).
        Kiloampere => { symbol: "kA"; definition: crate::consts::electric_current::KILOAMPERE; }
        /// Megaampere (`MA`).
        Megaampere => { symbol: "MA"; definition: crate::consts::electric_current::MEGAAMPERE; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    ElectricCurrent, UomElectricCurrent {
        base: ampere;
    }
}
