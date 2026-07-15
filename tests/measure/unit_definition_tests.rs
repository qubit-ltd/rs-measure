// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::fmt::Debug;
use std::str::FromStr;

use qubit_measure::{
    Decimal,
    Unit,
    unit,
};

struct DefinitionCase<U> {
    unit: U,
    numerator: &'static str,
    denominator: &'static str,
    offset: &'static str,
}

/// Checks exact unit definitions against independently written Decimal text.
fn assert_definition_cases<U>(cases: &[DefinitionCase<U>])
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
        );
        assert_eq!(
            definition.factor().denominator(),
            Decimal::from_str(case.denominator)
                .expect("denominator should be valid Decimal"),
        );
        assert_eq!(
            definition.offset(),
            Decimal::from_str(case.offset)
                .expect("offset should be valid Decimal"),
        );
    }
}

#[test]
fn test_length_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Length::Nanometer,
            numerator: "1",
            denominator: "1000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Length::Micrometer,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Length::Millimeter,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Length::Centimeter,
            numerator: "1",
            denominator: "100",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Length::Meter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Length::Kilometer,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Length::Inch,
            numerator: "127",
            denominator: "5000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Length::Foot,
            numerator: "381",
            denominator: "1250",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Length::Yard,
            numerator: "1143",
            denominator: "1250",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Length::Mile,
            numerator: "201168",
            denominator: "125",
            offset: "0",
        },
    ]);
}

#[test]
fn test_temperature_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Temperature::Kelvin,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Temperature::Celsius,
            numerator: "1",
            denominator: "1",
            offset: "273.15",
        },
        DefinitionCase {
            unit: unit::Temperature::Fahrenheit,
            numerator: "5",
            denominator: "9",
            offset: "459.67",
        },
        DefinitionCase {
            unit: unit::Temperature::Rankine,
            numerator: "5",
            denominator: "9",
            offset: "0",
        },
    ]);
}

#[test]
fn test_area_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Area::SquareMillimeter,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Area::SquareCentimeter,
            numerator: "1",
            denominator: "10000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Area::SquareMeter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Area::SquareKilometer,
            numerator: "1000000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Area::Hectare,
            numerator: "10000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Area::Acre,
            numerator: "316160658",
            denominator: "78125",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Area::SquareInch,
            numerator: "16129",
            denominator: "25000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Area::SquareFoot,
            numerator: "145161",
            denominator: "1562500",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Area::SquareYard,
            numerator: "1306449",
            denominator: "1562500",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Area::SquareMile,
            numerator: "40468564224",
            denominator: "15625",
            offset: "0",
        },
    ]);
}

#[test]
fn test_mass_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Mass::Microgram,
            numerator: "1",
            denominator: "1000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Mass::Milligram,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Mass::Gram,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Mass::Kilogram,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Mass::Tonne,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Mass::Carat,
            numerator: "1",
            denominator: "5000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Mass::Ounce,
            numerator: "45359237",
            denominator: "1600000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Mass::Pound,
            numerator: "45359237",
            denominator: "100000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Mass::TonShort,
            numerator: "45359237",
            denominator: "50000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Mass::TonLong,
            numerator: "317514659",
            denominator: "312500",
            offset: "0",
        },
    ]);
}

#[test]
fn test_temperature_interval_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::TemperatureInterval::Kelvin,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::TemperatureInterval::Celsius,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::TemperatureInterval::Fahrenheit,
            numerator: "5",
            denominator: "9",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::TemperatureInterval::Rankine,
            numerator: "5",
            denominator: "9",
            offset: "0",
        },
    ]);
}

#[test]
fn test_velocity_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Velocity::MicrometerPerSecond,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Velocity::MillimeterPerSecond,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Velocity::CentimeterPerSecond,
            numerator: "1",
            denominator: "100",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Velocity::MeterPerSecond,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Velocity::KilometerPerHour,
            numerator: "5",
            denominator: "18",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Velocity::FootPerSecond,
            numerator: "381",
            denominator: "1250",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Velocity::MilePerHour,
            numerator: "1397",
            denominator: "3125",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Velocity::Knot,
            numerator: "463",
            denominator: "900",
            offset: "0",
        },
    ]);
}

