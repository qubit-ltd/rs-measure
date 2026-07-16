// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_measure::{
    ConversionOptions,
    Measurement,
    MeasurementError,
    RoundingStrategy,
    Unit,
    measurement,
    unit,
};
use rust_decimal::Decimal;
use rust_decimal::dec;
use serde_json::json;
use std::str::FromStr;

#[test]
fn test_length_measurement_serde_preserves_value_and_unit() {
    let measurement = measurement::Length::new(
        Decimal::new(500, 1),
        unit::Length::Centimeter,
    );

    let value = serde_json::to_value(measurement)
        .expect("measurement should serialize");

    assert_eq!(
        value,
        json!({ "quantity": "length", "value": "50.0", "unit": "cm" }),
    );
}

#[test]
fn test_length_measurement_serde_deserializes_value_and_unit() {
    let measurement: measurement::Length = serde_json::from_value(json!({
        "quantity": "length",
        "value": "50.0",
        "unit": "cm",
    }))
    .expect("measurement should deserialize");

    assert_eq!(
        measurement,
        measurement::Length::new(
            Decimal::new(500, 1),
            unit::Length::Centimeter
        ),
    );
}

#[test]
fn test_length_conversion_uses_decimal_without_f64_loss() {
    let source = measurement::Length::new(
        Decimal::from_str("12345678901234567890.12345678")
            .expect("source should be valid Decimal"),
        unit::Length::Centimeter,
    );
    let options = ConversionOptions::maximum_precision(
        RoundingStrategy::MidpointNearestEven,
    );

    let converted = source
        .convert_to_with_options(unit::Length::Meter, options)
        .expect("length should convert exactly");

    assert_eq!(
        converted.value,
        Decimal::from_str("123456789012345678.9012345678")
            .expect("expected value should be valid Decimal"),
    );
}

#[test]
fn test_measurement_json_contains_and_validates_quantity() {
    let value = measurement::Length::new(dec!(50.0), unit::Length::Centimeter);
    assert_eq!(
        serde_json::to_value(value).expect("measurement should serialize"),
        json!({"quantity": "length", "value": "50.0", "unit": "cm"}),
    );

    let error = serde_json::from_value::<measurement::Length>(json!({
        "quantity": "mass",
        "value": "50.0",
        "unit": "cm",
    }))
    .expect_err("mismatched quantity should fail");
    assert!(error.to_string().contains("expected length"));
}

#[test]
fn test_measurement_explicit_scale_applies_to_same_unit() {
    let source = measurement::Length::new(dec!(12.345), unit::Length::Meter);
    let options = ConversionOptions::fixed_scale(
        2,
        RoundingStrategy::MidpointNearestEven,
    )
    .expect("scale should be valid");

    let converted = source
        .convert_to_with_options(unit::Length::Meter, options)
        .expect("same-unit conversion should apply scale");

    assert_eq!(converted.value, dec!(12.34));
    assert_eq!(converted.value.scale(), 2);
}

#[test]
fn test_measurement_alias_deserializes_leniently_and_serializes_canonically() {
    let measurement: measurement::Time = serde_json::from_value(json!({
        "quantity": "time",
        "value": "1",
        "unit": "year",
    }))
    .expect("documented alias should deserialize");

    assert_eq!(measurement.unit, unit::Time::CommonYear365);
    assert_eq!(
        serde_json::to_value(measurement)
            .expect("measurement should serialize"),
        json!({"quantity": "time", "value": "1", "unit": "a (365 d)"}),
    );
}

#[test]
fn test_measurement_parse_strict_rejects_unit_alias() {
    assert!(matches!(
        measurement::Time::parse_strict("1 year"),
        Err(MeasurementError::NonCanonicalUnit { .. }),
    ));
}

#[test]
fn test_measurement_parse_strict_accepts_canonical_input() {
    assert_eq!(
        measurement::Time::parse_strict("1 s")
            .expect("canonical measurement should parse"),
        measurement::Time::new(Decimal::ONE, unit::Time::Second),
    );
}

#[test]
fn test_measurement_parse_strict_rejects_malformed_input() {
    for input in ["1", "1e999 s"] {
        assert!(matches!(
            measurement::Time::parse_strict(input),
            Err(MeasurementError::InvalidMeasurement(_)),
        ));
    }
}

