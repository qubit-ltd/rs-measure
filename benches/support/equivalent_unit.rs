// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Unit definitions that are mathematically equal but structurally distinct.

use qubit_measure::define_unit_family;

define_unit_family! {
    /// Units used to benchmark mathematical-equivalence detection.
    pub enum EquivalentUnit for "benchmark_equivalent" {
        /// Factor ten expressed through division.
        DivideByPointOne => {
            symbol: "divide-by-point-one";
            coefficient: 1 / 0.1;
        }
        /// Integral factor ten.
        Ten => {
            symbol: "ten";
            coefficient: 10;
        }
    }
}
