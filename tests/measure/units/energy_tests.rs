// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the energy unit family.

use qubit_measure::assert_unit_family_valid;
use qubit_measure::unit;

use crate::measure::support::DefinitionCase;
use crate::measure::support::assert_definition_cases;
use crate::measure::support::assert_unit_contract;

#[test]
fn test_energy_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Energy::Joule,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Energy::Kilojoule,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Energy::Megajoule,
            numerator: "1000000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Energy::WattHour,
            numerator: "3600",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Energy::KilowattHour,
            numerator: "3600000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Energy::Electronvolt,
            numerator: "801088317",
            denominator: "5000000000000000000000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Energy::ThermochemicalCalorie,
            numerator: "523",
            denominator: "125",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Energy::ThermochemicalKilocalorie,
            numerator: "4184",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Energy::BritishThermalUnitInternationalTable,
            numerator: "131882",
            denominator: "125",
            offset: "0",
        },
    ]);
}

#[test]
fn test_energy_unit_contract() {
    assert_unit_family_valid::<unit::Energy>();
    assert_unit_contract::<unit::Energy>();
}
