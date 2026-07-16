// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Unit family whose canonical symbol collides with another variant's alias.

use qubit_measure::define_unit_family;

define_unit_family! {
    /// Unit family used to verify canonical-symbol parsing priority.
    pub(crate) enum CanonicalPriorityUnit for "canonical_priority" {
        /// Variant that owns the colliding alias.
        AliasOwner => {
            symbol: "alias-owner";
            coefficient: 1;
            aliases: ["canonical"];
        }
        /// Variant that owns the canonical symbol.
        CanonicalOwner => {
            symbol: "canonical";
            coefficient: 2;
        }
    }
}
