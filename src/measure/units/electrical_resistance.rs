// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted electrical resistance measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;
#[cfg(feature = "uom")]
use uom::si::electrical_resistance::ohm;
#[cfg(feature = "uom")]
use uom::si::f64::ElectricalResistance as UomElectricalResistance;

define_unit_family! {
    /// Units for persisted electrical resistance measurements.
    pub enum ElectricalResistance for "electrical_resistance" {
        /// Microohm (`µΩ`).
        Microohm => { symbol: "µΩ"; definition: crate::consts::electrical_resistance::MICROOHM; aliases: ["uΩ", "μΩ", "uOhm"]; }
        /// Milliohm (`mΩ`).
        Milliohm => { symbol: "mΩ"; definition: crate::consts::electrical_resistance::MILLIOHM; aliases: ["mOhm"]; }
        /// Ohm (`Ω`).
        Ohm => { symbol: "Ω"; definition: crate::consts::electrical_resistance::OHM; aliases: ["ohm", "Ohm"]; }
        /// Kiloohm (`kΩ`).
        Kiloohm => { symbol: "kΩ"; definition: crate::consts::electrical_resistance::KILOOHM; aliases: ["kOhm"]; }
        /// Megaohm (`MΩ`).
        Megaohm => { symbol: "MΩ"; definition: crate::consts::electrical_resistance::MEGAOHM; aliases: ["MOhm"]; }
        /// Gigaohm (`GΩ`).
        Gigaohm => { symbol: "GΩ"; definition: crate::consts::electrical_resistance::GIGAOHM; aliases: ["GOhm"]; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    ElectricalResistance, UomElectricalResistance {
        base: ohm;
    }
}
