// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted molar volume measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use uom::si::f64::MolarVolume as UomMolarVolume;
#[cfg(feature = "uom")]
use uom::si::molar_volume::{
    cubic_centimeter_per_mole,
    cubic_decimeter_per_mole,
    cubic_meter_per_mole,
};

define_unit_family! {
    /// Units for persisted molar volume measurements.
    pub enum MolarVolume for "molar_volume", uom = UomMolarVolume {
        /// Cubic centimeter per mole (`cm³/mol`).
        CubicCentimeterPerMole => { symbol: "cm³/mol"; definition: crate::consts::molar_volume::CUBIC_CENTIMETER_PER_MOLE; aliases: ["cm3/mol", "cm^3/mol"]; uom: cubic_centimeter_per_mole; }
        /// Cubic decimeter per mole (`dm³/mol`).
        CubicDecimeterPerMole => { symbol: "dm³/mol"; definition: crate::consts::molar_volume::CUBIC_DECIMETER_PER_MOLE; aliases: ["dm3/mol", "dm^3/mol"]; uom: cubic_decimeter_per_mole; }
        /// Cubic meter per mole (`m³/mol`).
        CubicMeterPerMole => { symbol: "m³/mol"; definition: crate::consts::molar_volume::CUBIC_METER_PER_MOLE; aliases: ["m3/mol", "m^3/mol"]; uom: cubic_meter_per_mole; }
    }
}
