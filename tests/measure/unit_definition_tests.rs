// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Focused tests for unit-definition construction and conversion.

use qubit_measure::ConversionFactor;
use qubit_measure::UnitDefinition;
use rust_decimal::Decimal;
use rust_decimal::dec;

#[test]
fn test_unit_definition_new_preserves_factor_and_offset() {
    let factor = ConversionFactor::new(dec!(2), dec!(3))
        .expect("factor should be valid");
    let definition = UnitDefinition::new(factor, dec!(4));

    assert_eq!(definition.factor(), factor);
    assert_eq!(definition.offset(), dec!(4));
}

#[test]
fn test_unit_definition_base_uses_identity_and_zero_offset() {
    let definition = UnitDefinition::base();

    assert_eq!(definition.factor().numerator(), Decimal::ONE);
    assert_eq!(definition.factor().denominator(), Decimal::ONE);
    assert_eq!(definition.offset(), Decimal::ZERO);
}
