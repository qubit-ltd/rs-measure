/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Units for persisted electrical resistivity measurements.

use super::define_measurement_unit;
use uom::si::electrical_resistivity::{
    milliohm_meter,
    ohm_centimeter,
    ohm_meter,
    ohm_square_millimeter_per_meter,
};
use uom::si::f64::ElectricalResistivity as UomElectricalResistivity;

define_measurement_unit! {
    /// Units for persisted `uom` electrical resistivity quantities.
    pub enum ElectricalResistivity for UomElectricalResistivity, "electrical resistivity" {
        /// Milliohm meter (`mΩ · m`).
        MilliohmMeter => "mΩ · m" | "mOhm m" | "mΩ*m", milliohm_meter;
        /// Ohm meter (`Ω · m`).
        OhmMeter => "Ω · m" | "ohm m" | "Ω*m", ohm_meter;
        /// Ohm centimeter (`Ω · cm`).
        OhmCentimeter => "Ω · cm" | "ohm cm" | "Ω*cm", ohm_centimeter;
        /// Ohm square millimeter per meter (`Ω · mm²/m`).
        OhmSquareMillimeterPerMeter => "Ω · mm²/m" | "Ω mm2/m" | "ohm mm2/m", ohm_square_millimeter_per_meter;
    }
}
