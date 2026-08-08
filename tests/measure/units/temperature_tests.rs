// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the temperature unit family.

use qubit_measure::assert_unit_family_valid;
use qubit_measure::unit;

use crate::measure::support::DefinitionCase;
use crate::measure::support::assert_definition_cases;
use crate::measure::support::assert_unit_contract;

#[test]
fn test_temperature_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Temperature::Kelvin,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Temperature::Celsius,
            numerator: "1",
            denominator: "1",
            offset: "273.15",
        },
        DefinitionCase {
            unit: unit::Temperature::Fahrenheit,
            numerator: "5",
            denominator: "9",
            offset: "459.67",
        },
        DefinitionCase {
            unit: unit::Temperature::Rankine,
            numerator: "5",
            denominator: "9",
            offset: "0",
        },
    ]);
}

#[test]
fn test_temperature_unit_contract() {
    assert_unit_family_valid::<unit::Temperature>();
    assert_unit_contract::<unit::Temperature>();
}
