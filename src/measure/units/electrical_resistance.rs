/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Units for persisted electrical resistance measurements.

use super::define_measurement_unit;
use uom::si::electrical_resistance::{
    gigaohm,
    kiloohm,
    megaohm,
    microohm,
    milliohm,
    ohm,
};
use uom::si::f64::ElectricalResistance as UomElectricalResistance;

define_measurement_unit! {
    /// Units for persisted `uom` electrical resistance quantities.
    pub enum ElectricalResistance for UomElectricalResistance, "electrical resistance" {
        /// Microohm (`µΩ`).
        Microohm => "µΩ" | "uΩ" | "μΩ" | "uOhm", microohm;
        /// Milliohm (`mΩ`).
        Milliohm => "mΩ" | "mOhm", milliohm;
        /// Ohm (`Ω`).
        Ohm => "Ω" | "ohm" | "Ohm", ohm;
        /// Kiloohm (`kΩ`).
        Kiloohm => "kΩ" | "kOhm", kiloohm;
        /// Megaohm (`MΩ`).
        Megaohm => "MΩ" | "MOhm", megaohm;
        /// Gigaohm (`GΩ`).
        Gigaohm => "GΩ" | "GOhm", gigaohm;
    }
}
