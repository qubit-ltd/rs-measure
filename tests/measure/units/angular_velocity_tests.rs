// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the angular velocity unit family.

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
fn test_angular_velocity_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::AngularVelocity::RadianPerSecond,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::AngularVelocity::DegreePerSecond,
            numerator: "3141592653589793",
            denominator: "180000000000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::AngularVelocity::RevolutionPerSecond,
            numerator: "3141592653589793",
            denominator: "500000000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::AngularVelocity::RevolutionPerMinute,
            numerator: "3141592653589793",
            denominator: "30000000000000000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_angular_velocity_unit_contract() {
    assert_unit_family_valid::<unit::AngularVelocity>();
    assert_unit_contract::<unit::AngularVelocity>();
}
