// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the molar concentration unit
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
fn test_molar_concentration_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::MolarConcentration::NanomolePerLiter,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MolarConcentration::MicromolePerLiter,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MolarConcentration::MillimolePerLiter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MolarConcentration::MolePerLiter,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MolarConcentration::MolePerCubicMeter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MolarConcentration::ParticlePerMilliliter,
            numerator: "1",
            denominator: "602214076000000000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_molar_concentration_unit_contract() {
    assert_unit_family_valid::<unit::MolarConcentration>();
    assert_unit_contract::<unit::MolarConcentration>();
}