#[test]
fn test_acceleration_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Acceleration::MillimeterPerSecondSquared,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Acceleration::MeterPerSecondSquared,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Acceleration::FootPerSecondSquared,
            numerator: "381",
            denominator: "1250",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Acceleration::StandardGravity,
            numerator: "196133",
            denominator: "20000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_force_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Force::Millinewton,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Force::Newton,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Force::Kilonewton,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Force::Meganewton,
            numerator: "1000000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Force::GramForce,
            numerator: "196133",
            denominator: "20000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Force::KilogramForce,
            numerator: "196133",
            denominator: "20000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Force::PoundForce,
            numerator: "8896443230521",
            denominator: "2000000000000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_pressure_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Pressure::Nanopascal,
            numerator: "1",
            denominator: "1000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Pressure::Micropascal,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Pressure::Millipascal,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Pressure::Pascal,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Pressure::Hectopascal,
            numerator: "100",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Pressure::Kilopascal,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Pressure::Megapascal,
            numerator: "1000000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Pressure::Bar,
            numerator: "100000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Pressure::Millibar,
            numerator: "100",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Pressure::Atmosphere,
            numerator: "101325",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Pressure::MillimeterOfMercury,
            numerator: "20265",
            denominator: "152",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Pressure::Psi,
            numerator: "8896443230521",
            denominator: "1290320000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_frequency_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Frequency::Hertz,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Frequency::Kilohertz,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Frequency::Megahertz,
            numerator: "1000000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Frequency::Gigahertz,
            numerator: "1000000000",
            denominator: "1",
            offset: "0",
        },
    ]);
}

#[test]
fn test_angle_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Angle::Radian,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Angle::Degree,
            numerator: "3490658503988659",
            denominator: "200000000000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Angle::Revolution,
            numerator: "3141592653589793",
            denominator: "500000000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Angle::Minute,
            numerator: "45451282604019",
            denominator: "156250000000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Angle::Second,
            numerator: "15150427534673",
            denominator: "3125000000000000000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_angular_velocity_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::AngularVelocity::RadianPerSecond,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::AngularVelocity::DegreePerSecond,
            numerator: "3490658503988659",
            denominator: "200000000000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::AngularVelocity::RevolutionPerSecond,
            numerator: "3141592653589793",
            denominator: "500000000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::AngularVelocity::RevolutionPerMinute,
            numerator: "10471975511965977",
            denominator: "100000000000000000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_torque_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Torque::MillinewtonMeter,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Torque::NewtonMeter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Torque::KilonewtonMeter,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Torque::PoundForceFoot,
            numerator: "3389544870828501",
            denominator: "2500000000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Torque::PoundForceInch,
            numerator: "1129848290276167",
            denominator: "10000000000000000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_dynamic_viscosity_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::DynamicViscosity::MicropascalSecond,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::DynamicViscosity::MillipascalSecond,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::DynamicViscosity::PascalSecond,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::DynamicViscosity::Poise,
            numerator: "1",
            denominator: "10",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::DynamicViscosity::Centipoise,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_kinematic_viscosity_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::KinematicViscosity::SquareMillimeterPerSecond,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::KinematicViscosity::SquareMeterPerSecond,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::KinematicViscosity::Stokes,
            numerator: "1",
            denominator: "10000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::KinematicViscosity::Centistokes,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_mass_rate_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::MassRate::MilligramPerSecond,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MassRate::GramPerSecond,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MassRate::KilogramPerSecond,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MassRate::KilogramPerHour,
            numerator: "1",
            denominator: "3600",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MassRate::TonnePerHour,
            numerator: "5",
            denominator: "18",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MassRate::PoundPerHour,
            numerator: "45359237",
            denominator: "360000000000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_capacitance_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Capacitance::Picofarad,
            numerator: "1",
            denominator: "1000000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Capacitance::Nanofarad,
            numerator: "1",
            denominator: "1000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Capacitance::Microfarad,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Capacitance::Millifarad,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Capacitance::Farad,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
    ]);
}

