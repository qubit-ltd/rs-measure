// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the heat capacity unit family.

use qubit_measure::Measurement;
use qubit_measure::assert_unit_family_valid;
use qubit_measure::unit;
use rust_decimal::Decimal;
use rust_decimal::dec;

use crate::measure::support::DefinitionCase;
use crate::measure::support::assert_definition_cases;
use crate::measure::support::assert_unit_contract;

#[test]
fn test_heat_capacity_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::HeatCapacity::JoulePerKelvin,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::HeatCapacity::KilojoulePerKelvin,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::HeatCapacity::JoulePerDegreeCelsius,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::HeatCapacity::ThermochemicalCaloriePerKelvin,
            numerator: "523",
            denominator: "125",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::HeatCapacity::BritishThermalUnitInternationalTablePerDegreeFahrenheit,
            numerator: "474775133679",
            denominator: "250000000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_btu_it_per_degree_fahrenheit_converts_to_independent_joule_value() {
    let joules_per_kelvin = Measurement::new(
        Decimal::ONE,
        unit::HeatCapacity::BritishThermalUnitInternationalTablePerDegreeFahrenheit,
    )
    .convert_to(unit::HeatCapacity::JoulePerKelvin)
    .expect("Btu (IT) per degree Fahrenheit should convert to joules per kelvin");

    assert_eq!(joules_per_kelvin.value, dec!(1899.100534716));
}

#[test]
fn test_heat_capacity_unit_contract() {
    assert_unit_family_valid::<unit::HeatCapacity>();
    assert_unit_contract::<unit::HeatCapacity>();
}
