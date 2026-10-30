// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact definitions and contract tests for the specific heat capacity unit
//! family.

use num_bigint_04::BigInt;
use num_rational::BigRational;
use qubit_measure::Measurement;
use qubit_measure::Unit;
use qubit_measure::assert_unit_family_valid;
use qubit_measure::unit;
use rust_decimal::Decimal;
use rust_decimal::dec;

use crate::measure::support::DefinitionCase;
use crate::measure::support::assert_definition_cases;
use crate::measure::support::assert_unit_contract;
use crate::measure::support::decimal_as_rational;

#[test]
fn test_specific_heat_capacity_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::SpecificHeatCapacity::JoulePerKilogramKelvin,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::SpecificHeatCapacity::KilojoulePerKilogramKelvin,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::SpecificHeatCapacity::JoulePerGramDegreeCelsius,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::SpecificHeatCapacity::ThermochemicalCaloriePerGramKelvin,
            numerator: "4184",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::SpecificHeatCapacity::BritishThermalUnitInternationalTablePerPoundDegreeFahrenheit,
            numerator: "20934",
            denominator: "5",
            offset: "0",
        },
    ]);
}

#[test]
fn test_btu_it_per_pound_degree_fahrenheit_converts_to_independent_joule_value() {
    let joules_per_kilogram_kelvin = Measurement::new(
        Decimal::ONE,
        unit::SpecificHeatCapacity::BritishThermalUnitInternationalTablePerPoundDegreeFahrenheit,
    )
    .convert_to(unit::SpecificHeatCapacity::JoulePerKilogramKelvin)
    .expect("Btu (IT) per pound degree Fahrenheit should convert to joules per kilogram kelvin");

    assert_eq!(joules_per_kilogram_kelvin.value, dec!(4186.8));
}

#[test]
fn test_btu_it_specific_heat_definition_matches_independent_derived_ratio() {
    let specific_heat = unit::SpecificHeatCapacity::BritishThermalUnitInternationalTablePerPoundDegreeFahrenheit
        .definition()
        .expect("specific heat definition should be valid")
        .factor();
    let btu_energy = unit::Energy::BritishThermalUnitInternationalTable
        .definition()
        .expect("energy definition should be valid")
        .factor();
    let specific_heat_ratio =
        decimal_as_rational(specific_heat.numerator()) / decimal_as_rational(specific_heat.denominator());
    let btu_energy_ratio = decimal_as_rational(btu_energy.numerator()) / decimal_as_rational(btu_energy.denominator());
    let pounds_per_kilogram = BigRational::new(BigInt::from(100_000_000_u64), BigInt::from(45_359_237_u64));
    let fahrenheit_interval_ratio = BigRational::new(BigInt::from(9_u8), BigInt::from(5_u8));

    assert_eq!(
        specific_heat_ratio,
        btu_energy_ratio * fahrenheit_interval_ratio * pounds_per_kilogram
    );
    assert_eq!(
        specific_heat_ratio,
        BigRational::new(BigInt::from(20_934_u32), BigInt::from(5_u8))
    );
}

#[test]
fn test_specific_heat_capacity_unit_contract() {
    assert_unit_family_valid::<unit::SpecificHeatCapacity>();
    assert_unit_contract::<unit::SpecificHeatCapacity>();
}
