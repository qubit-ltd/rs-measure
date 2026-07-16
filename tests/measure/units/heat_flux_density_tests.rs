// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the heat flux density unit family.

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
fn test_heat_flux_density_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::HeatFluxDensity::MilliwattPerSquareMeter,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::HeatFluxDensity::WattPerSquareMeter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::HeatFluxDensity::KilowattPerSquareMeter,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::HeatFluxDensity::WattPerSquareCentimeter,
            numerator: "10000",
            denominator: "1",
            offset: "0",
        },
    ]);
}

#[test]
fn test_heat_flux_density_unit_contract() {
    assert_unit_family_valid::<unit::HeatFluxDensity>();
    assert_unit_contract::<unit::HeatFluxDensity>();
}
