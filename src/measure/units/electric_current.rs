// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted electric current measurements.

use super::define_measurement_unit;
use uom::si::electric_current::{
    ampere,
    kiloampere,
    megaampere,
    microampere,
    milliampere,
    nanoampere,
    picoampere,
};
use uom::si::f64::ElectricCurrent as UomElectricCurrent;

define_measurement_unit! {
    /// Units for persisted `uom` electric current quantities.
    pub enum ElectricCurrent for UomElectricCurrent, "electric current" {
        /// Picoampere (`pA`).
        Picoampere => "pA", picoampere;
        /// Nanoampere (`nA`).
        Nanoampere => "nA", nanoampere;
        /// Microampere (`µA`).
        Microampere => "µA" | "uA" | "μA", microampere;
        /// Milliampere (`mA`).
        Milliampere => "mA", milliampere;
        /// Ampere (`A`).
        Ampere => "A", ampere;
        /// Kiloampere (`kA`).
        Kiloampere => "kA", kiloampere;
        /// Megaampere (`MA`).
        Megaampere => "MA", megaampere;
    }
}