#[test]
fn test_electric_charge_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::ElectricCharge::Microcoulomb,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricCharge::Millicoulomb,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricCharge::Coulomb,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricCharge::Kilocoulomb,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricCharge::AmpereHour,
            numerator: "3600",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricCharge::MilliampereHour,
            numerator: "18",
            denominator: "5",
            offset: "0",
        },
    ]);
}

#[test]
fn test_electric_current_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::ElectricCurrent::Picoampere,
            numerator: "1",
            denominator: "1000000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricCurrent::Nanoampere,
            numerator: "1",
            denominator: "1000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricCurrent::Microampere,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricCurrent::Milliampere,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricCurrent::Ampere,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricCurrent::Kiloampere,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricCurrent::Megaampere,
            numerator: "1000000",
            denominator: "1",
            offset: "0",
        },
    ]);
}

#[test]
fn test_electric_current_density_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::ElectricCurrentDensity::AmperePerSquareMeter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricCurrentDensity::AmperePerSquareCentimeter,
            numerator: "10000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricCurrentDensity::AmperePerSquareMillimeter,
            numerator: "1000000",
            denominator: "1",
            offset: "0",
        },
    ]);
}

#[test]
fn test_electric_field_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::ElectricField::VoltPerMeter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricField::VoltPerCentimeter,
            numerator: "100",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricField::VoltPerMillimeter,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricField::VoltPerMicrometer,
            numerator: "1000000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricField::KilovoltPerMillimeter,
            numerator: "1000000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricField::MegavoltPerMeter,
            numerator: "1000000",
            denominator: "1",
            offset: "0",
        },
    ]);
}

#[test]
fn test_electric_potential_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::ElectricPotential::Nanovolt,
            numerator: "1",
            denominator: "1000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricPotential::Microvolt,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricPotential::Millivolt,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricPotential::Volt,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricPotential::Kilovolt,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricPotential::Megavolt,
            numerator: "1000000",
            denominator: "1",
            offset: "0",
        },
    ]);
}

#[test]
fn test_electrical_conductance_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::ElectricalConductance::Microsiemens,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricalConductance::Millisiemens,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricalConductance::Siemens,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
    ]);
}

#[test]
fn test_electrical_conductivity_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::ElectricalConductivity::SiemensPerMeter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricalConductivity::SiemensPerCentimeter,
            numerator: "100",
            denominator: "1",
            offset: "0",
        },
    ]);
}

#[test]
fn test_electrical_resistance_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::ElectricalResistance::Microohm,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricalResistance::Milliohm,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricalResistance::Ohm,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricalResistance::Kiloohm,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricalResistance::Megaohm,
            numerator: "1000000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricalResistance::Gigaohm,
            numerator: "1000000000",
            denominator: "1",
            offset: "0",
        },
    ]);
}

#[test]
fn test_electrical_resistivity_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::ElectricalResistivity::MilliohmMeter,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricalResistivity::OhmMeter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricalResistivity::OhmCentimeter,
            numerator: "1",
            denominator: "100",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ElectricalResistivity::OhmSquareMillimeterPerMeter,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_inductance_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Inductance::Nanohenry,
            numerator: "1",
            denominator: "1000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Inductance::Microhenry,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Inductance::Millihenry,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Inductance::Henry,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
    ]);
}

