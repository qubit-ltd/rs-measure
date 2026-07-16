// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the catalytic activity
//! concentration unit family.

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
fn test_catalytic_activity_concentration_definitions_match_exact_golden_values()
{
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::CatalyticActivityConcentration::KatalPerCubicMeter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::CatalyticActivityConcentration::EnzymeUnitPerLiter,
            numerator: "1",
            denominator: "60000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::CatalyticActivityConcentration::MilliEnzymeUnitPerMilliliter,
            numerator: "1",
            denominator: "60000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_catalytic_activity_concentration_unit_contract() {
    assert_unit_family_valid::<unit::CatalyticActivityConcentration>();
    assert_unit_contract::<unit::CatalyticActivityConcentration>();
}
