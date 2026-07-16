// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the magnetic flux density unit
//! family.

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
fn test_magnetic_flux_density_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::MagneticFluxDensity::Nanotesla,
            numerator: "1",
            denominator: "1000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MagneticFluxDensity::Microtesla,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MagneticFluxDensity::Millitesla,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MagneticFluxDensity::Tesla,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MagneticFluxDensity::Gauss,
            numerator: "1",
            denominator: "10000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_magnetic_flux_density_unit_contract() {
    assert_unit_family_valid::<unit::MagneticFluxDensity>();
    assert_unit_contract::<unit::MagneticFluxDensity>();
}
