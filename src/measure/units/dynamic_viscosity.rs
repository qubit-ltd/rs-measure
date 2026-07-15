// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted dynamic viscosity measurements.

use crate::define_unit_family;
use uom::si::dynamic_viscosity::{
    centipoise,
    micropascal_second,
    millipascal_second,
    pascal_second,
    poise,
};
use uom::si::f64::DynamicViscosity as UomDynamicViscosity;

define_unit_family! {
    /// Units for persisted `uom` dynamic viscosity quantities.
    pub enum DynamicViscosity for "dynamic_viscosity", uom = UomDynamicViscosity {
        /// Micropascal second (`µPa · s`).
        MicropascalSecond => { symbol: "µPa · s"; coefficient: 1 / 1000000; aliases: ["uPa · s", "μPa · s", "uPa*s"]; uom: micropascal_second; }
        /// Millipascal second (`mPa · s`).
        MillipascalSecond => { symbol: "mPa · s"; coefficient: 1 / 1000; aliases: ["mPa*s"]; uom: millipascal_second; }
        /// Pascal second (`Pa · s`).
        PascalSecond => { symbol: "Pa · s"; coefficient: 1; aliases: ["Pa*s", "Pa s"]; uom: pascal_second; }
        /// Poise (`P`).
        Poise => { symbol: "P"; coefficient: 1 / 10; uom: poise; }
        /// Centipoise (`cP`).
        Centipoise => { symbol: "cP"; coefficient: 1 / 1000; uom: centipoise; }
    }
}