#[test]
fn test_mass_measurement_convert_to_uses_decimal_conversion() {
    let measurement =
        measurement::Mass::new(Decimal::new(1, 1), unit::Mass::Gram);

    let converted = measurement
        .convert_to_with_options(
            unit::Mass::Kilogram,
            ConversionOptions::default(),
        )
        .expect("gram should convert to kilogram");

    assert_eq!(
        converted,
        measurement::Mass::new(Decimal::new(1, 4), unit::Mass::Kilogram),
    );
}

#[test]
fn test_measurement_convert_to_keeps_same_unit() {
    let measurement =
        measurement::Length::new(Decimal::new(125, 2), unit::Length::Meter);

    let converted = measurement
        .convert_to_with_options(
            unit::Length::Meter,
            ConversionOptions::default(),
        )
        .expect("same unit conversion should be a no-op");

    assert_eq!(converted, measurement);
}

#[test]
fn test_measurement_convert_to_uses_immutable_default_options() {
    let measurement =
        measurement::Length::new(Decimal::ONE, unit::Length::Meter);

    let implicit = measurement
        .convert_to(unit::Length::Foot)
        .expect("default conversion should succeed");
    let explicit = measurement
        .convert_to_with_options(unit::Length::Foot, ConversionOptions::DEFAULT)
        .expect("explicit default conversion should succeed");

    assert_eq!(implicit, explicit);
}

#[test]
fn test_measurement_convert_to_converts_length_area_volume_and_time_units() {
    let millimeters = measurement::Length::new(
        Decimal::new(1000, 0),
        unit::Length::Millimeter,
    );
    let square_centimeters = measurement::Area::new(
        Decimal::new(10000, 0),
        unit::Area::SquareCentimeter,
    );
    let milliliters = measurement::Volume::new(
        Decimal::new(1000, 0),
        unit::Volume::Milliliter,
    );
    let minutes =
        measurement::Time::new(Decimal::new(2, 0), unit::Time::Minute);

    assert_eq!(
        millimeters
            .convert_to_with_options(
                unit::Length::Meter,
                ConversionOptions::default()
            )
            .expect("millimeter should convert to meter"),
        measurement::Length::new(Decimal::ONE, unit::Length::Meter),
    );
    assert_eq!(
        square_centimeters
            .convert_to_with_options(
                unit::Area::SquareMeter,
                ConversionOptions::default()
            )
            .expect("square centimeter should convert to square meter"),
        measurement::Area::new(Decimal::ONE, unit::Area::SquareMeter),
    );
    assert_eq!(
        milliliters
            .convert_to_with_options(
                unit::Volume::Liter,
                ConversionOptions::default()
            )
            .expect("milliliter should convert to liter"),
        measurement::Volume::new(Decimal::ONE, unit::Volume::Liter),
    );
    assert_eq!(
        minutes
            .convert_to_with_options(
                unit::Time::Second,
                ConversionOptions::default()
            )
            .expect("minute should convert to second"),
        measurement::Time::new(Decimal::new(120, 0), unit::Time::Second),
    );
}

#[test]
fn test_angle_conversion_constants_preserve_full_revolution_identity() {
    let degrees = measurement::Angle::new(dec!(360), unit::Angle::Degree);

    let revolution = degrees
        .convert_to_with_options(
            unit::Angle::Revolution,
            ConversionOptions::default(),
        )
        .expect("degrees should convert to revolutions");

    assert_eq!(revolution.value, Decimal::ONE);
}

#[test]
fn test_angular_velocity_constants_preserve_minute_second_identity() {
    let revolutions_per_minute = measurement::AngularVelocity::new(
        dec!(60),
        unit::AngularVelocity::RevolutionPerMinute,
    );

    let revolutions_per_second = revolutions_per_minute
        .convert_to_with_options(
            unit::AngularVelocity::RevolutionPerSecond,
            ConversionOptions::default(),
        )
        .expect("revolutions per minute should convert per second");

    assert_eq!(revolutions_per_second.value, Decimal::ONE);
}

#[test]
fn test_measurement_from_str_parses_compact_value_and_typed_unit() {
    let measurement = measurement::Length::from_str("50.0cm")
        .expect("compact measurement should parse");

    assert_eq!(
        measurement,
        measurement::Length::new(
            Decimal::new(500, 1),
            unit::Length::Centimeter
        ),
    );
}

#[test]
fn test_measurement_from_str_parses_spaced_value_and_typed_unit() {
    let measurement = measurement::Mass::from_str("  1.25 kg  ")
        .expect("spaced measurement should parse");

    assert_eq!(
        measurement,
        measurement::Mass::new(Decimal::new(125, 2), unit::Mass::Kilogram),
    );
}

