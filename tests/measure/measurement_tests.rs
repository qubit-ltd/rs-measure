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
    AreaMeasurement,
    AreaUnit,
    LengthMeasurement,
    LengthUnit,
    MassMeasurement,
    MassUnit,
    Measurement,
    MeasurementError,
    TimeMeasurement,
    TimeUnit,
    VolumeMeasurement,
    VolumeUnit,
};
use rust_decimal::Decimal;
use serde_json::json;
use std::str::FromStr;
use uom::si::area::square_meter;
use uom::si::f64::{
    Area,
    Length,
    Mass,
    Time,
    Volume,
};
use uom::si::length::meter;
use uom::si::mass::kilogram;
use uom::si::time::second;
use uom::si::volume::liter;

#[test]
fn test_length_measurement_serde_preserves_value_and_unit() {
    let measurement = LengthMeasurement::new(Decimal::new(500, 1), LengthUnit::Centimeter);

    let value = serde_json::to_value(measurement).expect("measurement should serialize");

    assert_eq!(value, json!({ "value": "50.0", "unit": "cm" }));
}

#[test]
fn test_length_measurement_serde_deserializes_value_and_unit() {
    let measurement: LengthMeasurement = serde_json::from_value(json!({
        "value": "50.0",
        "unit": "cm",
    }))
    .expect("measurement should deserialize");

    assert_eq!(
        measurement,
        LengthMeasurement::new(Decimal::new(500, 1), LengthUnit::Centimeter)
    );
}

#[test]
fn test_mass_measurement_convert_to_uses_uom_conversion() {
    let measurement = MassMeasurement::new(Decimal::new(1, 1), MassUnit::Gram);

    let converted = measurement
        .convert_to(MassUnit::Kilogram)
        .expect("gram should convert to kilogram");

    assert_eq!(converted, MassMeasurement::new(Decimal::new(1, 4), MassUnit::Kilogram));
}

#[test]
fn test_measurement_convert_to_keeps_same_unit() {
    let measurement = LengthMeasurement::new(Decimal::new(125, 2), LengthUnit::Meter);

    let converted = measurement
        .convert_to(LengthUnit::Meter)
        .expect("same unit conversion should be a no-op");

    assert_eq!(converted, measurement);
}

#[test]
fn test_measurement_convert_to_converts_length_area_volume_and_time_units() {
    let millimeters = LengthMeasurement::new(Decimal::new(1000, 0), LengthUnit::Millimeter);
    let square_centimeters = AreaMeasurement::new(Decimal::new(10000, 0), AreaUnit::SquareCentimeter);
    let milliliters = VolumeMeasurement::new(Decimal::new(1000, 0), VolumeUnit::Milliliter);
    let minutes = TimeMeasurement::new(Decimal::new(2, 0), TimeUnit::Minute);

    assert_eq!(
        millimeters
            .convert_to(LengthUnit::Meter)
            .expect("millimeter should convert to meter"),
        LengthMeasurement::new(Decimal::ONE, LengthUnit::Meter),
    );
    assert_eq!(
        square_centimeters
            .convert_to(AreaUnit::SquareMeter)
            .expect("square centimeter should convert to square meter"),
        AreaMeasurement::new(Decimal::ONE, AreaUnit::SquareMeter),
    );
    assert_eq!(
        milliliters
            .convert_to(VolumeUnit::Liter)
            .expect("milliliter should convert to liter"),
        VolumeMeasurement::new(Decimal::ONE, VolumeUnit::Liter),
    );
    assert_eq!(
        minutes
            .convert_to(TimeUnit::Second)
            .expect("minute should convert to second"),
        TimeMeasurement::new(Decimal::new(120, 0), TimeUnit::Second),
    );
}

#[test]
fn test_measurement_from_str_parses_compact_value_and_typed_unit() {
    let measurement = LengthMeasurement::from_str("50.0cm").expect("compact measurement should parse");

    assert_eq!(
        measurement,
        LengthMeasurement::new(Decimal::new(500, 1), LengthUnit::Centimeter)
    );
}

