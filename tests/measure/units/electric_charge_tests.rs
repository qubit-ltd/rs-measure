// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the electric charge unit family.

use qubit_measure::assert_unit_family_valid;
use qubit_measure::unit;

use crate::measure::support::DefinitionCase;
use crate::measure::support::assert_definition_cases;
use crate::measure::support::assert_unit_contract;

#[test]
fn test_electric_charge_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::ElectricCharge::Microcoulomb,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricCharge::Millicoulomb,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricCharge::Coulomb,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricCharge::Kilocoulomb,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricCharge::AmpereHour,
            numerator: "3600",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricCharge::MilliampereHour,
            numerator: "18",
            denominator: "5",
            offset: "0",
        },
    ]);
}

#[test]
fn test_electric_charge_unit_contract() {
    assert_unit_family_valid::<unit::ElectricCharge>();
    assert_unit_contract::<unit::ElectricCharge>();
}
