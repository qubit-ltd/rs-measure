// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the electric current density unit
//! family.

use qubit_measure::assert_unit_family_valid;
use qubit_measure::unit;

use crate::measure::support::DefinitionCase;
use crate::measure::support::assert_definition_cases;
use crate::measure::support::assert_unit_contract;

#[test]
fn test_electric_current_density_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::ElectricCurrentDensity::AmperePerSquareMeter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricCurrentDensity::AmperePerSquareCentimeter,
            numerator: "10000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricCurrentDensity::AmperePerSquareMillimeter,
            numerator: "1000000",
            denominator: "1",
            offset: "0",
        },
    ]);
}

#[test]
fn test_electric_current_density_unit_contract() {
    assert_unit_family_valid::<unit::ElectricCurrentDensity>();
    assert_unit_contract::<unit::ElectricCurrentDensity>();
}
