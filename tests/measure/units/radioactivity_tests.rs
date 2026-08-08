// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the radioactivity unit family.

use qubit_measure::assert_unit_family_valid;
use qubit_measure::unit;

use crate::measure::support::DefinitionCase;
use crate::measure::support::assert_definition_cases;
use crate::measure::support::assert_unit_contract;

#[test]
fn test_radioactivity_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Radioactivity::Becquerel,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Radioactivity::Kilobecquerel,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Radioactivity::Megabecquerel,
            numerator: "1000000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Radioactivity::Curie,
            numerator: "37000000000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Radioactivity::Millicurie,
            numerator: "37000000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Radioactivity::Microcurie,
            numerator: "37000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Radioactivity::DisintegrationsPerMinute,
            numerator: "1",
            denominator: "60",
            offset: "0",
        },
    ]);
}

#[test]
fn test_radioactivity_unit_contract() {
    assert_unit_family_valid::<unit::Radioactivity>();
    assert_unit_contract::<unit::Radioactivity>();
}
