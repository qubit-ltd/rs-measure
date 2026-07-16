// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the volume unit family.

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
fn test_volume_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Volume::CubicMillimeter,
            numerator: "1",
            denominator: "1000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Volume::CubicCentimeter,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Volume::CubicMeter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Volume::Microliter,
            numerator: "1",
            denominator: "1000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Volume::Milliliter,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Volume::Liter,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Volume::CubicInch,
            numerator: "2048383",
            denominator: "125000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Volume::CubicFoot,
            numerator: "55306341",
            denominator: "1953125000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Volume::CubicYard,
            numerator: "1493271207",
            denominator: "1953125000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Volume::UsFluidOunce,
            numerator: "473176473",
            denominator: "16000000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Volume::UsCustomaryCup,
            numerator: "473176473",
            denominator: "2000000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Volume::UsLiquidPint,
            numerator: "473176473",
            denominator: "1000000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Volume::UsLiquidQuart,
            numerator: "473176473",
            denominator: "500000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Volume::UsLiquidGallon,
            numerator: "473176473",
            denominator: "125000000000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_volume_unit_contract() {
    assert_unit_family_valid::<unit::Volume>();
    assert_unit_contract::<unit::Volume>();
}