#[test]
fn test_magnetic_field_strength_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::MagneticFieldStrength::AmperePerMeter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MagneticFieldStrength::AmperePerCentimeter,
            numerator: "100",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MagneticFieldStrength::Oersted,
            numerator: "7957747154594767",
            denominator: "100000000000000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_magnetic_flux_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::MagneticFlux::Microweber,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MagneticFlux::Milliweber,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MagneticFlux::Weber,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MagneticFlux::Maxwell,
            numerator: "1",
            denominator: "100000000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_magnetic_flux_density_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::MagneticFluxDensity::Nanotesla,
            numerator: "1",
            denominator: "1000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MagneticFluxDensity::Microtesla,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MagneticFluxDensity::Millitesla,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MagneticFluxDensity::Tesla,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MagneticFluxDensity::Gauss,
            numerator: "1",
            denominator: "10000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_amount_of_substance_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::AmountOfSubstance::Micromole,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::AmountOfSubstance::Millimole,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::AmountOfSubstance::Mole,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::AmountOfSubstance::Kilomole,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::AmountOfSubstance::Particle,
            numerator: "1",
            denominator: "602214076000000000000000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_catalytic_activity_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::CatalyticActivity::Microkatal,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::CatalyticActivity::Millikatal,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::CatalyticActivity::Katal,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::CatalyticActivity::EnzymeUnit,
            numerator: "1",
            denominator: "60000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::CatalyticActivity::MilliEnzymeUnit,
            numerator: "1",
            denominator: "60000000000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_catalytic_activity_concentration_definitions_match_exact_golden_values()
{
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::CatalyticActivityConcentration::KatalPerCubicMeter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::CatalyticActivityConcentration::EnzymeUnitPerLiter,
            numerator: "1",
            denominator: "60000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::CatalyticActivityConcentration::MilliEnzymeUnitPerMilliliter,
            numerator: "1",
            denominator: "60000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_heat_flux_density_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::HeatFluxDensity::MilliwattPerSquareMeter,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::HeatFluxDensity::WattPerSquareMeter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::HeatFluxDensity::KilowattPerSquareMeter,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::HeatFluxDensity::WattPerSquareCentimeter,
            numerator: "10000",
            denominator: "1",
            offset: "0",
        },
    ]);
}

#[test]
fn test_illuminance_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Illuminance::Lux,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Illuminance::Kilolux,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Illuminance::Footcandle,
            numerator: "1562500",
            denominator: "145161",
            offset: "0",
        },
    ]);
}

#[test]
fn test_luminance_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Luminance::CandelaPerSquareMeter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Luminance::CandelaPerSquareCentimeter,
            numerator: "10000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Luminance::CandelaPerSquareFoot,
            numerator: "1562500",
            denominator: "145161",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Luminance::Footlambert,
            numerator: "6852518199270781",
            denominator: "2000000000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Luminance::Stilb,
            numerator: "10000",
            denominator: "1",
            offset: "0",
        },
    ]);
}

#[test]
fn test_luminous_intensity_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::LuminousIntensity::Millicandela,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::LuminousIntensity::Candela,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::LuminousIntensity::Kilocandela,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
    ]);
}

#[test]
fn test_mass_concentration_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::MassConcentration::MicrogramPerLiter,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MassConcentration::MilligramPerLiter,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MassConcentration::GramPerLiter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MassConcentration::KilogramPerCubicMeter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MassConcentration::MilligramPerDeciliter,
            numerator: "1",
            denominator: "100",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MassConcentration::GramPerDeciliter,
            numerator: "10",
            denominator: "1",
            offset: "0",
        },
    ]);
}

#[test]
fn test_molality_definitions_match_exact_golden_values() {
    assert_definition_cases(&[DefinitionCase {
        unit: unit::Molality::MolePerKilogram,
        numerator: "1",
        denominator: "1",
        offset: "0",
    }]);
}

#[test]
fn test_molar_concentration_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::MolarConcentration::NanomolePerLiter,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MolarConcentration::MicromolePerLiter,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MolarConcentration::MillimolePerLiter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MolarConcentration::MolePerLiter,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MolarConcentration::MolePerCubicMeter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MolarConcentration::ParticlePerMilliliter,
            numerator: "1",
            denominator: "602214076000000000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_molar_mass_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::MolarMass::MilligramPerMole,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MolarMass::GramPerMole,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MolarMass::KilogramPerMole,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
    ]);
}

#[test]
fn test_molar_volume_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::MolarVolume::CubicCentimeterPerMole,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MolarVolume::CubicDecimeterPerMole,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MolarVolume::CubicMeterPerMole,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
    ]);
}

