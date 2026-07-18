// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Assertions for exact unit definitions.

use qubit_measure::{
    Unit,
    unit,
};
use rust_decimal::Decimal;
use std::fmt::Debug;

use crate::measure::support::DefinitionCase;

/// Checks exact unit definitions against independently written Decimal text.
///
/// # Parameters
///
/// * `cases` - Complete ordered oracle cases for one unit family.
///
/// # Panics
///
/// Panics if the oracle is incomplete, contains invalid Decimal text, or any
/// generated definition differs from its oracle.
pub(crate) fn assert_definition_cases<U>(cases: &[DefinitionCase<U>])
where
    U: Unit + Debug,
{
    assert_eq!(cases.len(), U::all().len());
    for (expected_unit, case) in U::all().iter().zip(cases) {
        assert_eq!(
            *expected_unit, case.unit,
            "definition cases must follow Unit::all() order",
        );
        let definition =
            case.unit.definition().expect("definition should be valid");
        let normalized = qubit_measure::ConversionFactor::new(
            definition.factor().numerator(),
            definition.factor().denominator(),
        )
        .expect("built-in factor should be positive");
        assert_eq!(definition.factor(), normalized);
        assert_eq!(
            definition.factor().numerator(),
            Decimal::from_str_exact(case.numerator)
                .expect("numerator should be valid Decimal"),
            "unexpected numerator for {:?}",
            case.unit,
        );
        assert_eq!(
            definition.factor().denominator(),
            Decimal::from_str_exact(case.denominator)
                .expect("denominator should be valid Decimal"),
            "unexpected denominator for {:?}",
            case.unit,
        );
        assert_eq!(
            definition.offset(),
            Decimal::from_str_exact(case.offset)
                .expect("offset should be valid Decimal"),
            "unexpected offset for {:?}",
            case.unit,
        );
    }
}

/// Verifies that the definition oracle rejects cases in a different order
/// from `Unit::all()`.
#[test]
#[should_panic(expected = "definition cases must follow Unit::all() order")]
fn test_assert_definition_cases_rejects_out_of_order_cases() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::ElectricalConductivity::SiemensPerCentimeter,
            numerator: "100",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricalConductivity::SiemensPerMeter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
    ]);
}
