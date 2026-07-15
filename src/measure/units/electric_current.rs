// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted electric current measurements.

use crate::define_unit_family;
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

define_unit_family! {
    /// Units for persisted `uom` electric current quantities.
    pub enum ElectricCurrent for "electric_current", uom = UomElectricCurrent {
        /// Picoampere (`pA`).
        Picoampere => { symbol: "pA"; coefficient: 1 / 1000000000000; uom: picoampere; }
        /// Nanoampere (`nA`).
        Nanoampere => { symbol: "nA"; coefficient: 1 / 1000000000; uom: nanoampere; }
        /// Microampere (`µA`).
        Microampere => { symbol: "µA"; coefficient: 1 / 1000000; aliases: ["uA", "μA"]; uom: microampere; }
        /// Milliampere (`mA`).
        Milliampere => { symbol: "mA"; coefficient: 1 / 1000; uom: milliampere; }
        /// Ampere (`A`).
        Ampere => { symbol: "A"; coefficient: 1; uom: ampere; }
        /// Kiloampere (`kA`).
        Kiloampere => { symbol: "kA"; coefficient: 1000; uom: kiloampere; }
        /// Megaampere (`MA`).
        Megaampere => { symbol: "MA"; coefficient: 1000000; uom: megaampere; }
    }
}
