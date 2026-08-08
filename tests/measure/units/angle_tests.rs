// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the angle unit family.

use qubit_measure::assert_unit_family_valid;
use qubit_measure::unit;

use crate::measure::support::DefinitionCase;
use crate::measure::support::assert_definition_cases;
use crate::measure::support::assert_unit_contract;

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
            numerator: "39269908169872415480783",
            denominator: "2250000000000000000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Angle::Revolution,
            numerator: "39269908169872415480783",
            denominator: "6250000000000000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Angle::Minute,
            numerator: "39269908169872415480783",
            denominator: "135000000000000000000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Angle::Second,
            numerator: "39269908169872415480783",
            denominator: "8100000000000000000000000000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_angle_unit_contract() {
    assert_unit_family_valid::<unit::Angle>();
    assert_unit_contract::<unit::Angle>();
}
