// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the magnetic field strength unit
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
fn test_magnetic_field_strength_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::MagneticFieldStrength::AmperePerMeter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MagneticFieldStrength::AmperePerCentimeter,
            numerator: "100",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MagneticFieldStrength::Oersted,
            numerator: "7957747154594767",
            denominator: "100000000000000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_magnetic_field_strength_unit_contract() {
    assert_unit_family_valid::<unit::MagneticFieldStrength>();
    assert_unit_contract::<unit::MagneticFieldStrength>();
}
