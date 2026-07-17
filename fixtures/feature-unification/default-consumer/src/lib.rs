// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Default-feature member of the Cargo feature-unification fixture.

use qubit_measure::define_unit_family;

define_unit_family! {
    /// Exact-only external unit family.
    pub enum DefaultFixtureUnit for "default_fixture" {
        /// Base fixture unit.
        Base => {
            symbol: "dfu";
            coefficient: 1;
        }
    }
}
