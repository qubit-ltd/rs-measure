// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted molar volume measurements.

use super::define_measurement_unit;
use uom::si::f64::MolarVolume as UomMolarVolume;
use uom::si::molar_volume::{
    cubic_centimeter_per_mole,
    cubic_decimeter_per_mole,
    cubic_meter_per_mole,
};

define_measurement_unit! {
    /// Units for persisted `uom` molar volume quantities.
    pub enum MolarVolume for UomMolarVolume, "molar volume" {
        /// Cubic centimeter per mole (`cm³/mol`).
        CubicCentimeterPerMole => "cm³/mol" | "cm3/mol" | "cm^3/mol", cubic_centimeter_per_mole;
        /// Cubic decimeter per mole (`dm³/mol`).
        CubicDecimeterPerMole => "dm³/mol" | "dm3/mol" | "dm^3/mol", cubic_decimeter_per_mole;
        /// Cubic meter per mole (`m³/mol`).
        CubicMeterPerMole => "m³/mol" | "m3/mol" | "m^3/mol", cubic_meter_per_mole;
    }
}
