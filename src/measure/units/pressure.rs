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
use crate::impl_uom_unit;
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
    pub enum Pressure for "pressure" {
        /// Nanopascal (`nPa`).
        Nanopascal => { symbol: "nPa"; definition: crate::consts::pressure::NANOPASCAL; }
        /// Micropascal (`µPa`).
        Micropascal => { symbol: "µPa"; definition: crate::consts::pressure::MICROPASCAL; aliases: ["uPa", "μPa"]; }
        /// Millipascal (`mPa`).
        Millipascal => { symbol: "mPa"; definition: crate::consts::pressure::MILLIPASCAL; }
        /// Pascal (`Pa`).
        Pascal => { symbol: "Pa"; definition: crate::consts::pressure::PASCAL; }
        /// Hectopascal (`hPa`).
        Hectopascal => { symbol: "hPa"; definition: crate::consts::pressure::HECTOPASCAL; }
        /// Kilopascal (`kPa`).
        Kilopascal => { symbol: "kPa"; definition: crate::consts::pressure::KILOPASCAL; }
        /// Megapascal (`MPa`).
        Megapascal => { symbol: "MPa"; definition: crate::consts::pressure::MEGAPASCAL; }
        /// Bar (`bar`).
        Bar => { symbol: "bar"; definition: crate::consts::pressure::BAR; }
        /// Millibar (`mbar`).
        Millibar => { symbol: "mbar"; definition: crate::consts::pressure::MILLIBAR; }
        /// Standard atmosphere (`atm`).
        Atmosphere => { symbol: "atm"; definition: crate::consts::pressure::ATMOSPHERE; }
        /// Millimeter of mercury using the exact Torr-equivalent definition
        /// 101325/760 Pa (20265/152 Pa), with canonical symbol `mm Hg`.
        ///
        /// The lenient alias is `mmHg`. This definition differs from the
        /// conventional rounded 133.3224 Pa value used by some conversion
        /// tables. The optional `uom` bridge uses an internal Torr-equivalent
        /// marker so it preserves this unit's Torr-equivalent semantic instead
        /// of using `uom`'s conventional millimeter-of-mercury coefficient.
        MillimeterOfMercury => { symbol: "mm Hg"; definition: crate::consts::pressure::MILLIMETER_OF_MERCURY; aliases: ["mmHg"]; }
        /// Pound-force per square inch (`psi`).
        Psi => { symbol: "psi"; definition: crate::consts::pressure::PSI; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    Pressure, UomPressure {
        Nanopascal => nanopascal;
        Micropascal => micropascal;
        Millipascal => millipascal;
        Pascal => pascal;
        Hectopascal => hectopascal;
        Kilopascal => kilopascal;
        Megapascal => megapascal;
        Bar => bar;
        Millibar => millibar;
        Atmosphere => atmosphere;
        MillimeterOfMercury => ExactTorrEquivalent;
        Psi => psi;
    }
}