#[test]
fn test_measurement_from_str_parses_spaced_value_and_typed_unit() {
    let measurement = MassMeasurement::from_str("  1.25 kg  ").expect("spaced measurement should parse");

    assert_eq!(
        measurement,
        MassMeasurement::new(Decimal::new(125, 2), MassUnit::Kilogram)
    );
}

#[test]
fn test_measurement_from_str_parses_signed_and_fractional_values() {
    assert_eq!(
        MassMeasurement::from_str("-1.25kg").expect("negative measurement should parse"),
        MassMeasurement::new(Decimal::new(-125, 2), MassUnit::Kilogram),
    );
    assert_eq!(
        LengthMeasurement::from_str("+3cm").expect("positive measurement should parse"),
        LengthMeasurement::new(Decimal::new(3, 0), LengthUnit::Centimeter),
    );
    assert_eq!(
        LengthMeasurement::from_str(".5m").expect("fractional measurement should parse"),
        LengthMeasurement::new(Decimal::new(5, 1), LengthUnit::Meter),
    );
}

#[test]
fn test_typed_measurement_from_str_rejects_unit_from_other_quantity() {
    let error = LengthMeasurement::from_str("12 kg").expect_err("mass unit should not parse as length");

    assert_eq!(
        error,
        MeasurementError::UnknownUnit {
            quantity: "length".to_owned(),
            unit: "kg".to_owned(),
        },
    );
}

#[test]
fn test_measurement_from_str_rejects_missing_unit() {
    let error = LengthMeasurement::from_str("12").expect_err("missing unit should fail");

    assert_eq!(error, MeasurementError::InvalidMeasurement("12".to_owned()));
}

#[test]
fn test_measurement_from_str_rejects_missing_value() {
    let error = LengthMeasurement::from_str("cm").expect_err("missing value should fail");

    assert_eq!(error, MeasurementError::InvalidMeasurement("cm".to_owned()));
}

#[test]
fn test_measurement_from_str_rejects_invalid_value() {
    let error = LengthMeasurement::from_str("1..2cm").expect_err("invalid decimal should fail");

    assert_eq!(error, MeasurementError::InvalidMeasurement("1..2cm".to_owned()));
}

#[test]
fn test_measurement_display_formats_value_and_unit() {
    let measurement = LengthMeasurement::new(Decimal::new(500, 1), LengthUnit::Centimeter);

    assert_eq!(measurement.to_string(), "50.0 cm");
}

#[test]
fn test_length_measurement_to_uom_converts_unit() {
    let measurement = LengthMeasurement::new(Decimal::new(50, 0), LengthUnit::Centimeter);
    let millimeters = LengthMeasurement::new(Decimal::new(500, 0), LengthUnit::Millimeter);
    let meters = LengthMeasurement::new(Decimal::new(2, 0), LengthUnit::Meter);

    let length = measurement.to_uom().expect("centimeter should become uom length");

    assert_eq!(length.get::<meter>(), 0.5);
    assert_eq!(
        millimeters
            .to_uom()
            .expect("millimeter should become uom length")
            .get::<meter>(),
        0.5,
    );
    assert_eq!(
        meters.to_uom().expect("meter should become uom length").get::<meter>(),
        2.0,
    );
}

#[test]
fn test_mass_measurement_to_uom_converts_unit() {
    let measurement = MassMeasurement::new(Decimal::new(1, 0), MassUnit::Tonne);
    let grams = MassMeasurement::new(Decimal::new(500, 0), MassUnit::Gram);
    let kilograms = MassMeasurement::new(Decimal::new(2, 0), MassUnit::Kilogram);

    let mass = measurement.to_uom().expect("tonne should become uom mass");

    assert_eq!(mass.get::<kilogram>(), 1000.0);
    assert_eq!(
        grams.to_uom().expect("gram should become uom mass").get::<kilogram>(),
        0.5,
    );
    assert_eq!(
        kilograms
            .to_uom()
            .expect("kilogram should become uom mass")
            .get::<kilogram>(),
        2.0,
    );
}

