// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the amount of substance unit
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
fn test_amount_of_substance_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::AmountOfSubstance::Micromole,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::AmountOfSubstance::Millimole,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::AmountOfSubstance::Mole,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::AmountOfSubstance::Kilomole,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::AmountOfSubstance::Particle,
            numerator: "1",
            denominator: "602214076000000000000000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_amount_of_substance_unit_contract() {
    assert_unit_family_valid::<unit::AmountOfSubstance>();
    assert_unit_contract::<unit::AmountOfSubstance>();
}
