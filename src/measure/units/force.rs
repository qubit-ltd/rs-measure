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
        Millinewton => { symbol: "mN"; coefficient: 1 / 1000; uom: millinewton; }
        /// Newton (`N`).
        Newton => { symbol: "N"; coefficient: 1; uom: newton; }
        /// Kilonewton (`kN`).
        Kilonewton => { symbol: "kN"; coefficient: 1000; uom: kilonewton; }
        /// Meganewton (`MN`).
        Meganewton => { symbol: "MN"; coefficient: 1000000; uom: meganewton; }
        /// Gram-force (`gf`).
        GramForce => { symbol: "gf"; coefficient: 196133 / 20000000; uom: gram_force; }
        /// Kilogram-force (`kgf`).
        KilogramForce => { symbol: "kgf"; coefficient: 196133 / 20000; uom: kilogram_force; }
        /// Pound-force (`lbf`).
        PoundForce => { symbol: "lbf"; coefficient: 8896443230521 / 2000000000000; uom: pound_force; }
    }
}
