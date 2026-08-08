// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Unit-family module aggregation tests.

use qubit_measure::Unit;
use qubit_measure::unit;

/// Verifies that the public unit module aggregates the Length family.
#[test]
fn test_unit_module_aggregates_length_family() {
    assert_eq!(unit::Length::QUANTITY, "length");
    assert!(!unit::Length::all().is_empty());
}
