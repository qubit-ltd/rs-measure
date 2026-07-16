// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the magnetic flux unit family.

use qubit_measure::{
    assert_unit_family_valid,
    unit,
};

use crate::measure::support::{
    DefinitionCase,
    assert_definition_cases,
    assert_unit_contract,
};

#[test]
fn test_magnetic_flux_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::MagneticFlux::Microweber,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MagneticFlux::Milliweber,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MagneticFlux::Weber,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MagneticFlux::Maxwell,
            numerator: "1",
            denominator: "100000000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_magnetic_flux_unit_contract() {
    assert_unit_family_valid::<unit::MagneticFlux>();
    assert_unit_contract::<unit::MagneticFlux>();
}
