// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the specific heat capacity unit
//! family.

use qubit_measure::assert_unit_family_valid;
use qubit_measure::unit;

use crate::measure::support::DefinitionCase;
use crate::measure::support::assert_definition_cases;
use crate::measure::support::assert_unit_contract;

#[test]
fn test_specific_heat_capacity_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::SpecificHeatCapacity::JoulePerKilogramKelvin,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::SpecificHeatCapacity::KilojoulePerKilogramKelvin,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::SpecificHeatCapacity::JoulePerGramDegreeCelsius,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::SpecificHeatCapacity::ThermochemicalCaloriePerGramKelvin,
            numerator: "4184",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::SpecificHeatCapacity::BritishThermalUnitInternationalTablePerPoundDegreeFahrenheit,
            numerator: "189910080000",
            denominator: "45359237",
            offset: "0",
        },
    ]);
}

#[test]
fn test_specific_heat_capacity_unit_contract() {
    assert_unit_family_valid::<unit::SpecificHeatCapacity>();
    assert_unit_contract::<unit::SpecificHeatCapacity>();
}
