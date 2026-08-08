// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Shared parsing and deserialization behavior for generated unit families.

use std::fmt::Write;
use std::str::FromStr;

use qubit_measure::Measurement;
use qubit_measure::MeasurementError;
use qubit_measure::Unit;
use qubit_measure::unit;
use rust_decimal::Decimal;
use serde_json::from_value;
use serde_json::json;
use serde_json::to_string;

/// Appends one built-in unit family's stable persistence contract.
///
/// # Parameters
///
/// * `contract` - Text buffer receiving quantity, symbol, alias, and wire data.
fn append_unit_persistence_contract<U>(contract: &mut String)
where
    U: Unit,
{
    writeln!(
        contract,
        "quantity {}",
        to_string(U::QUANTITY).expect("quantity should serialize as JSON text"),
    )
    .expect("writing to a String should succeed");
    for unit in U::all() {
        writeln!(
            contract,
            "unit {} aliases {}",
            to_string(unit.symbol())
                .expect("unit symbol should serialize as JSON text"),
            to_string(unit.aliases())
                .expect("unit aliases should serialize as JSON text"),
        )
        .expect("writing to a String should succeed");
    }
    let first_unit = *U::all()
        .first()
        .expect("built-in unit family should not be empty");
    let measurement = Measurement::<U>::new(Decimal::ONE, first_unit);
    writeln!(
        contract,
        "wire {}",
        to_string(&measurement)
            .expect("measurement should serialize as JSON text"),
    )
    .expect("writing to a String should succeed");
}

/// Appends the persistence contract for each listed built-in unit family.
macro_rules! append_builtin_unit_contracts {
    ($contract:expr, $($unit:ty),+ $(,)?) => {
        $(append_unit_persistence_contract::<$unit>($contract);)+
    };
}

#[test]
fn test_builtin_unit_persistence_contract_matches_golden_manifest() {
    let mut actual = String::new();
    append_builtin_unit_contracts!(
        &mut actual,
        unit::Acceleration,
        unit::AmountOfSubstance,
        unit::Angle,
        unit::AngularVelocity,
        unit::Area,
        unit::Capacitance,
        unit::CatalyticActivity,
        unit::CatalyticActivityConcentration,
        unit::DynamicViscosity,
        unit::ElectricCharge,
        unit::ElectricCurrent,
        unit::ElectricCurrentDensity,
        unit::ElectricField,
        unit::ElectricPotential,
        unit::ElectricalConductance,
        unit::ElectricalConductivity,
        unit::ElectricalResistance,
        unit::ElectricalResistivity,
        unit::Energy,
        unit::Force,
        unit::Frequency,
        unit::HeatCapacity,
        unit::HeatFluxDensity,
        unit::Illuminance,
        unit::Inductance,
        unit::Information,
        unit::KinematicViscosity,
        unit::Length,
        unit::Luminance,
        unit::LuminousIntensity,
        unit::MagneticFieldStrength,
        unit::MagneticFlux,
        unit::MagneticFluxDensity,
        unit::Mass,
        unit::MassConcentration,
        unit::MassDensity,
        unit::MassRate,
        unit::Molality,
        unit::MolarConcentration,
        unit::MolarMass,
        unit::MolarVolume,
        unit::Power,
        unit::Pressure,
        unit::Radioactivity,
        unit::SolidAngle,
        unit::SpecificHeatCapacity,
        unit::SpecificRadioactivity,
        unit::SurfaceTension,
        unit::Temperature,
        unit::TemperatureInterval,
        unit::ThermalConductivity,
        unit::ThermalResistance,
        unit::Time,
        unit::Torque,
        unit::Velocity,
        unit::Volume,
        unit::VolumeRate,
    );

    assert_eq!(
        actual,
        include_str!("fixtures/unit_persistence_contract.txt"),
    );
}

#[test]
fn test_unit_parse_lenient_accepts_ascii_micro_aliases() {
    assert_eq!(
        unit::Length::parse_lenient("um")
            .expect("ASCII micrometer should parse"),
        unit::Length::Micrometer
    );
    assert_eq!(
        unit::Mass::parse_lenient("ug").expect("ASCII microgram should parse"),
        unit::Mass::Microgram
    );
    assert_eq!(
        unit::Time::parse_lenient("us")
            .expect("ASCII microsecond should parse"),
        unit::Time::Microsecond
    );
    assert_eq!(
        unit::Volume::parse_lenient("uL")
            .expect("ASCII microliter should parse"),
        unit::Volume::Microliter
    );
    assert_eq!(
        unit::Pressure::parse_lenient("uPa")
            .expect("ASCII micropascal should parse"),
        unit::Pressure::Micropascal
    );
    assert_eq!(
        unit::Power::parse_lenient("uW").expect("ASCII microwatt should parse"),
        unit::Power::Microwatt
    );
    assert_eq!(
        unit::Velocity::parse_lenient("um/s")
            .expect("ASCII micrometer per second should parse"),
        unit::Velocity::MicrometerPerSecond
    );
    assert_eq!(
        unit::ElectricCurrent::parse_lenient("uA")
            .expect("ASCII microampere should parse"),
        unit::ElectricCurrent::Microampere
    );
    assert_eq!(
        unit::Capacitance::parse_lenient("uF")
            .expect("ASCII microfarad should parse"),
        unit::Capacitance::Microfarad
    );
    assert_eq!(
        unit::Inductance::parse_lenient("uH")
            .expect("ASCII microhenry should parse"),
        unit::Inductance::Microhenry
    );
    assert_eq!(
        unit::MagneticFluxDensity::parse_lenient("uT")
            .expect("ASCII microtesla should parse"),
        unit::MagneticFluxDensity::Microtesla
    );
}

