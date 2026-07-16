// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the luminance unit family.

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
fn test_luminance_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Luminance::CandelaPerSquareMeter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Luminance::CandelaPerSquareCentimeter,
            numerator: "10000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Luminance::CandelaPerSquareFoot,
            numerator: "1562500",
            denominator: "145161",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Luminance::Footlambert,
            numerator: "6852518199270781",
            denominator: "2000000000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Luminance::Stilb,
            numerator: "10000",
            denominator: "1",
            offset: "0",
        },
    ]);
}

#[test]
fn test_luminance_unit_contract() {
    assert_unit_family_valid::<unit::Luminance>();
    assert_unit_contract::<unit::Luminance>();
}