#[test]
fn test_measurement_from_str_parses_signed_and_fractional_values() {
    assert_eq!(
        measurement::Mass::from_str("-1.25kg")
            .expect("negative measurement should parse"),
        measurement::Mass::new(Decimal::new(-125, 2), unit::Mass::Kilogram),
    );
    assert_eq!(
        measurement::Length::from_str("+3cm")
            .expect("positive measurement should parse"),
        measurement::Length::new(Decimal::new(3, 0), unit::Length::Centimeter),
    );
    assert_eq!(
        measurement::Length::from_str(".5m")
            .expect("fractional measurement should parse"),
        measurement::Length::new(Decimal::new(5, 1), unit::Length::Meter),
    );
}

#[test]
fn test_measurement_from_str_parses_scientific_notation_values() {
    assert_eq!(
        measurement::Length::from_str("1e-3 m")
            .expect("scientific length should parse"),
        measurement::Length::new(Decimal::new(1, 3), unit::Length::Meter),
    );
    assert_eq!(
        measurement::Pressure::from_str("-2.5E+3Pa")
            .expect("scientific pressure should parse"),
        measurement::Pressure::new(
            Decimal::new(-2500, 0),
            unit::Pressure::Pascal
        ),
    );
}

#[test]
fn test_measurement_from_str_keeps_unit_starting_with_e() {
    assert_eq!(
        measurement::Energy::from_str("1eV")
            .expect("electronvolt should parse without a space"),
        measurement::Energy::new(Decimal::ONE, unit::Energy::Electronvolt),
    );
}

#[test]
fn test_measurement_from_str_parses_compact_ascii_unit_aliases() {
    assert_eq!(
        measurement::Area::from_str("1m2")
            .expect("compact square meter alias should parse"),
        measurement::Area::new(Decimal::ONE, unit::Area::SquareMeter),
    );
    assert_eq!(
        measurement::Velocity::from_str("65mph")
            .expect("compact mph alias should parse"),
        measurement::Velocity::new(
            Decimal::new(65, 0),
            unit::Velocity::MilePerHour
        ),
    );
}

#[test]
fn test_typed_measurement_from_str_rejects_unit_from_other_quantity() {
    let error = measurement::Length::from_str("12 kg")
        .expect_err("mass unit should not parse as length");

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
    let error = measurement::Length::from_str("12")
        .expect_err("missing unit should fail");

    assert_eq!(error, MeasurementError::InvalidMeasurement("12".to_owned()));
}

#[test]
fn test_measurement_from_str_rejects_missing_value() {
    let error = measurement::Length::from_str("cm")
        .expect_err("missing value should fail");

    assert_eq!(error, MeasurementError::InvalidMeasurement("cm".to_owned()));
}

#[test]
fn test_measurement_from_str_rejects_invalid_value() {
    for input in ["1..2cm", "1+2", "1.2.3m"] {
        let error = measurement::Length::from_str(input)
            .expect_err("invalid decimal should fail");

        assert_eq!(
            error,
            MeasurementError::InvalidMeasurement(input.to_owned()),
        );
    }
}

#[test]
fn test_measurement_from_str_rejects_decimal_overflow() {
    let input = "792281625142643375935439503360 m";

    let error = measurement::Length::from_str(input)
        .expect_err("overflowing decimal should fail");

    assert_eq!(
        error,
        MeasurementError::InvalidMeasurement(input.to_owned())
    );
}

#[test]
fn test_measurement_display_formats_value_and_unit() {
    let measurement = measurement::Length::new(
        Decimal::new(500, 1),
        unit::Length::Centimeter,
    );

    assert_eq!(measurement.to_string(), "50.0 cm");
}

#[test]
fn test_generic_measurement_type_remains_available_for_helpers() {
    fn format_measurement<U: Unit>(measurement: Measurement<U>) -> String {
        measurement.to_string()
    }

    let measurement = measurement::Length::new(
        Decimal::new(500, 1),
        unit::Length::Centimeter,
    );
    let _: measurement::Length =
        Measurement::new(Decimal::ONE, unit::Length::Meter);
    let _: measurement::ElectricPotential =
        Measurement::new(Decimal::ONE, unit::ElectricPotential::Volt);
    let _: measurement::Voltage =
        Measurement::new(Decimal::ONE, unit::ElectricPotential::Volt);

    assert_eq!(format_measurement(measurement), "50.0 cm");
}