#[test]
fn test_radioactivity_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Radioactivity::Becquerel,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Radioactivity::Kilobecquerel,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Radioactivity::Megabecquerel,
            numerator: "1000000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Radioactivity::Curie,
            numerator: "37000000000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Radioactivity::Millicurie,
            numerator: "37000000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Radioactivity::Microcurie,
            numerator: "37000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Radioactivity::DisintegrationsPerMinute,
            numerator: "1",
            denominator: "60",
            offset: "0",
        },
    ]);
}

#[test]
fn test_solid_angle_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::SolidAngle::Steradian,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::SolidAngle::Spat,
            numerator: "12566370614359173",
            denominator: "1000000000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::SolidAngle::SquareDegree,
            numerator: "1523087098933543",
            denominator: "5000000000000000000",
            offset: "0",
        },
    ]);
}

#[test]
fn test_specific_radioactivity_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::SpecificRadioactivity::BecquerelPerKilogram,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::SpecificRadioactivity::CuriePerKilogram,
            numerator: "37000000000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit:
                unit::SpecificRadioactivity::DisintegrationsPerMinutePerKilogram,
            numerator: "1",
            denominator: "60",
            offset: "0",
        },
    ]);
}

#[test]
fn test_surface_tension_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::SurfaceTension::MillinewtonPerMeter,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::SurfaceTension::NewtonPerMeter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::SurfaceTension::DynePerCentimeter,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::SurfaceTension::JoulePerSquareMeter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
    ]);
}

#[test]
fn test_thermal_conductivity_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::ThermalConductivity::MilliwattPerMeterKelvin,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ThermalConductivity::WattPerMeterKelvin,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ThermalConductivity::KilowattPerMeterKelvin,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ThermalConductivity::WattPerMeterDegreeCelsius,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
    ]);
}

#[test]
fn test_thermal_resistance_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::ThermalResistance::KelvinPerMilliwatt,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ThermalResistance::KelvinPerWatt,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::ThermalResistance::KelvinPerKilowatt,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
    ]);
}

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
fn test_energy_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Energy::Joule,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Energy::Kilojoule,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Energy::Megajoule,
            numerator: "1000000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Energy::WattHour,
            numerator: "3600",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Energy::KilowattHour,
            numerator: "3600000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Energy::Electronvolt,
            numerator: "801088317",
            denominator: "5000000000000000000000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Energy::ThermochemicalCalorie,
            numerator: "523",
            denominator: "125",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Energy::ThermochemicalKilocalorie,
            numerator: "4184",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Energy::BritishThermalUnitInternationalTable,
            numerator: "131882",
            denominator: "125",
            offset: "0",
        },
    ]);
}

#[test]
fn test_power_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::Power::Nanowatt,
            numerator: "1",
            denominator: "1000000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Power::Microwatt,
            numerator: "1",
            denominator: "1000000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Power::Milliwatt,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Power::Watt,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Power::Kilowatt,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Power::Megawatt,
            numerator: "1000000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::Power::MechanicalHorsepower,
            numerator: "37284993579113511",
            denominator: "50000000000000",
            offset: "0",
        },
    ]);
}

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
fn test_mass_density_definitions_match_exact_golden_values() {
    assert_definition_cases(&[
        DefinitionCase {
            unit: unit::MassDensity::KilogramPerCubicMeter,
            numerator: "1",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MassDensity::GramPerCubicMeter,
            numerator: "1",
            denominator: "1000",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MassDensity::GramPerCubicCentimeter,
            numerator: "1000",
            denominator: "1",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MassDensity::PoundPerCubicFoot,
            numerator: "28349523125",
            denominator: "1769802912",
            offset: "0",
        },
        DefinitionCase {
            unit: unit::MassDensity::PoundPerUsGallon,
            numerator: "736351250",
            denominator: "6145149",
            offset: "0",
        },
    ]);
}

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
            numerator: "1186938",
            denominator: "625",
            offset: "0",
        },
    ]);
}

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
            numerator: "189910080000",
            denominator: "45359237",
            offset: "0",
        },
    ]);
}
