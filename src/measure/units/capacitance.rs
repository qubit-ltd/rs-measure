// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted capacitance measurements.

#[cfg(feature = "uom")]
use uom::si::capacitance::farad;
#[cfg(feature = "uom")]
use uom::si::f64::Capacitance as UomCapacitance;

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;

define_unit_family! {
    /// Units for persisted capacitance measurements.
    pub enum Capacitance for "capacitance" {
        /// Picofarad (`pF`).
        Picofarad => { symbol: "pF"; definition: crate::consts::capacitance::PICOFARAD; }
        /// Nanofarad (`nF`).
        Nanofarad => { symbol: "nF"; definition: crate::consts::capacitance::NANOFARAD; }
        /// Microfarad (`µF`).
        Microfarad => { symbol: "µF"; definition: crate::consts::capacitance::MICROFARAD; aliases: ["uF", "μF"]; }
        /// Millifarad (`mF`).
        Millifarad => { symbol: "mF"; definition: crate::consts::capacitance::MILLIFARAD; }
        /// Farad (`F`).
        Farad => { symbol: "F"; definition: crate::consts::capacitance::FARAD; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    Capacitance, UomCapacitance {
        base: farad;
    }
}
