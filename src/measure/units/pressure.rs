// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted pressure measurements.

#[cfg(feature = "uom")]
use super::internal::ExactTorrEquivalent;
use crate::define_unit_family;
#[cfg(feature = "uom")]
use uom::si::f64::Pressure as UomPressure;
#[cfg(feature = "uom")]
use uom::si::pressure::{
    atmosphere,
    bar,
    hectopascal,
    kilopascal,
    megapascal,
    micropascal,
    millibar,
    millipascal,
    nanopascal,
    pascal,
    psi,
};

define_unit_family! {
    /// Units for persisted pressure measurements.
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
        /// Millimeter of mercury using the exact Torr-equivalent definition
        /// 101325/760 Pa (20265/152 Pa), with canonical symbol `mm Hg`.
        ///
        /// The lenient alias is `mmHg`. This definition differs from the
        /// conventional rounded 133.3224 Pa value used by some conversion
        /// tables. The optional `uom` bridge uses an internal Torr-equivalent
        /// marker so it preserves this unit's Torr-equivalent semantic instead
        /// of using `uom`'s conventional millimeter-of-mercury coefficient.
        MillimeterOfMercury => { symbol: "mm Hg"; definition: crate::consts::pressure::MILLIMETER_OF_MERCURY; aliases: ["mmHg"]; uom: ExactTorrEquivalent; }
        /// Pound-force per square inch (`psi`).
        Psi => { symbol: "psi"; definition: crate::consts::pressure::PSI; uom: psi; }
    }
}
