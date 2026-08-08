// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the mass density unit family.

use qubit_measure::assert_unit_family_valid;
use qubit_measure::unit;

use crate::measure::support::DefinitionCase;
use crate::measure::support::assert_definition_cases;
use crate::measure::support::assert_unit_contract;

#[test]
fn test_mass_density_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::MassDensity::KilogramPerCubicMeter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MassDensity::GramPerCubicMeter,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MassDensity::GramPerCubicCentimeter,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MassDensity::PoundPerCubicFoot,
            numerator: "28349523125",
            denominator: "1769802912",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MassDensity::PoundPerUsGallon,
            numerator: "736351250",
            denominator: "6145149",
            offset: "0",
        },
    ]);
}

#[test]
fn test_mass_density_unit_contract() {
    assert_unit_family_valid::<unit::MassDensity>();
    assert_unit_contract::<unit::MassDensity>();
}
