// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the velocity unit family.

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
fn test_velocity_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Velocity::MicrometerPerSecond,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Velocity::MillimeterPerSecond,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Velocity::CentimeterPerSecond,
            numerator: "1",
            denominator: "100",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Velocity::MeterPerSecond,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Velocity::KilometerPerHour,
            numerator: "5",
            denominator: "18",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Velocity::FootPerSecond,
            numerator: "381",
            denominator: "1250",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Velocity::MilePerHour,
            numerator: "1397",
            denominator: "3125",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Velocity::Knot,
            numerator: "463",
            denominator: "900",
            offset: "0",
        },
    ]);
}

#[test]
fn test_velocity_unit_contract() {
    assert_unit_family_valid::<unit::Velocity>();
    assert_unit_contract::<unit::Velocity>();
}
