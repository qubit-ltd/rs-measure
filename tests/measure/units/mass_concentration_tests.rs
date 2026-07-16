// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the mass concentration unit family.

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
fn test_mass_concentration_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::MassConcentration::MicrogramPerLiter,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MassConcentration::MilligramPerLiter,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MassConcentration::GramPerLiter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MassConcentration::KilogramPerCubicMeter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MassConcentration::MilligramPerDeciliter,
            numerator: "1",
            denominator: "100",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MassConcentration::GramPerDeciliter,
            numerator: "10",
            denominator: "1",
            offset: "0",
        },
    ]);
}

#[test]
fn test_mass_concentration_unit_contract() {
    assert_unit_family_valid::<unit::MassConcentration>();
    assert_unit_contract::<unit::MassConcentration>();
}
