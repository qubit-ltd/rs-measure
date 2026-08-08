// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the information unit family.

use qubit_measure::assert_unit_family_valid;
use qubit_measure::unit;

use crate::measure::support::DefinitionCase;
use crate::measure::support::assert_definition_cases;
use crate::measure::support::assert_unit_contract;

#[test]
fn test_information_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Information::Bit,
            numerator: "1",
            denominator: "8",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Information::Byte,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Information::Kilobyte,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Information::Megabyte,
            numerator: "1000000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Information::Gigabyte,
            numerator: "1000000000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Information::Terabyte,
            numerator: "1000000000000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Information::Kibibyte,
            numerator: "1024",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Information::Mebibyte,
            numerator: "1048576",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Information::Gibibyte,
            numerator: "1073741824",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Information::Tebibyte,
            numerator: "1099511627776",
            denominator: "1",
            offset: "0",
        },
    ]);
}

#[test]
fn test_information_unit_contract() {
    assert_unit_family_valid::<unit::Information>();
    assert_unit_contract::<unit::Information>();
}
