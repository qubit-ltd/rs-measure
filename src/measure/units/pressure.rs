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
        Nanopascal => { symbol: "nPa"; definition: crate::consts::pressure::NANOPASCAL; uom: nanopascal; }
        /// Micropascal (`µPa`).
        Micropascal => { symbol: "µPa"; definition: crate::consts::pressure::MICROPASCAL; aliases: ["uPa", "μPa"]; uom: micropascal; }
        /// Millipascal (`mPa`).
        Millipascal => { symbol: "mPa"; definition: crate::consts::pressure::MILLIPASCAL; uom: millipascal; }
        /// Pascal (`Pa`).
        Pascal => { symbol: "Pa"; definition: crate::consts::pressure::PASCAL; uom: pascal; }
        /// Hectopascal (`hPa`).
        Hectopascal => { symbol: "hPa"; definition: crate::consts::pressure::HECTOPASCAL; uom: hectopascal; }
        /// Kilopascal (`kPa`).
        Kilopascal => { symbol: "kPa"; definition: crate::consts::pressure::KILOPASCAL; uom: kilopascal; }
        /// Megapascal (`MPa`).
        Megapascal => { symbol: "MPa"; definition: crate::consts::pressure::MEGAPASCAL; uom: megapascal; }
        /// Bar (`bar`).
        Bar => { symbol: "bar"; definition: crate::consts::pressure::BAR; uom: bar; }
        /// Millibar (`mbar`).
        Millibar => { symbol: "mbar"; definition: crate::consts::pressure::MILLIBAR; uom: millibar; }
        /// Standard atmosphere (`atm`).
        Atmosphere => { symbol: "atm"; definition: crate::consts::pressure::ATMOSPHERE; uom: atmosphere; }
        /// Millimeter of mercury (`mm Hg`).
        MillimeterOfMercury => { symbol: "mm Hg"; definition: crate::consts::pressure::MILLIMETER_OF_MERCURY; aliases: ["mmHg"]; uom: millimeter_of_mercury; }
        /// Pound-force per square inch (`psi`).
        Psi => { symbol: "psi"; definition: crate::consts::pressure::PSI; uom: psi; }
    }
}
