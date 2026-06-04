/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_measure::{
    MeasurementError,
    Unit,
    unit,
};
use serde_json::json;
use std::str::FromStr;

#[test]
fn test_units_expose_quantity_and_symbol() {
    assert_eq!(unit::Length::QUANTITY, "length");
    assert_eq!(unit::Length::Centimeter.symbol(), "cm");
    assert_eq!(unit::Mass::QUANTITY, "mass");
    assert_eq!(unit::Mass::Kilogram.symbol(), "kg");
    assert_eq!(unit::Time::QUANTITY, "time");
    assert_eq!(unit::Time::Minute.symbol(), "min");
    assert_eq!(unit::Pressure::QUANTITY, "pressure");
    assert_eq!(unit::Pressure::Kilopascal.symbol(), "kPa");
}

#[test]
fn test_length_unit_all_lists_supported_units() {
    let symbols: Vec<&str> = unit::Length::all().iter().map(|unit| unit.symbol()).collect();

    assert_eq!(symbols, vec!["nm", "µm", "mm", "cm", "m", "km", "in", "ft", "yd", "mi"]);
}

#[test]
fn test_area_and_volume_unit_all_lists_supported_units() {
    let area_symbols: Vec<&str> = unit::Area::all().iter().map(|unit| unit.symbol()).collect();
    let volume_symbols: Vec<&str> = unit::Volume::all().iter().map(|unit| unit.symbol()).collect();

    assert_eq!(
        area_symbols,
        vec!["mm²", "cm²", "m²", "km²", "ha", "ac", "in²", "ft²", "yd²", "mi²"],
    );
    assert_eq!(
        volume_symbols,
        vec![
            "mm³", "cm³", "m³", "µL", "mL", "L", "in³", "ft³", "yd³", "fl oz", "cup", "liq pt", "liq qt", "gal",
        ],
    );
}

#[test]
fn test_mass_and_time_unit_all_lists_supported_units() {
    let mass_symbols: Vec<&str> = unit::Mass::all().iter().map(|unit| unit.symbol()).collect();
    let time_symbols: Vec<&str> = unit::Time::all().iter().map(|unit| unit.symbol()).collect();

    assert_eq!(
        mass_symbols,
        vec!["µg", "mg", "g", "kg", "t", "ct", "oz", "lb", "2000 lb", "2240 lb"],
    );
    assert_eq!(time_symbols, vec!["ns", "µs", "ms", "s", "min", "h", "d", "a"]);
}

#[test]
fn test_production_quantity_family_units_are_available() {
    let pressure_symbols: Vec<&str> = unit::Pressure::all().iter().map(|unit| unit.symbol()).collect();
    let energy_symbols: Vec<&str> = unit::Energy::all().iter().map(|unit| unit.symbol()).collect();
    let power_symbols: Vec<&str> = unit::Power::all().iter().map(|unit| unit.symbol()).collect();
    let velocity_symbols: Vec<&str> = unit::Velocity::all().iter().map(|unit| unit.symbol()).collect();
    let frequency_symbols: Vec<&str> = unit::Frequency::all().iter().map(|unit| unit.symbol()).collect();
    let density_symbols: Vec<&str> = unit::MassDensity::all().iter().map(|unit| unit.symbol()).collect();
    let temperature_symbols: Vec<&str> = unit::Temperature::all().iter().map(|unit| unit.symbol()).collect();
    let interval_symbols: Vec<&str> = unit::TemperatureInterval::all()
        .iter()
        .map(|unit| unit.symbol())
        .collect();

    assert_eq!(
        pressure_symbols,
        vec![
            "nPa", "µPa", "mPa", "Pa", "hPa", "kPa", "MPa", "bar", "mbar", "atm", "mm Hg", "psi"
        ],
    );
    assert_eq!(
        energy_symbols,
        vec!["J", "kJ", "MJ", "W · h", "kW · h", "eV", "cal", "kcal", "Btu"]
    );
    assert_eq!(power_symbols, vec!["nW", "µW", "mW", "W", "kW", "MW", "hp"]);
    assert_eq!(
        velocity_symbols,
        vec!["µm/s", "mm/s", "cm/s", "m/s", "km/h", "ft/s", "mi/h", "kn"]
    );
    assert_eq!(frequency_symbols, vec!["Hz", "kHz", "MHz", "GHz"]);
    assert_eq!(density_symbols, vec!["kg/m³", "g/m³", "g/cm³", "lb/ft³", "lb/gal"],);
    assert_eq!(temperature_symbols, vec!["K", "°C", "°F", "°R"]);
    assert_eq!(interval_symbols, vec!["K", "°C", "°F", "°R"]);
}

#[test]
fn test_unit_from_str_parses_all_symbols() {
    for unit in unit::Length::all() {
        assert_eq!(
            unit::Length::from_str(unit.symbol()).expect("unit symbol should parse"),
            *unit,
        );
        assert_eq!(unit.to_string(), unit.symbol());
    }
    for unit in unit::Mass::all() {
        assert_eq!(
            unit::Mass::from_str(unit.symbol()).expect("unit symbol should parse"),
            *unit,
        );
        assert_eq!(unit.to_string(), unit.symbol());
    }
    for unit in unit::Time::all() {
        assert_eq!(
            unit::Time::from_str(unit.symbol()).expect("unit symbol should parse"),
            *unit,
        );
        assert_eq!(unit.to_string(), unit.symbol());
    }
}

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
        unit::Pressure::from_str("uPa").expect("ASCII micropascal should parse"),
        unit::Pressure::Micropascal
    );
    assert_eq!(
        unit::Power::from_str("uW").expect("ASCII microwatt should parse"),
        unit::Power::Microwatt
    );
    assert_eq!(
        unit::Velocity::from_str("um/s").expect("ASCII micrometer per second should parse"),
        unit::Velocity::MicrometerPerSecond
    );
}

#[test]
fn test_unit_from_str_rejects_unknown_symbol_with_quantity_context() {
    let error = unit::Length::from_str("kg").expect_err("wrong quantity unit should fail");

    assert_eq!(
        error,
        MeasurementError::UnknownUnit {
            quantity: "length".to_owned(),
            unit: "kg".to_owned(),
        },
    );
}

#[test]
fn test_unit_serde_round_trips_all_symbols() {
    for unit in unit::Length::all() {
        let value = serde_json::to_value(unit).expect("unit should serialize");

        assert_eq!(value, json!(unit.symbol()));
        assert_eq!(
            serde_json::from_value::<unit::Length>(value).expect("unit should deserialize"),
            *unit,
        );
    }
}

#[test]
fn test_unit_deserialize_rejects_non_string() {
    assert!(serde_json::from_value::<unit::Length>(json!(123)).is_err());
}
