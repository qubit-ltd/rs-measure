// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted electrical resistivity measurements.

use crate::define_unit_family;
use uom::si::electrical_resistivity::{
    milliohm_meter,
    ohm_centimeter,
    ohm_meter,
    ohm_square_millimeter_per_meter,
};
use uom::si::f64::ElectricalResistivity as UomElectricalResistivity;

define_unit_family! {
    /// Units for persisted `uom` electrical resistivity quantities.
    pub enum ElectricalResistivity for "electrical_resistivity", uom = UomElectricalResistivity {
        /// Milliohm meter (`mΩ · m`).
        MilliohmMeter => { symbol: "mΩ · m"; coefficient: 1 / 1000; aliases: ["mOhm m", "mΩ*m"]; uom: milliohm_meter; }
        /// Ohm meter (`Ω · m`).
        OhmMeter => { symbol: "Ω · m"; coefficient: 1; aliases: ["ohm m", "Ω*m"]; uom: ohm_meter; }
        /// Ohm centimeter (`Ω · cm`).
        OhmCentimeter => { symbol: "Ω · cm"; coefficient: 1 / 100; aliases: ["ohm cm", "Ω*cm"]; uom: ohm_centimeter; }
        /// Ohm square millimeter per meter (`Ω · mm²/m`).
        OhmSquareMillimeterPerMeter => { symbol: "Ω · mm²/m"; coefficient: 1 / 1000000; aliases: ["Ω mm2/m", "ohm mm2/m"]; uom: ohm_square_millimeter_per_meter; }
    }
}
