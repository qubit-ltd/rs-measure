// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the time unit family.

use qubit_measure::MeasurementError;
use qubit_measure::Unit;
use qubit_measure::assert_unit_family_valid;
use qubit_measure::unit;

use crate::measure::support::DefinitionCase;
use crate::measure::support::assert_definition_cases;
use crate::measure::support::assert_unit_contract;

#[test]
fn test_time_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Time::Nanosecond,
            numerator: "1",
            denominator: "1000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Time::Microsecond,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Time::Millisecond,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Time::Second,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Time::Minute,
            numerator: "60",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Time::Hour,
            numerator: "3600",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Time::Day,
            numerator: "86400",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Time::CommonYear365,
            numerator: "31536000",
            denominator: "1",
            offset: "0",
        },
    ]);
}

#[test]
fn test_time_unit_contract() {
    assert_unit_family_valid::<unit::Time>();
    assert_unit_contract::<unit::Time>();
}

#[test]
fn test_minute_m_alias_is_lenient_but_not_canonical() {
    assert_eq!(unit::Time::parse_lenient("m"), Ok(unit::Time::Minute));
    assert!(matches!(
        "m".parse::<unit::Time>(),
        Err(MeasurementError::NonCanonicalUnit { canonical, .. })
            if canonical == "min",
    ));
    assert!(matches!(
        unit::Time::parse_strict("m"),
        Err(MeasurementError::NonCanonicalUnit { canonical, .. })
            if canonical == "min",
    ));
    assert_eq!(unit::Time::Minute.to_string(), "min");
}
