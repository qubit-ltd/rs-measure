// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the heat capacity unit family.

use qubit_measure::assert_unit_family_valid;
use qubit_measure::unit;

use crate::measure::support::DefinitionCase;
use crate::measure::support::assert_definition_cases;
use crate::measure::support::assert_unit_contract;

#[test]
fn test_heat_capacity_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::HeatCapacity::JoulePerKelvin,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::HeatCapacity::KilojoulePerKelvin,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::HeatCapacity::JoulePerDegreeCelsius,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::HeatCapacity::ThermochemicalCaloriePerKelvin,
            numerator: "523",
            denominator: "125",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::HeatCapacity::BritishThermalUnitInternationalTablePerDegreeFahrenheit,
            numerator: "1186938",
            denominator: "625",
            offset: "0",
        },
    ]);
}

#[test]
fn test_heat_capacity_unit_contract() {
    assert_unit_family_valid::<unit::HeatCapacity>();
    assert_unit_contract::<unit::HeatCapacity>();
}
