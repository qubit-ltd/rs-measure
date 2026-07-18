// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_measure::Unit;
use qubit_measure_downstream_default_fixture::{
    DefaultFixtureUnit,
    DefinitionFixtureUnit,
};

/// Verifies the exact factor and offset strings for a downstream unit.
///
/// # Parameters
///
/// * `unit` - Downstream unit whose definition is inspected.
/// * `numerator` - Expected exact factor numerator.
/// * `denominator` - Expected exact factor denominator.
/// * `offset` - Expected exact conversion offset.
fn assert_definition(
    unit: DefaultFixtureUnit,
    numerator: &str,
    denominator: &str,
    offset: &str,
) {
    let definition = unit
        .definition()
        .expect("fixture unit definition should be valid");
    assert_eq!(definition.factor().numerator().to_string(), numerator);
    assert_eq!(definition.factor().denominator().to_string(), denominator);
    assert_eq!(definition.offset().to_string(), offset);
}

#[test]
fn test_define_unit_family_preserves_decimal_literal_grammar() {
    assert_definition(DefaultFixtureUnit::Base, "1", "1", "0");
    assert_definition(DefaultFixtureUnit::Integer, "42", "1", "0");
    assert_definition(DefaultFixtureUnit::Decimal, "1.25", "1", "0");
    assert_definition(DefaultFixtureUnit::NegativeOffset, "2", "1", "-273.15");
    assert_definition(DefaultFixtureUnit::Scientific, "0.0000012345", "1", "0");
    assert_definition(DefaultFixtureUnit::DigitSeparated, "1234567", "1", "0");
    assert_definition(DefaultFixtureUnit::RadixInteger, "511", "1", "0");
    assert_definition(DefaultFixtureUnit::RadixRatio, "8", "1", "0");
}

#[test]
fn test_define_unit_family_accepts_external_const_definition_path() {
    let definition = DefinitionFixtureUnit::ExactDefinition
        .definition()
        .expect("fixture unit definition should be valid");

    assert_eq!(definition.factor().numerator().to_string(), "2");
    assert_eq!(definition.factor().denominator().to_string(), "3");
    assert_eq!(definition.offset().to_string(), "0");
}
