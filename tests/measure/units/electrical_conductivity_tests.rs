// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the electrical conductivity unit
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
fn test_electrical_conductivity_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::ElectricalConductivity::SiemensPerMeter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricalConductivity::SiemensPerCentimeter,
            numerator: "100",
            denominator: "1",
            offset: "0",
        },
    ]);
}

#[test]
fn test_electrical_conductivity_unit_contract() {
    assert_unit_family_valid::<unit::ElectricalConductivity>();
    assert_unit_contract::<unit::ElectricalConductivity>();
}
