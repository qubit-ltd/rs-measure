// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Uom-enabled downstream macro fixture without a local `uom` feature.

use qubit_measure::{
    UomUnit,
    define_unit_family,
};
use uom::si::f64::Length as UomLength;
use uom::si::length::meter;

define_unit_family! {
    /// External length family used to verify dependency-owned features.
    pub enum DownstreamLength for "downstream_length", uom = UomLength {
        /// Meter fixture unit.
        Meter => {
            symbol: "m";
            coefficient: 1;
            uom: meter;
        }
    }
}

/// Requires the generated downstream family to implement `UomUnit`.
pub fn assert_uom_bridge_is_generated()
where
    DownstreamLength: UomUnit,
{
}