#[test]
fn test_unit_parse_lenient_accepts_common_input_aliases() {
    assert_eq!(
        unit::Area::parse_lenient("m2")
            .expect("ASCII square meter should parse"),
        unit::Area::SquareMeter
    );
    assert_eq!(
        unit::Area::parse_lenient("ft^2")
            .expect("ASCII square foot should parse"),
        unit::Area::SquareFoot
    );
    assert_eq!(
        unit::Volume::parse_lenient("m3")
            .expect("ASCII cubic meter should parse"),
        unit::Volume::CubicMeter
    );
    assert_eq!(
        unit::Volume::parse_lenient("in^3")
            .expect("ASCII cubic inch should parse"),
        unit::Volume::CubicInch
    );
    assert_eq!(
        unit::MassDensity::parse_lenient("kg/m3")
            .expect("ASCII kilogram per cubic meter should parse"),
        unit::MassDensity::KilogramPerCubicMeter
    );
    assert_eq!(
        unit::MassDensity::parse_lenient("g/cm^3")
            .expect("ASCII gram per cubic centimeter should parse"),
        unit::MassDensity::GramPerCubicCentimeter
    );
    assert_eq!(
        unit::Pressure::parse_lenient("mmHg")
            .expect("millimeter mercury alias should parse"),
        unit::Pressure::MillimeterOfMercury
    );
    assert_eq!(
        unit::Velocity::parse_lenient("mph")
            .expect("mile per hour alias should parse"),
        unit::Velocity::MilePerHour
    );
    assert_eq!(
        unit::Velocity::parse_lenient("kph")
            .expect("kilometer per hour alias should parse"),
        unit::Velocity::KilometerPerHour
    );
    assert_eq!(
        unit::Time::parse_lenient("year").expect("year alias should parse"),
        unit::Time::CommonYear365
    );
    assert_eq!(
        unit::Time::parse_lenient("yr")
            .expect("year abbreviation should parse"),
        unit::Time::CommonYear365
    );
    assert_eq!(
        unit::ElectricPotential::parse_lenient("volt")
            .expect("voltage name should parse"),
        unit::ElectricPotential::Volt
    );
    assert_eq!(
        unit::ElectricCharge::parse_lenient("mAh")
            .expect("battery charge alias should parse"),
        unit::ElectricCharge::MilliampereHour
    );
    assert_eq!(
        unit::ElectricalResistance::parse_lenient("kOhm")
            .expect("ASCII kiloohm should parse"),
        unit::ElectricalResistance::Kiloohm
    );
    assert_eq!(
        unit::Acceleration::parse_lenient("m/s2")
            .expect("ASCII acceleration should parse"),
        unit::Acceleration::MeterPerSecondSquared
    );
    assert_eq!(
        unit::Torque::parse_lenient("Nm")
            .expect("compact newton meter should parse"),
        unit::Torque::NewtonMeter
    );
    assert_eq!(
        unit::Angle::parse_lenient("deg").expect("degree alias should parse"),
        unit::Angle::Degree
    );
    assert_eq!(
        unit::VolumeRate::parse_lenient("m3/h")
            .expect("ASCII cubic meter per hour should parse"),
        unit::VolumeRate::CubicMeterPerHour
    );
    assert_eq!(
        unit::MolarConcentration::parse_lenient("M")
            .expect("molar concentration alias should parse"),
        unit::MolarConcentration::MolePerLiter
    );
    assert_eq!(
        unit::ElectricField::parse_lenient("V/um")
            .expect("ASCII electric field alias should parse"),
        unit::ElectricField::VoltPerMicrometer
    );
    assert_eq!(
        unit::Luminance::parse_lenient("cd/m2")
            .expect("ASCII luminance alias should parse"),
        unit::Luminance::CandelaPerSquareMeter
    );
}

#[test]
fn test_unit_from_str_rejects_alias() {
    assert!(matches!(
        unit::Time::from_str("year"),
        Err(MeasurementError::NonCanonicalUnit { .. }),
    ));
}

#[test]
fn test_unit_deserialize_rejects_alias() {
    let error = from_value::<unit::Time>(json!("year"))
        .expect_err("default Serde must reject aliases");

    assert!(error.to_string().contains("non-canonical"));
}

#[test]
fn test_unit_from_str_rejects_unknown_symbol_with_quantity_context() {
    let error = unit::Length::from_str("kg")
        .expect_err("wrong quantity unit should fail");

    assert_eq!(
        error,
        MeasurementError::UnknownUnit {
            quantity: "length".to_owned(),
            unit: "kg".to_owned(),
        },
    );
}

#[test]
fn test_unit_deserialize_rejects_non_string() {
    assert!(from_value::<unit::Length>(json!(123)).is_err());
}
