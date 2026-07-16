// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the kinematic viscosity unit
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
fn test_kinematic_viscosity_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::KinematicViscosity::SquareMillimeterPerSecond,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::KinematicViscosity::SquareMeterPerSecond,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::KinematicViscosity::Stokes,
            numerator: "1",
            denominator: "10000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::KinematicViscosity::Centistokes,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_kinematic_viscosity_unit_contract() {
    assert_unit_family_valid::<unit::KinematicViscosity>();
    assert_unit_contract::<unit::KinematicViscosity>();
}
