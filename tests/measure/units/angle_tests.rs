// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the angle unit family.

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
fn test_angle_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Angle::Radian,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Angle::Degree,
            numerator: "3141592653589793",
            denominator: "180000000000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Angle::Revolution,
            numerator: "3141592653589793",
            denominator: "500000000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Angle::Minute,
            numerator: "3141592653589793",
            denominator: "10800000000000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Angle::Second,
            numerator: "3141592653589793",
            denominator: "648000000000000000000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_angle_unit_contract() {
    assert_unit_family_valid::<unit::Angle>();
    assert_unit_contract::<unit::Angle>();
}
