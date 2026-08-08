// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted dynamic viscosity measurements.

#[cfg(feature = "uom")]
use uom::si::dynamic_viscosity::pascal_second;
#[cfg(feature = "uom")]
use uom::si::f64::DynamicViscosity as UomDynamicViscosity;

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;

define_unit_family! {
    /// Units for persisted dynamic viscosity measurements.
    pub enum DynamicViscosity for "dynamic_viscosity" {
        /// Micropascal second (`µPa · s`).
        MicropascalSecond => { symbol: "µPa · s"; definition: crate::consts::dynamic_viscosity::MICROPASCAL_SECOND; aliases: ["uPa · s", "μPa · s", "uPa*s"]; }
        /// Millipascal second (`mPa · s`).
        MillipascalSecond => { symbol: "mPa · s"; definition: crate::consts::dynamic_viscosity::MILLIPASCAL_SECOND; aliases: ["mPa*s"]; }
        /// Pascal second (`Pa · s`).
        PascalSecond => { symbol: "Pa · s"; definition: crate::consts::dynamic_viscosity::PASCAL_SECOND; aliases: ["Pa*s", "Pa s"]; }
        /// Poise (`P`).
        Poise => { symbol: "P"; definition: crate::consts::dynamic_viscosity::POISE; }
        /// Centipoise (`cP`).
        Centipoise => { symbol: "cP"; definition: crate::consts::dynamic_viscosity::CENTIPOISE; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    DynamicViscosity, UomDynamicViscosity {
        base: pascal_second;
    }
}
