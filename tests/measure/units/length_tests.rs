// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the length unit family.

use qubit_measure::assert_unit_family_valid;
use qubit_measure::unit;

use crate::measure::support::DefinitionCase;
use crate::measure::support::assert_definition_cases;
use crate::measure::support::assert_unit_contract;

#[test]
fn test_length_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Length::Nanometer,
            numerator: "1",
            denominator: "1000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Length::Micrometer,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Length::Millimeter,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Length::Centimeter,
            numerator: "1",
            denominator: "100",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Length::Meter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Length::Kilometer,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Length::Inch,
            numerator: "127",
            denominator: "5000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Length::Foot,
            numerator: "381",
            denominator: "1250",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Length::Yard,
            numerator: "1143",
            denominator: "1250",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Length::Mile,
            numerator: "201168",
            denominator: "125",
            offset: "0",
        },
    ]);
}

#[test]
fn test_length_unit_contract() {
    assert_unit_family_valid::<unit::Length>();
    assert_unit_contract::<unit::Length>();
}
