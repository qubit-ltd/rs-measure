// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the pressure unit family.

use qubit_measure::assert_unit_family_valid;
use qubit_measure::unit;

use crate::measure::support::DefinitionCase;
use crate::measure::support::assert_definition_cases;
use crate::measure::support::assert_unit_contract;

#[test]
fn test_pressure_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Pressure::Nanopascal,
            numerator: "1",
            denominator: "1000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Pressure::Micropascal,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Pressure::Millipascal,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Pressure::Pascal,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Pressure::Hectopascal,
            numerator: "100",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Pressure::Kilopascal,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Pressure::Megapascal,
            numerator: "1000000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Pressure::Bar,
            numerator: "100000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Pressure::Millibar,
            numerator: "100",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Pressure::Atmosphere,
            numerator: "101325",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Pressure::MillimeterOfMercury,
            numerator: "20265",
            denominator: "152",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Pressure::Psi,
            numerator: "8896443230521",
            denominator: "1290320000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_pressure_unit_contract() {
    assert_unit_family_valid::<unit::Pressure>();
    assert_unit_contract::<unit::Pressure>();
}
