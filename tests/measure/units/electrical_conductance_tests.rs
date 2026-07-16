// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the electrical conductance unit
//! family.

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
fn test_electrical_conductance_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::ElectricalConductance::Microsiemens,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricalConductance::Millisiemens,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricalConductance::Siemens,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
    ]);
}

#[test]
fn test_electrical_conductance_unit_contract() {
    assert_unit_family_valid::<unit::ElectricalConductance>();
    assert_unit_contract::<unit::ElectricalConductance>();
}
