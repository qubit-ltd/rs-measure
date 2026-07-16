// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Shared parsing and deserialization behavior for generated unit families.

use std::str::FromStr;

use qubit_measure::{
    MeasurementError,
    unit,
};
use serde_json::json;

#[test]
fn test_unit_from_str_accepts_ascii_micro_aliases() {
    assert_eq!(
        unit::Length::from_str("um").expect("ASCII micrometer should parse"),
        unit::Length::Micrometer
    );
    assert_eq!(
        unit::Mass::from_str("ug").expect("ASCII microgram should parse"),
        unit::Mass::Microgram
    );
    assert_eq!(
        unit::Time::from_str("us").expect("ASCII microsecond should parse"),
        unit::Time::Microsecond
    );
    assert_eq!(
        unit::Volume::from_str("uL").expect("ASCII microliter should parse"),
        unit::Volume::Microliter
    );
    assert_eq!(
        unit::Pressure::from_str("uPa")
            .expect("ASCII micropascal should parse"),
        unit::Pressure::Micropascal
    );
    assert_eq!(
        unit::Power::from_str("uW").expect("ASCII microwatt should parse"),
        unit::Power::Microwatt
    );
    assert_eq!(
        unit::Velocity::from_str("um/s")
            .expect("ASCII micrometer per second should parse"),
        unit::Velocity::MicrometerPerSecond
    );
    assert_eq!(
        unit::ElectricCurrent::from_str("uA")
            .expect("ASCII microampere should parse"),
        unit::ElectricCurrent::Microampere
    );
    assert_eq!(
        unit::Capacitance::from_str("uF")
            .expect("ASCII microfarad should parse"),
        unit::Capacitance::Microfarad
    );
    assert_eq!(
        unit::Inductance::from_str("uH")
            .expect("ASCII microhenry should parse"),
        unit::Inductance::Microhenry
    );
    assert_eq!(
        unit::MagneticFluxDensity::from_str("uT")
            .expect("ASCII microtesla should parse"),
        unit::MagneticFluxDensity::Microtesla
    );
}

#[test]
fn test_unit_from_str_accepts_common_input_aliases() {
    assert_eq!(
        unit::Area::from_str("m2").expect("ASCII square meter should parse"),
        unit::Area::SquareMeter
    );
    assert_eq!(
        unit::Area::from_str("ft^2").expect("ASCII square foot should parse"),
        unit::Area::SquareFoot
    );
    assert_eq!(
        unit::Volume::from_str("m3").expect("ASCII cubic meter should parse"),
        unit::Volume::CubicMeter
    );
    assert_eq!(
        unit::Volume::from_str("in^3").expect("ASCII cubic inch should parse"),
        unit::Volume::CubicInch
    );
    assert_eq!(
        unit::MassDensity::from_str("kg/m3")
            .expect("ASCII kilogram per cubic meter should parse"),
        unit::MassDensity::KilogramPerCubicMeter
    );
    assert_eq!(
        unit::MassDensity::from_str("g/cm^3")
            .expect("ASCII gram per cubic centimeter should parse"),
        unit::MassDensity::GramPerCubicCentimeter
    );
    assert_eq!(
        unit::Pressure::from_str("mmHg")
            .expect("millimeter mercury alias should parse"),
        unit::Pressure::MillimeterOfMercury
    );
    assert_eq!(
        unit::Velocity::from_str("mph")
            .expect("mile per hour alias should parse"),
        unit::Velocity::MilePerHour
    );
    assert_eq!(
        unit::Velocity::from_str("kph")
            .expect("kilometer per hour alias should parse"),
        unit::Velocity::KilometerPerHour
    );
    assert_eq!(
        unit::Time::from_str("year").expect("year alias should parse"),
        unit::Time::CommonYear365
    );
    assert_eq!(
        unit::Time::from_str("yr").expect("year abbreviation should parse"),
        unit::Time::CommonYear365
    );
    assert_eq!(
        unit::ElectricPotential::from_str("volt")
            .expect("voltage name should parse"),
        unit::ElectricPotential::Volt
    );
    assert_eq!(
        unit::ElectricCharge::from_str("mAh")
            .expect("battery charge alias should parse"),
        unit::ElectricCharge::MilliampereHour
    );
    assert_eq!(
        unit::ElectricalResistance::from_str("kOhm")
            .expect("ASCII kiloohm should parse"),
        unit::ElectricalResistance::Kiloohm
    );
    assert_eq!(
        unit::Acceleration::from_str("m/s2")
            .expect("ASCII acceleration should parse"),
        unit::Acceleration::MeterPerSecondSquared
    );
    assert_eq!(
        unit::Torque::from_str("Nm")
            .expect("compact newton meter should parse"),
        unit::Torque::NewtonMeter
    );
    assert_eq!(
        unit::Angle::from_str("deg").expect("degree alias should parse"),
        unit::Angle::Degree
    );
    assert_eq!(
        unit::VolumeRate::from_str("m3/h")
            .expect("ASCII cubic meter per hour should parse"),
        unit::VolumeRate::CubicMeterPerHour
    );
    assert_eq!(
        unit::MolarConcentration::from_str("M")
            .expect("molar concentration alias should parse"),
        unit::MolarConcentration::MolePerLiter
    );
    assert_eq!(
        unit::ElectricField::from_str("V/um")
            .expect("ASCII electric field alias should parse"),
        unit::ElectricField::VoltPerMicrometer
    );
    assert_eq!(
        unit::Luminance::from_str("cd/m2")
            .expect("ASCII luminance alias should parse"),
        unit::Luminance::CandelaPerSquareMeter
    );
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
    assert!(serde_json::from_value::<unit::Length>(json!(123)).is_err());
}
