// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the volume rate unit family.

use qubit_measure::assert_unit_family_valid;
use qubit_measure::unit;

use crate::measure::support::DefinitionCase;
use crate::measure::support::assert_definition_cases;
use crate::measure::support::assert_unit_contract;

#[test]
fn test_volume_rate_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::VolumeRate::CubicMeterPerSecond,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::VolumeRate::CubicMeterPerHour,
            numerator: "1",
            denominator: "3600",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::VolumeRate::MilliliterPerSecond,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::VolumeRate::LiterPerSecond,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::VolumeRate::LiterPerMinute,
            numerator: "1",
            denominator: "60000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::VolumeRate::UsGallonPerMinute,
            numerator: "157725491",
            denominator: "2500000000000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_volume_rate_unit_contract() {
    assert_unit_family_valid::<unit::VolumeRate>();
    assert_unit_contract::<unit::VolumeRate>();
}
