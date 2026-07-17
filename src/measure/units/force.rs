// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted force measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;
#[cfg(feature = "uom")]
use uom::si::f64::Force as UomForce;
#[cfg(feature = "uom")]
use uom::si::force::newton;

define_unit_family! {
    /// Units for persisted force measurements.
    pub enum Force for "force" {
        /// Millinewton (`mN`).
        Millinewton => { symbol: "mN"; definition: crate::consts::force::MILLINEWTON; }
        /// Newton (`N`).
        Newton => { symbol: "N"; definition: crate::consts::force::NEWTON; }
        /// Kilonewton (`kN`).
        Kilonewton => { symbol: "kN"; definition: crate::consts::force::KILONEWTON; }
        /// Meganewton (`MN`).
        Meganewton => { symbol: "MN"; definition: crate::consts::force::MEGANEWTON; }
        /// Gram-force (`gf`).
        GramForce => { symbol: "gf"; definition: crate::consts::force::GRAM_FORCE; }
        /// Kilogram-force (`kgf`).
        KilogramForce => { symbol: "kgf"; definition: crate::consts::force::KILOGRAM_FORCE; }
        /// Pound-force (`lbf`).
        PoundForce => { symbol: "lbf"; definition: crate::consts::force::POUND_FORCE; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    Force, UomForce {
        base: newton;
    }
}