#[test]
fn test_time_measurement_to_uom_converts_unit() {
    let measurement = TimeMeasurement::new(Decimal::new(2, 0), TimeUnit::Minute);

    let time = measurement.to_uom().expect("minute should become uom time");

    assert_eq!(time.get::<second>(), 120.0);
}

#[test]
fn test_area_and_volume_measurements_to_uom_convert_units() {
    let area = AreaMeasurement::new(Decimal::new(10000, 0), AreaUnit::SquareCentimeter);
    let volume = VolumeMeasurement::new(Decimal::new(1, 0), VolumeUnit::Liter);

    assert_eq!(area.to_uom().expect("area should convert").get::<square_meter>(), 1.0);
    assert_eq!(volume.to_uom().expect("volume should convert").get::<liter>(), 1.0);
}

#[test]
fn test_length_measurement_from_uom_uses_target_unit() {
    let length = Length::new::<meter>(0.5);

    let measurement = LengthMeasurement::from_uom(length, LengthUnit::Centimeter)
        .expect("uom length should convert to centimeter measurement");

    assert_eq!(
        measurement,
        LengthMeasurement::new(Decimal::new(50, 0), LengthUnit::Centimeter)
    );
    assert_eq!(
        LengthMeasurement::from_uom(length, LengthUnit::Millimeter)
            .expect("uom length should convert to millimeter measurement"),
        LengthMeasurement::new(Decimal::new(500, 0), LengthUnit::Millimeter),
    );
}

#[test]
fn test_mass_measurement_from_uom_uses_target_unit() {
    let mass = Mass::new::<kilogram>(1.0);

    assert_eq!(
        MassMeasurement::from_uom(mass, MassUnit::Gram).expect("uom mass should convert to gram measurement"),
        MassMeasurement::new(Decimal::new(1000, 0), MassUnit::Gram),
    );
    assert_eq!(
        MassMeasurement::from_uom(mass, MassUnit::Kilogram).expect("uom mass should convert to kilogram measurement"),
        MassMeasurement::new(Decimal::ONE, MassUnit::Kilogram),
    );
    assert_eq!(
        MassMeasurement::from_uom(mass, MassUnit::Tonne).expect("uom mass should convert to tonne measurement"),
        MassMeasurement::new(Decimal::new(1, 3), MassUnit::Tonne),
    );
}

#[test]
fn test_time_area_and_volume_measurements_from_uom_use_target_unit() {
    let time = Time::new::<second>(120.0);
    let area = Area::new::<square_meter>(1.0);
    let volume = Volume::new::<liter>(1.0);

    assert_eq!(
        TimeMeasurement::from_uom(time, TimeUnit::Minute).expect("uom time should convert to minutes"),
        TimeMeasurement::new(Decimal::new(2, 0), TimeUnit::Minute),
    );
    assert_eq!(
        AreaMeasurement::from_uom(area, AreaUnit::SquareCentimeter)
            .expect("uom area should convert to square centimeters"),
        AreaMeasurement::new(Decimal::new(10000, 0), AreaUnit::SquareCentimeter),
    );
    assert_eq!(
        VolumeMeasurement::from_uom(volume, VolumeUnit::Milliliter).expect("uom volume should convert to milliliters"),
        VolumeMeasurement::new(Decimal::new(1000, 0), VolumeUnit::Milliliter),
    );
}

#[test]
fn test_measurement_from_uom_rejects_nan() {
    let length = Length::new::<meter>(f64::NAN);

    let error = LengthMeasurement::from_uom(length, LengthUnit::Meter).expect_err("NaN should not become Decimal");

    assert_eq!(error, MeasurementError::DecimalConversion("NaN".to_owned()));
}

#[test]
fn test_generic_measurement_type_remains_available_for_helpers() {
    fn format_measurement<U: qubit_measure::MeasurementUnit>(measurement: Measurement<U>) -> String {
        measurement.to_string()
    }

    let measurement = LengthMeasurement::new(Decimal::new(500, 1), LengthUnit::Centimeter);

    assert_eq!(format_measurement(measurement), "50.0 cm");
}
