// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted electrical resistivity measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use uom::si::electrical_resistivity::{
    milliohm_meter,
    ohm_centimeter,
    ohm_meter,
    ohm_square_millimeter_per_meter,
};
#[cfg(feature = "uom")]
use uom::si::f64::ElectricalResistivity as UomElectricalResistivity;

define_unit_family! {
    /// Units for persisted electrical resistivity measurements.
    pub enum ElectricalResistivity for "electrical_resistivity", uom = UomElectricalResistivity {
        /// Milliohm meter (`mΩ · m`).
        MilliohmMeter => { symbol: "mΩ · m"; definition: crate::consts::electrical_resistivity::MILLIOHM_METER; aliases: ["mOhm m", "mΩ*m"]; uom: milliohm_meter; }
        /// Ohm meter (`Ω · m`).
        OhmMeter => { symbol: "Ω · m"; definition: crate::consts::electrical_resistivity::OHM_METER; aliases: ["ohm m", "Ω*m"]; uom: ohm_meter; }
        /// Ohm centimeter (`Ω · cm`).
        OhmCentimeter => { symbol: "Ω · cm"; definition: crate::consts::electrical_resistivity::OHM_CENTIMETER; aliases: ["ohm cm", "Ω*cm"]; uom: ohm_centimeter; }
        /// Ohm square millimeter per meter (`Ω · mm²/m`).
        OhmSquareMillimeterPerMeter => { symbol: "Ω · mm²/m"; definition: crate::consts::electrical_resistivity::OHM_SQUARE_MILLIMETER_PER_METER; aliases: ["Ω mm2/m", "ohm mm2/m"]; uom: ohm_square_millimeter_per_meter; }
    }
}
