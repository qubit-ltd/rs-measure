// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the mass unit family.

use qubit_measure::assert_unit_family_valid;
use qubit_measure::unit;

use crate::measure::support::DefinitionCase;
use crate::measure::support::assert_definition_cases;
use crate::measure::support::assert_unit_contract;

#[test]
fn test_mass_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Mass::Microgram,
            numerator: "1",
            denominator: "1000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Mass::Milligram,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Mass::Gram,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Mass::Kilogram,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Mass::Tonne,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Mass::Carat,
            numerator: "1",
            denominator: "5000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Mass::Ounce,
            numerator: "45359237",
            denominator: "1600000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Mass::Pound,
            numerator: "45359237",
            denominator: "100000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Mass::TonShort,
            numerator: "45359237",
            denominator: "50000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Mass::TonLong,
            numerator: "317514659",
            denominator: "312500",
            offset: "0",
        },
    ]);
}

#[test]
fn test_mass_unit_contract() {
    assert_unit_family_valid::<unit::Mass>();
    assert_unit_contract::<unit::Mass>();
}
