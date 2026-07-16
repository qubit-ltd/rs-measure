// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the acceleration unit family.

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
fn test_acceleration_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Acceleration::MillimeterPerSecondSquared,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Acceleration::MeterPerSecondSquared,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Acceleration::FootPerSecondSquared,
            numerator: "381",
            denominator: "1250",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Acceleration::StandardGravity,
            numerator: "196133",
            denominator: "20000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_acceleration_unit_contract() {
    assert_unit_family_valid::<unit::Acceleration>();
    assert_unit_contract::<unit::Acceleration>();
}
