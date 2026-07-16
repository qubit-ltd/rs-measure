// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Assertions for exact unit definitions.

use std::fmt::Debug;
use std::str::FromStr;

use qubit_measure::{
    Decimal,
    Unit,
};

use crate::measure::support::DefinitionCase;

/// Checks exact unit definitions against independently written Decimal text.
///
/// # Arguments
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
    for case in cases {
        let definition =
            case.unit.definition().expect("definition should be valid");
        assert_eq!(
            definition.factor().numerator(),
            Decimal::from_str(case.numerator)
                .expect("numerator should be valid Decimal"),
            "unexpected numerator for {:?}",
            case.unit,
        );
        assert_eq!(
            definition.factor().denominator(),
            Decimal::from_str(case.denominator)
                .expect("denominator should be valid Decimal"),
            "unexpected denominator for {:?}",
            case.unit,
        );
        assert_eq!(
            definition.offset(),
            Decimal::from_str(case.offset)
                .expect("offset should be valid Decimal"),
            "unexpected offset for {:?}",
            case.unit,
        );
    }
}
