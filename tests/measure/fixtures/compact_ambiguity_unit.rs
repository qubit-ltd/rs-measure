// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! External unit family exercising compact-suffix ambiguity.

use qubit_measure::define_unit_family;

define_unit_family! {
    /// Units whose symbols overlap valid Decimal suffix text.
    pub enum CompactAmbiguityUnit for "compact_ambiguity" {
        /// Single-letter suffix.
        X => { symbol: "x"; coefficient: 1; }
        /// Digit-leading suffix sharing `x`.
        TwoX => { symbol: "2x"; coefficient: 2; }
        /// Longer digit-leading suffix sharing `2x` and `x`.
        TwelveX => { symbol: "12x"; coefficient: 12; }
        /// Exponent-like suffix.
        ExponentLike => { symbol: "e3"; coefficient: 3; }
    }
}
