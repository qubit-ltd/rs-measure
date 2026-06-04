/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Units for persisted pressure measurements.

use super::define_measurement_unit;
use uom::si::f64::Pressure as UomPressure;
use uom::si::pressure::{
    atmosphere,
    bar,
    kilopascal,
    megapascal,
    millibar,
    millimeter_of_mercury,
    pascal,
    psi,
};

define_measurement_unit! {
    /// Units for persisted `uom` pressure quantities.
    pub enum Pressure for UomPressure, "pressure" {
        /// Pascal (`Pa`).
        Pascal => "Pa", pascal;
        /// Kilopascal (`kPa`).
        Kilopascal => "kPa", kilopascal;
        /// Megapascal (`MPa`).
        Megapascal => "MPa", megapascal;
        /// Bar (`bar`).
        Bar => "bar", bar;
        /// Millibar (`mbar`).
        Millibar => "mbar", millibar;
        /// Standard atmosphere (`atm`).
        Atmosphere => "atm", atmosphere;
        /// Millimeter of mercury (`mm Hg`).
        MillimeterOfMercury => "mm Hg", millimeter_of_mercury;
        /// Pound-force per square inch (`psi`).
        Psi => "psi", psi;
    }
}
