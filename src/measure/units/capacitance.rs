// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted capacitance measurements.

use crate::define_unit_family;
use uom::si::capacitance::{
    farad,
    microfarad,
    millifarad,
    nanofarad,
    picofarad,
};
use uom::si::f64::Capacitance as UomCapacitance;

define_unit_family! {
    /// Units for persisted `uom` capacitance quantities.
    pub enum Capacitance for "capacitance", uom = UomCapacitance {
        /// Picofarad (`pF`).
        Picofarad => { symbol: "pF"; definition: crate::consts::capacitance::PICOFARAD; uom: picofarad; }
        /// Nanofarad (`nF`).
        Nanofarad => { symbol: "nF"; definition: crate::consts::capacitance::NANOFARAD; uom: nanofarad; }
        /// Microfarad (`µF`).
        Microfarad => { symbol: "µF"; definition: crate::consts::capacitance::MICROFARAD; aliases: ["uF", "μF"]; uom: microfarad; }
        /// Millifarad (`mF`).
        Millifarad => { symbol: "mF"; definition: crate::consts::capacitance::MILLIFARAD; uom: millifarad; }
        /// Farad (`F`).
        Farad => { symbol: "F"; definition: crate::consts::capacitance::FARAD; uom: farad; }
    }
}
