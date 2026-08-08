// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the luminous intensity unit family.

use qubit_measure::assert_unit_family_valid;
use qubit_measure::unit;

use crate::measure::support::DefinitionCase;
use crate::measure::support::assert_definition_cases;
use crate::measure::support::assert_unit_contract;

#[test]
fn test_luminous_intensity_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::LuminousIntensity::Millicandela,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::LuminousIntensity::Candela,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::LuminousIntensity::Kilocandela,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
    ]);
}

#[test]
fn test_luminous_intensity_unit_contract() {
    assert_unit_family_valid::<unit::LuminousIntensity>();
    assert_unit_contract::<unit::LuminousIntensity>();
}
