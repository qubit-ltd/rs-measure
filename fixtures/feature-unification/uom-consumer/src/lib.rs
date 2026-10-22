// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Uom-enabled member of the Cargo feature-unification fixture.

use qubit_measure::UomUnit;
use qubit_measure::define_unit_family;
use qubit_measure::impl_uom_unit;
use uom::si::f64::Length as UomLength;
use uom::si::length::meter;

define_unit_family! {
    /// External length family with an explicit uom bridge.
    ///
    /// # Examples
    ///
    /// ```
    /// use qubit_measure::Unit;
    /// use qubit_measure_feature_unification_uom::FixtureLength;
    ///
    /// assert_eq!(FixtureLength::Meter.symbol(), "m");
    /// ```
    pub enum FixtureLength for "fixture_length" {
        /// Meter fixture unit.
        Meter => {
            symbol: "m";
            coefficient: 1;
        }
    }
}

impl_uom_unit! {
    FixtureLength, UomLength {
        base: meter;
    }
}

/// Requires the external family to implement [`UomUnit`].
pub fn assert_uom_bridge_is_generated()
where
    FixtureLength: UomUnit,
{
}
