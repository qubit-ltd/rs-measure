// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the force unit family.

use qubit_measure::assert_unit_family_valid;
use qubit_measure::unit;

use crate::measure::support::DefinitionCase;
use crate::measure::support::assert_definition_cases;
use crate::measure::support::assert_unit_contract;

#[test]
fn test_force_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Force::Millinewton,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Force::Newton,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Force::Kilonewton,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Force::Meganewton,
            numerator: "1000000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Force::GramForce,
            numerator: "196133",
            denominator: "20000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Force::KilogramForce,
            numerator: "196133",
            denominator: "20000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Force::PoundForce,
            numerator: "8896443230521",
            denominator: "2000000000000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_force_unit_contract() {
    assert_unit_family_valid::<unit::Force>();
    assert_unit_contract::<unit::Force>();
}
