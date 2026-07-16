// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the capacitance unit family.

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
fn test_capacitance_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Capacitance::Picofarad,
            numerator: "1",
            denominator: "1000000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Capacitance::Nanofarad,
            numerator: "1",
            denominator: "1000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Capacitance::Microfarad,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Capacitance::Millifarad,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Capacitance::Farad,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
    ]);
}

#[test]
fn test_capacitance_unit_contract() {
    assert_unit_family_valid::<unit::Capacitance>();
    assert_unit_contract::<unit::Capacitance>();
}
