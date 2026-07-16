// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the torque unit family.

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
fn test_torque_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Torque::MillinewtonMeter,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Torque::NewtonMeter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Torque::KilonewtonMeter,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Torque::PoundForceFoot,
            numerator: "3389544870828501",
            denominator: "2500000000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Torque::PoundForceInch,
            numerator: "1129848290276167",
            denominator: "10000000000000000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_torque_unit_contract() {
    assert_unit_family_valid::<unit::Torque>();
    assert_unit_contract::<unit::Torque>();
}
