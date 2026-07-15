// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted pressure measurements.

use crate::define_unit_family;
use uom::si::f64::Pressure as UomPressure;
use uom::si::pressure::{
    atmosphere,
    bar,
    hectopascal,
    kilopascal,
    megapascal,
    micropascal,
    millibar,
    millimeter_of_mercury,
    millipascal,
    nanopascal,
    pascal,
    psi,
};

define_unit_family! {
    /// Units for persisted `uom` pressure quantities.
    pub enum Pressure for "pressure", uom = UomPressure {
        /// Nanopascal (`nPa`).
        Nanopascal => { symbol: "nPa"; coefficient: 1 / 1000000000; uom: nanopascal; }
        /// Micropascal (`µPa`).
        Micropascal => { symbol: "µPa"; coefficient: 1 / 1000000; aliases: ["uPa", "μPa"]; uom: micropascal; }
        /// Millipascal (`mPa`).
        Millipascal => { symbol: "mPa"; coefficient: 1 / 1000; uom: millipascal; }
        /// Pascal (`Pa`).
        Pascal => { symbol: "Pa"; coefficient: 1; uom: pascal; }
        /// Hectopascal (`hPa`).
        Hectopascal => { symbol: "hPa"; coefficient: 100; uom: hectopascal; }
        /// Kilopascal (`kPa`).
        Kilopascal => { symbol: "kPa"; coefficient: 1000; uom: kilopascal; }
        /// Megapascal (`MPa`).
        Megapascal => { symbol: "MPa"; coefficient: 1000000; uom: megapascal; }
        /// Bar (`bar`).
        Bar => { symbol: "bar"; coefficient: 100000; uom: bar; }
        /// Millibar (`mbar`).
        Millibar => { symbol: "mbar"; coefficient: 100; uom: millibar; }
        /// Standard atmosphere (`atm`).
        Atmosphere => { symbol: "atm"; coefficient: 101325; uom: atmosphere; }
        /// Millimeter of mercury (`mm Hg`).
        MillimeterOfMercury => { symbol: "mm Hg"; coefficient: 20265 / 152; aliases: ["mmHg"]; uom: millimeter_of_mercury; }
        /// Pound-force per square inch (`psi`).
        Psi => { symbol: "psi"; coefficient: 8896443230521 / 1290320000; uom: psi; }
    }
}
