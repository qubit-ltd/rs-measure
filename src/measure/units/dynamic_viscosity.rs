// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted dynamic viscosity measurements.

use super::define_measurement_unit;
use uom::si::dynamic_viscosity::{
    centipoise,
    micropascal_second,
    millipascal_second,
    pascal_second,
    poise,
};
use uom::si::f64::DynamicViscosity as UomDynamicViscosity;

define_measurement_unit! {
    /// Units for persisted `uom` dynamic viscosity quantities.
    pub enum DynamicViscosity for UomDynamicViscosity, "dynamic viscosity" {
        /// Micropascal second (`µPa · s`).
        MicropascalSecond => "µPa · s" | "uPa · s" | "μPa · s" | "uPa*s", micropascal_second;
        /// Millipascal second (`mPa · s`).
        MillipascalSecond => "mPa · s" | "mPa*s", millipascal_second;
        /// Pascal second (`Pa · s`).
        PascalSecond => "Pa · s" | "Pa*s" | "Pa s", pascal_second;
        /// Poise (`P`).
        Poise => "P", poise;
        /// Centipoise (`cP`).
        Centipoise => "cP", centipoise;
    }
}
