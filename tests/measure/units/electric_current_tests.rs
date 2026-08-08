// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the electric current unit family.

use qubit_measure::assert_unit_family_valid;
use qubit_measure::unit;

use crate::measure::support::DefinitionCase;
use crate::measure::support::assert_definition_cases;
use crate::measure::support::assert_unit_contract;

#[test]
fn test_electric_current_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::ElectricCurrent::Picoampere,
            numerator: "1",
            denominator: "1000000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricCurrent::Nanoampere,
            numerator: "1",
            denominator: "1000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricCurrent::Microampere,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricCurrent::Milliampere,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricCurrent::Ampere,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricCurrent::Kiloampere,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricCurrent::Megaampere,
            numerator: "1000000",
            denominator: "1",
            offset: "0",
        },
    ]);
}

#[test]
fn test_electric_current_unit_contract() {
    assert_unit_family_valid::<unit::ElectricCurrent>();
    assert_unit_contract::<unit::ElectricCurrent>();
}
