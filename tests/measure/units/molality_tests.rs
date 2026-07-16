// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the molality unit family.

use qubit_measure::{
    assert_unit_family_valid,
    unit,
};

use crate::measure::support::{
    DefinitionCase,
    assert_definition_cases,
    assert_unit_contract,
};

#[test]
fn test_molality_definitions_match_exact_golden_values() {
    assert_definition_cases(&[DefinitionCase {
        unit: unit::Molality::MolePerKilogram,
        numerator: "1",
        denominator: "1",
        offset: "0",
    }]);
}

#[test]
fn test_molality_unit_contract() {
    assert_unit_family_valid::<unit::Molality>();
    assert_unit_contract::<unit::Molality>();
}
