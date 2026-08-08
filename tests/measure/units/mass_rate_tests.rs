// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the mass rate unit family.

use qubit_measure::assert_unit_family_valid;
use qubit_measure::unit;

use crate::measure::support::DefinitionCase;
use crate::measure::support::assert_definition_cases;
use crate::measure::support::assert_unit_contract;

#[test]
fn test_mass_rate_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::MassRate::MilligramPerSecond,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MassRate::GramPerSecond,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MassRate::KilogramPerSecond,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MassRate::KilogramPerHour,
            numerator: "1",
            denominator: "3600",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MassRate::TonnePerHour,
            numerator: "5",
            denominator: "18",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MassRate::PoundPerHour,
            numerator: "45359237",
            denominator: "360000000000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_mass_rate_unit_contract() {
    assert_unit_family_valid::<unit::MassRate>();
    assert_unit_contract::<unit::MassRate>();
}
