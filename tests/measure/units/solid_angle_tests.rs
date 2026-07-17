// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the solid angle unit family.

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
fn test_solid_angle_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::SolidAngle::Steradian,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::SolidAngle::Spat,
            numerator: "39269908169872415480783",
            denominator: "3125000000000000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::SolidAngle::SquareDegree,
            numerator: "609234839573417198693487",
            denominator: "2000000000000000000000000000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_solid_angle_unit_contract() {
    assert_unit_family_valid::<unit::SolidAngle>();
    assert_unit_contract::<unit::SolidAngle>();
}
