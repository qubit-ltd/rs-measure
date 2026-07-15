// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted force measurements.

use crate::define_unit_family;
use uom::si::f64::Force as UomForce;
use uom::si::force::{
    gram_force,
    kilogram_force,
    kilonewton,
    meganewton,
    millinewton,
    newton,
    pound_force,
};

define_unit_family! {
    /// Units for persisted `uom` force quantities.
    pub enum Force for "force", uom = UomForce {
        /// Millinewton (`mN`).
        Millinewton => { symbol: "mN"; definition: crate::consts::force::MILLINEWTON; uom: millinewton; }
        /// Newton (`N`).
        Newton => { symbol: "N"; definition: crate::consts::force::NEWTON; uom: newton; }
        /// Kilonewton (`kN`).
        Kilonewton => { symbol: "kN"; definition: crate::consts::force::KILONEWTON; uom: kilonewton; }
        /// Meganewton (`MN`).
        Meganewton => { symbol: "MN"; definition: crate::consts::force::MEGANEWTON; uom: meganewton; }
        /// Gram-force (`gf`).
        GramForce => { symbol: "gf"; definition: crate::consts::force::GRAM_FORCE; uom: gram_force; }
        /// Kilogram-force (`kgf`).
        KilogramForce => { symbol: "kgf"; definition: crate::consts::force::KILOGRAM_FORCE; uom: kilogram_force; }
        /// Pound-force (`lbf`).
        PoundForce => { symbol: "lbf"; definition: crate::consts::force::POUND_FORCE; uom: pound_force; }
    }
}
