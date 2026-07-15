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
        Picofarad => { symbol: "pF"; coefficient: 1 / 1000000000000; uom: picofarad; }
        /// Nanofarad (`nF`).
        Nanofarad => { symbol: "nF"; coefficient: 1 / 1000000000; uom: nanofarad; }
        /// Microfarad (`µF`).
        Microfarad => { symbol: "µF"; coefficient: 1 / 1000000; aliases: ["uF", "μF"]; uom: microfarad; }
        /// Millifarad (`mF`).
        Millifarad => { symbol: "mF"; coefficient: 1 / 1000; uom: millifarad; }
        /// Farad (`F`).
        Farad => { symbol: "F"; coefficient: 1; uom: farad; }
    }
}
