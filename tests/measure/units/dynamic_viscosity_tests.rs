// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the dynamic viscosity unit family.

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
fn test_dynamic_viscosity_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::DynamicViscosity::MicropascalSecond,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::DynamicViscosity::MillipascalSecond,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::DynamicViscosity::PascalSecond,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::DynamicViscosity::Poise,
            numerator: "1",
            denominator: "10",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::DynamicViscosity::Centipoise,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_dynamic_viscosity_unit_contract() {
    assert_unit_family_valid::<unit::DynamicViscosity>();
    assert_unit_contract::<unit::DynamicViscosity>();
}
