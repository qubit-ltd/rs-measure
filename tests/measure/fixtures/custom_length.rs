// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Macro-generated external length unit family used by integration tests.

use qubit_measure::define_unit_family;

define_unit_family! {
    /// Custom length units used to exercise the exported family macro.
    pub enum CustomLength for "custom_length" {
        /// Base custom unit.
        Base => {
            symbol: "cu";
            coefficient: 1;
        }
        /// Half of one custom unit.
        Half => {
            symbol: "hcu";
            coefficient: 1 / 2;
            aliases: ["half-cu"];
        }
    }
}
