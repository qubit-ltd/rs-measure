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
    AreaUnit,
    LengthUnit,
    MassUnit,
    MeasurementError,
    MeasurementUnit,
    TimeUnit,
    VolumeUnit,
};
use serde_json::json;
use std::str::FromStr;

#[test]
fn test_measurement_units_expose_quantity_and_symbol() {
    assert_eq!(LengthUnit::QUANTITY, "length");
    assert_eq!(LengthUnit::Centimeter.symbol(), "cm");
    assert_eq!(MassUnit::QUANTITY, "mass");
    assert_eq!(MassUnit::Kilogram.symbol(), "kg");
    assert_eq!(TimeUnit::QUANTITY, "time");
    assert_eq!(TimeUnit::Minute.symbol(), "min");
}

#[test]
fn test_length_unit_all_lists_supported_units() {
    let symbols: Vec<&str> = LengthUnit::all().iter().map(|unit| unit.symbol()).collect();

    assert_eq!(symbols, vec!["mm", "cm", "m", "km", "in", "ft", "yd"],);
}

#[test]
fn test_area_and_volume_unit_all_lists_supported_units() {
    let area_symbols: Vec<&str> = AreaUnit::all().iter().map(|unit| unit.symbol()).collect();
    let volume_symbols: Vec<&str> = VolumeUnit::all().iter().map(|unit| unit.symbol()).collect();

    assert_eq!(area_symbols, vec!["mm²", "cm²", "m²", "km²", "ha", "ac"]);
    assert_eq!(volume_symbols, vec!["mm³", "cm³", "m³", "mL", "L", "in³", "gal"]);
}

#[test]
fn test_measurement_unit_from_str_parses_all_symbols() {
    for unit in LengthUnit::all() {
        assert_eq!(
            LengthUnit::from_str(unit.symbol()).expect("unit symbol should parse"),
            *unit
        );
        assert_eq!(unit.to_string(), unit.symbol());
    }
    for unit in MassUnit::all() {
        assert_eq!(
            MassUnit::from_str(unit.symbol()).expect("unit symbol should parse"),
            *unit
        );
        assert_eq!(unit.to_string(), unit.symbol());
    }
    for unit in TimeUnit::all() {
        assert_eq!(
            TimeUnit::from_str(unit.symbol()).expect("unit symbol should parse"),
            *unit
        );
        assert_eq!(unit.to_string(), unit.symbol());
    }
}

#[test]
fn test_measurement_unit_from_str_rejects_unknown_symbol_with_quantity_context() {
    let error = LengthUnit::from_str("kg").expect_err("wrong quantity unit should fail");

    assert_eq!(
        error,
        MeasurementError::UnknownUnit {
            quantity: "length".to_owned(),
            unit: "kg".to_owned(),
        },
    );
}

#[test]
fn test_measurement_unit_serde_round_trips_all_symbols() {
    for unit in LengthUnit::all() {
        let value = serde_json::to_value(unit).expect("unit should serialize");

        assert_eq!(value, json!(unit.symbol()));
        assert_eq!(
            serde_json::from_value::<LengthUnit>(value).expect("unit should deserialize"),
            *unit,
        );
    }
}

#[test]
fn test_measurement_unit_deserialize_rejects_non_string() {
    assert!(serde_json::from_value::<LengthUnit>(json!(123)).is_err());
}
