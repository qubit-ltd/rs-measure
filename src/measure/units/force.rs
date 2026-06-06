// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted force measurements.

use super::define_measurement_unit;
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

define_measurement_unit! {
    /// Units for persisted `uom` force quantities.
    pub enum Force for UomForce, "force" {
        /// Millinewton (`mN`).
        Millinewton => "mN", millinewton;
        /// Newton (`N`).
        Newton => "N", newton;
        /// Kilonewton (`kN`).
        Kilonewton => "kN", kilonewton;
        /// Meganewton (`MN`).
        Meganewton => "MN", meganewton;
        /// Gram-force (`gf`).
        GramForce => "gf", gram_force;
        /// Kilogram-force (`kgf`).
        KilogramForce => "kgf", kilogram_force;
        /// Pound-force (`lbf`).
        PoundForce => "lbf", pound_force;
    }
}
