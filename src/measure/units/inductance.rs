// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted inductance measurements.

#[cfg(feature = "uom")]
use uom::si::f64::Inductance as UomInductance;
#[cfg(feature = "uom")]
use uom::si::inductance::henry;

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;

define_unit_family! {
    /// Units for persisted inductance measurements.
    pub enum Inductance for "inductance" {
        /// Nanohenry (`nH`).
        Nanohenry => { symbol: "nH"; definition: crate::consts::inductance::NANOHENRY; }
        /// Microhenry (`µH`).
        Microhenry => { symbol: "µH"; definition: crate::consts::inductance::MICROHENRY; aliases: ["uH", "μH"]; }
        /// Millihenry (`mH`).
        Millihenry => { symbol: "mH"; definition: crate::consts::inductance::MILLIHENRY; }
        /// Henry (`H`).
        Henry => { symbol: "H"; definition: crate::consts::inductance::HENRY; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    Inductance, UomInductance {
        base: henry;
    }
}
