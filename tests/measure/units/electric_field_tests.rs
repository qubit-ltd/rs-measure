// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the electric field unit family.

use qubit_measure::assert_unit_family_valid;
use qubit_measure::unit;

use crate::measure::support::DefinitionCase;
use crate::measure::support::assert_definition_cases;
use crate::measure::support::assert_unit_contract;

#[test]
fn test_electric_field_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::ElectricField::VoltPerMeter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricField::VoltPerCentimeter,
            numerator: "100",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricField::VoltPerMillimeter,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricField::VoltPerMicrometer,
            numerator: "1000000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricField::KilovoltPerMillimeter,
            numerator: "1000000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricField::MegavoltPerMeter,
            numerator: "1000000",
            denominator: "1",
            offset: "0",
        },
    ]);
}

#[test]
fn test_electric_field_unit_contract() {
    assert_unit_family_valid::<unit::ElectricField>();
    assert_unit_contract::<unit::ElectricField>();
}
