// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the temperature interval unit
//! family.

use qubit_measure::assert_unit_family_valid;
use qubit_measure::unit;

use crate::measure::support::DefinitionCase;
use crate::measure::support::assert_definition_cases;
use crate::measure::support::assert_unit_contract;

#[test]
fn test_temperature_interval_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::TemperatureInterval::Kelvin,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::TemperatureInterval::Celsius,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::TemperatureInterval::Fahrenheit,
            numerator: "5",
            denominator: "9",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::TemperatureInterval::Rankine,
            numerator: "5",
            denominator: "9",
            offset: "0",
        },
    ]);
}

#[test]
fn test_temperature_interval_unit_contract() {
    assert_unit_family_valid::<unit::TemperatureInterval>();
    assert_unit_contract::<unit::TemperatureInterval>();
}
