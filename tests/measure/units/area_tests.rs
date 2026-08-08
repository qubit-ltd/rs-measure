// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the area unit family.

use qubit_measure::assert_unit_family_valid;
use qubit_measure::unit;

use crate::measure::support::DefinitionCase;
use crate::measure::support::assert_definition_cases;
use crate::measure::support::assert_unit_contract;

#[test]
fn test_area_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Area::SquareMillimeter,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Area::SquareCentimeter,
            numerator: "1",
            denominator: "10000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Area::SquareMeter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Area::SquareKilometer,
            numerator: "1000000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Area::Hectare,
            numerator: "10000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Area::Acre,
            numerator: "316160658",
            denominator: "78125",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Area::SquareInch,
            numerator: "16129",
            denominator: "25000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Area::SquareFoot,
            numerator: "145161",
            denominator: "1562500",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Area::SquareYard,
            numerator: "1306449",
            denominator: "1562500",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Area::SquareMile,
            numerator: "40468564224",
            denominator: "15625",
            offset: "0",
        },
    ]);
}

#[test]
fn test_area_unit_contract() {
    assert_unit_family_valid::<unit::Area>();
    assert_unit_contract::<unit::Area>();
}
