// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted electrical resistance measurements.

use crate::define_unit_family;
use uom::si::electrical_resistance::{
    gigaohm,
    kiloohm,
    megaohm,
    microohm,
    milliohm,
    ohm,
};
use uom::si::f64::ElectricalResistance as UomElectricalResistance;

define_unit_family! {
    /// Units for persisted `uom` electrical resistance quantities.
    pub enum ElectricalResistance for "electrical_resistance", uom = UomElectricalResistance {
        /// Microohm (`µΩ`).
        Microohm => { symbol: "µΩ"; coefficient: 1 / 1000000; aliases: ["uΩ", "μΩ", "uOhm"]; uom: microohm; }
        /// Milliohm (`mΩ`).
        Milliohm => { symbol: "mΩ"; coefficient: 1 / 1000; aliases: ["mOhm"]; uom: milliohm; }
        /// Ohm (`Ω`).
        Ohm => { symbol: "Ω"; coefficient: 1; aliases: ["ohm", "Ohm"]; uom: ohm; }
        /// Kiloohm (`kΩ`).
        Kiloohm => { symbol: "kΩ"; coefficient: 1000; aliases: ["kOhm"]; uom: kiloohm; }
        /// Megaohm (`MΩ`).
        Megaohm => { symbol: "MΩ"; coefficient: 1000000; aliases: ["MOhm"]; uom: megaohm; }
        /// Gigaohm (`GΩ`).
        Gigaohm => { symbol: "GΩ"; coefficient: 1000000000; aliases: ["GOhm"]; uom: gigaohm; }
    }
}
