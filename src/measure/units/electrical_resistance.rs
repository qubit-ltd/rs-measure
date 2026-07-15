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
        Microohm => { symbol: "µΩ"; definition: crate::consts::electrical_resistance::MICROOHM; aliases: ["uΩ", "μΩ", "uOhm"]; uom: microohm; }
        /// Milliohm (`mΩ`).
        Milliohm => { symbol: "mΩ"; definition: crate::consts::electrical_resistance::MILLIOHM; aliases: ["mOhm"]; uom: milliohm; }
        /// Ohm (`Ω`).
        Ohm => { symbol: "Ω"; definition: crate::consts::electrical_resistance::OHM; aliases: ["ohm", "Ohm"]; uom: ohm; }
        /// Kiloohm (`kΩ`).
        Kiloohm => { symbol: "kΩ"; definition: crate::consts::electrical_resistance::KILOOHM; aliases: ["kOhm"]; uom: kiloohm; }
        /// Megaohm (`MΩ`).
        Megaohm => { symbol: "MΩ"; definition: crate::consts::electrical_resistance::MEGAOHM; aliases: ["MOhm"]; uom: megaohm; }
        /// Gigaohm (`GΩ`).
        Gigaohm => { symbol: "GΩ"; definition: crate::consts::electrical_resistance::GIGAOHM; aliases: ["GOhm"]; uom: gigaohm; }
    }
}
