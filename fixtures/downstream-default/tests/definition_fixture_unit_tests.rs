// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for the reusable-definition fixture family.

use qubit_measure::Unit;
use qubit_measure_downstream_default_fixture::DefinitionFixtureUnit;

#[test]
fn test_define_unit_family_accepts_external_const_definition_path() {
    let definition = DefinitionFixtureUnit::ExactDefinition
        .definition()
        .expect("fixture unit definition should be valid");

    assert_eq!(definition.factor().numerator().to_string(), "2");
    assert_eq!(definition.factor().denominator().to_string(), "3");
    assert_eq!(definition.offset().to_string(), "0");
}
