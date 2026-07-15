// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted inductance measurements.

use crate::define_unit_family;
use uom::si::f64::Inductance as UomInductance;
use uom::si::inductance::{
    henry,
    microhenry,
    millihenry,
    nanohenry,
};

define_unit_family! {
    /// Units for persisted `uom` inductance quantities.
    pub enum Inductance for "inductance", uom = UomInductance {
        /// Nanohenry (`nH`).
        Nanohenry => { symbol: "nH"; coefficient: 1 / 1000000000; uom: nanohenry; }
        /// Microhenry (`µH`).
        Microhenry => { symbol: "µH"; coefficient: 1 / 1000000; aliases: ["uH", "μH"]; uom: microhenry; }
        /// Millihenry (`mH`).
        Millihenry => { symbol: "mH"; coefficient: 1 / 1000; uom: millihenry; }
        /// Henry (`H`).
        Henry => { symbol: "H"; coefficient: 1; uom: henry; }
    }
}
