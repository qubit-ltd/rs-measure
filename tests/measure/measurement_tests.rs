// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_measure::{
    Measurement,
    MeasurementError,
    Unit,
    measurement,
    unit,
};
use rust_decimal::Decimal;
use serde_json::json;
use std::str::FromStr;
use uom::si::area::square_meter;
use uom::si::electric_current::ampere;
use uom::si::electric_potential::volt;
use uom::si::energy::joule;
use uom::si::f64::{
    Area as UomArea,
    ElectricCurrent as UomElectricCurrent,
    ElectricPotential as UomElectricPotential,
    Energy as UomEnergy,
    Frequency as UomFrequency,
    Length as UomLength,
    Mass as UomMass,
    MassDensity as UomMassDensity,
    Power as UomPower,
    Pressure as UomPressure,
    TemperatureInterval as UomTemperatureInterval,
    ThermodynamicTemperature as UomTemperature,
    Time as UomTime,
    Velocity as UomVelocity,
    Volume as UomVolume,
};
use uom::si::frequency::hertz;
use uom::si::length::meter;
use uom::si::mass::kilogram;
use uom::si::mass_density::kilogram_per_cubic_meter;
use uom::si::power::watt;
use uom::si::pressure::pascal;
use uom::si::temperature_interval::kelvin as kelvin_interval;
use uom::si::thermodynamic_temperature::kelvin;
use uom::si::time::second;
use uom::si::velocity::meter_per_second;
use uom::si::volume::liter;

fn assert_approx_eq(actual: f64, expected: f64) {
    let tolerance = expected.abs().max(1.0) * 1.0E-12;
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {actual} to approximately equal {expected}",
    );
}

fn assert_all_unit_variants_bridge_uom<U>()
where
    U: Unit,
{
    for unit in U::all() {
        let measurement = Measurement::<U>::new(Decimal::ONE, *unit);
        let quantity = measurement.to_uom();

        Measurement::<U>::from_uom(quantity, *unit)
            .expect("uom quantity should convert back");
    }
}

#[test]
fn test_length_measurement_serde_preserves_value_and_unit() {
    let measurement = measurement::Length::new(
        Decimal::new(500, 1),
        unit::Length::Centimeter,
    );

    let value = serde_json::to_value(measurement)
        .expect("measurement should serialize");

    assert_eq!(value, json!({ "value": "50.0", "unit": "cm" }));
}

#[test]
fn test_length_measurement_serde_deserializes_value_and_unit() {
    let measurement: measurement::Length = serde_json::from_value(json!({
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
fn test_mass_measurement_convert_to_uses_uom_conversion() {
    let measurement =
        measurement::Mass::new(Decimal::new(1, 1), unit::Mass::Gram);

    let converted = measurement
        .convert_to(unit::Mass::Kilogram)
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
        .convert_to(unit::Length::Meter)
        .expect("same unit conversion should be a no-op");

    assert_eq!(converted, measurement);
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
            .convert_to(unit::Length::Meter)
            .expect("millimeter should convert to meter"),
        measurement::Length::new(Decimal::ONE, unit::Length::Meter),
    );
    assert_eq!(
        square_centimeters
            .convert_to(unit::Area::SquareMeter)
            .expect("square centimeter should convert to square meter"),
        measurement::Area::new(Decimal::ONE, unit::Area::SquareMeter),
    );
    assert_eq!(
        milliliters
            .convert_to(unit::Volume::Liter)
            .expect("milliliter should convert to liter"),
        measurement::Volume::new(Decimal::ONE, unit::Volume::Liter),
    );
    assert_eq!(
        minutes
            .convert_to(unit::Time::Second)
            .expect("minute should convert to second"),
        measurement::Time::new(Decimal::new(120, 0), unit::Time::Second),
    );
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
    let error = measurement::Length::from_str("1..2cm")
        .expect_err("invalid decimal should fail");

    assert_eq!(
        error,
        MeasurementError::InvalidMeasurement("1..2cm".to_owned())
    );
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
fn test_length_measurement_to_uom_converts_unit() {
    let measurement =
        measurement::Length::new(Decimal::new(50, 0), unit::Length::Centimeter);
    let millimeters = measurement::Length::new(
        Decimal::new(500, 0),
        unit::Length::Millimeter,
    );
    let meters =
        measurement::Length::new(Decimal::new(2, 0), unit::Length::Meter);

    let length = measurement.to_uom();

    assert_eq!(length.get::<meter>(), 0.5);
    assert_eq!(millimeters.to_uom().get::<meter>(), 0.5);
    assert_eq!(meters.to_uom().get::<meter>(), 2.0);
}

#[test]
fn test_mass_measurement_to_uom_converts_unit() {
    let measurement =
        measurement::Mass::new(Decimal::new(1, 0), unit::Mass::Tonne);
    let grams = measurement::Mass::new(Decimal::new(500, 0), unit::Mass::Gram);
    let kilograms =
        measurement::Mass::new(Decimal::new(2, 0), unit::Mass::Kilogram);

    let mass = measurement.to_uom();

    assert_eq!(mass.get::<kilogram>(), 1000.0);
    assert_eq!(grams.to_uom().get::<kilogram>(), 0.5);
    assert_eq!(kilograms.to_uom().get::<kilogram>(), 2.0);
}

#[test]
fn test_time_measurement_to_uom_converts_unit() {
    let measurement =
        measurement::Time::new(Decimal::new(2, 0), unit::Time::Minute);

    let time = measurement.to_uom();

    assert_eq!(time.get::<second>(), 120.0);
}

#[test]
fn test_area_and_volume_measurements_to_uom_convert_units() {
    let area = measurement::Area::new(
        Decimal::new(10000, 0),
        unit::Area::SquareCentimeter,
    );
    let volume =
        measurement::Volume::new(Decimal::new(1, 0), unit::Volume::Liter);

    assert_eq!(area.to_uom().get::<square_meter>(), 1.0);
    assert_eq!(volume.to_uom().get::<liter>(), 1.0);
}

#[test]
fn test_new_quantity_families_to_uom_convert_units() {
    let pressure = measurement::Pressure::new(
        Decimal::new(1013, 1),
        unit::Pressure::Kilopascal,
    );
    let millipascal = measurement::Pressure::new(
        Decimal::new(2500, 0),
        unit::Pressure::Millipascal,
    );
    let energy =
        measurement::Energy::new(Decimal::ONE, unit::Energy::KilowattHour);
    let power =
        measurement::Power::new(Decimal::new(25, 1), unit::Power::Kilowatt);
    let milliwatt =
        measurement::Power::new(Decimal::new(2500, 0), unit::Power::Milliwatt);
    let velocity = measurement::Velocity::new(
        Decimal::new(36, 0),
        unit::Velocity::KilometerPerHour,
    );
    let centimeters_per_second = measurement::Velocity::new(
        Decimal::new(100, 0),
        unit::Velocity::CentimeterPerSecond,
    );
    let frequency = measurement::Frequency::new(
        Decimal::new(25, 1),
        unit::Frequency::Kilohertz,
    );
    let density = measurement::MassDensity::new(
        Decimal::ONE,
        unit::MassDensity::GramPerCubicCentimeter,
    );
    let temperature = measurement::Temperature::new(
        Decimal::ZERO,
        unit::Temperature::Celsius,
    );
    let interval = measurement::TemperatureInterval::new(
        Decimal::new(10, 0),
        unit::TemperatureInterval::Celsius,
    );

    assert_approx_eq(pressure.to_uom().get::<pascal>(), 101_300.0);
    assert_approx_eq(millipascal.to_uom().get::<pascal>(), 2.5);
    assert_approx_eq(energy.to_uom().get::<joule>(), 3_600_000.0);
    assert_approx_eq(power.to_uom().get::<watt>(), 2_500.0);
    assert_approx_eq(milliwatt.to_uom().get::<watt>(), 2.5);
    assert_approx_eq(velocity.to_uom().get::<meter_per_second>(), 10.0);
    assert_approx_eq(
        centimeters_per_second.to_uom().get::<meter_per_second>(),
        1.0,
    );
    assert_approx_eq(frequency.to_uom().get::<hertz>(), 2_500.0);
    assert_approx_eq(
        density.to_uom().get::<kilogram_per_cubic_meter>(),
        1_000.0,
    );
    assert_approx_eq(temperature.to_uom().get::<kelvin>(), 273.15);
    assert_approx_eq(interval.to_uom().get::<kelvin_interval>(), 10.0);
}

#[test]
fn test_electrical_measurements_to_uom_convert_units() {
    let current = measurement::ElectricCurrent::new(
        Decimal::new(2500, 0),
        unit::ElectricCurrent::Milliampere,
    );
    let voltage = measurement::Voltage::new(
        Decimal::new(12, 0),
        unit::ElectricPotential::Volt,
    );

    assert_approx_eq(current.to_uom().get::<ampere>(), 2.5);
    assert_approx_eq(voltage.to_uom().get::<volt>(), 12.0);
    assert_eq!(voltage.quantity_name(), "electric potential");
}

#[test]
fn test_all_supported_unit_variants_bridge_through_uom() {
    assert_all_unit_variants_bridge_uom::<unit::Length>();
    assert_all_unit_variants_bridge_uom::<unit::Area>();
    assert_all_unit_variants_bridge_uom::<unit::Volume>();
    assert_all_unit_variants_bridge_uom::<unit::Mass>();
    assert_all_unit_variants_bridge_uom::<unit::Time>();
    assert_all_unit_variants_bridge_uom::<unit::Pressure>();
    assert_all_unit_variants_bridge_uom::<unit::Energy>();
    assert_all_unit_variants_bridge_uom::<unit::Power>();
    assert_all_unit_variants_bridge_uom::<unit::Velocity>();
    assert_all_unit_variants_bridge_uom::<unit::Frequency>();
    assert_all_unit_variants_bridge_uom::<unit::MassDensity>();
    assert_all_unit_variants_bridge_uom::<unit::Temperature>();
    assert_all_unit_variants_bridge_uom::<unit::TemperatureInterval>();
    assert_all_unit_variants_bridge_uom::<unit::ElectricCurrent>();
    assert_all_unit_variants_bridge_uom::<unit::ElectricPotential>();
    assert_all_unit_variants_bridge_uom::<unit::ElectricCharge>();
    assert_all_unit_variants_bridge_uom::<unit::Capacitance>();
    assert_all_unit_variants_bridge_uom::<unit::ElectricalResistance>();
    assert_all_unit_variants_bridge_uom::<unit::ElectricalConductance>();
    assert_all_unit_variants_bridge_uom::<unit::Inductance>();
    assert_all_unit_variants_bridge_uom::<unit::Force>();
    assert_all_unit_variants_bridge_uom::<unit::Acceleration>();
    assert_all_unit_variants_bridge_uom::<unit::Torque>();
    assert_all_unit_variants_bridge_uom::<unit::Angle>();
    assert_all_unit_variants_bridge_uom::<unit::AngularVelocity>();
    assert_all_unit_variants_bridge_uom::<unit::VolumeRate>();
    assert_all_unit_variants_bridge_uom::<unit::MassRate>();
    assert_all_unit_variants_bridge_uom::<unit::DynamicViscosity>();
    assert_all_unit_variants_bridge_uom::<unit::KinematicViscosity>();
    assert_all_unit_variants_bridge_uom::<unit::AmountOfSubstance>();
    assert_all_unit_variants_bridge_uom::<unit::MolarConcentration>();
    assert_all_unit_variants_bridge_uom::<unit::MassConcentration>();
    assert_all_unit_variants_bridge_uom::<unit::CatalyticActivity>();
    assert_all_unit_variants_bridge_uom::<unit::Radioactivity>();
    assert_all_unit_variants_bridge_uom::<unit::ElectricField>();
    assert_all_unit_variants_bridge_uom::<unit::ElectricCurrentDensity>();
    assert_all_unit_variants_bridge_uom::<unit::ElectricalConductivity>();
    assert_all_unit_variants_bridge_uom::<unit::ElectricalResistivity>();
    assert_all_unit_variants_bridge_uom::<unit::MagneticFluxDensity>();
    assert_all_unit_variants_bridge_uom::<unit::MagneticFlux>();
    assert_all_unit_variants_bridge_uom::<unit::MagneticFieldStrength>();
    assert_all_unit_variants_bridge_uom::<unit::HeatCapacity>();
    assert_all_unit_variants_bridge_uom::<unit::SpecificHeatCapacity>();
    assert_all_unit_variants_bridge_uom::<unit::ThermalConductivity>();
    assert_all_unit_variants_bridge_uom::<unit::ThermalResistance>();
    assert_all_unit_variants_bridge_uom::<unit::HeatFluxDensity>();
    assert_all_unit_variants_bridge_uom::<unit::SurfaceTension>();
    assert_all_unit_variants_bridge_uom::<unit::LuminousIntensity>();
    assert_all_unit_variants_bridge_uom::<unit::Illuminance>();
    assert_all_unit_variants_bridge_uom::<unit::Luminance>();
    assert_all_unit_variants_bridge_uom::<unit::SolidAngle>();
    assert_all_unit_variants_bridge_uom::<unit::Molality>();
    assert_all_unit_variants_bridge_uom::<unit::MolarMass>();
    assert_all_unit_variants_bridge_uom::<unit::MolarVolume>();
    assert_all_unit_variants_bridge_uom::<unit::CatalyticActivityConcentration>(
    );
    assert_all_unit_variants_bridge_uom::<unit::SpecificRadioactivity>();
}

#[test]
fn test_length_measurement_from_uom_uses_target_unit() {
    let length = UomLength::new::<meter>(0.5);

    let measurement =
        measurement::Length::from_uom(length, unit::Length::Centimeter)
            .expect("uom length should convert to centimeter measurement");

    assert_eq!(
        measurement,
        measurement::Length::new(Decimal::new(50, 0), unit::Length::Centimeter),
    );
    assert_eq!(
        measurement::Length::from_uom(length, unit::Length::Millimeter)
            .expect("uom length should convert to millimeter measurement"),
        measurement::Length::new(
            Decimal::new(500, 0),
            unit::Length::Millimeter
        ),
    );
}

#[test]
fn test_mass_measurement_from_uom_uses_target_unit() {
    let mass = UomMass::new::<kilogram>(1.0);

    assert_eq!(
        measurement::Mass::from_uom(mass, unit::Mass::Gram)
            .expect("uom mass should convert to gram measurement"),
        measurement::Mass::new(Decimal::new(1000, 0), unit::Mass::Gram),
    );
    assert_eq!(
        measurement::Mass::from_uom(mass, unit::Mass::Kilogram)
            .expect("uom mass should convert to kilogram measurement"),
        measurement::Mass::new(Decimal::ONE, unit::Mass::Kilogram),
    );
    assert_eq!(
        measurement::Mass::from_uom(mass, unit::Mass::Tonne)
            .expect("uom mass should convert to tonne measurement"),
        measurement::Mass::new(Decimal::new(1, 3), unit::Mass::Tonne),
    );
}

#[test]
fn test_time_area_and_volume_measurements_from_uom_use_target_unit() {
    let time = UomTime::new::<second>(120.0);
    let area = UomArea::new::<square_meter>(1.0);
    let volume = UomVolume::new::<liter>(1.0);

    assert_eq!(
        measurement::Time::from_uom(time, unit::Time::Minute)
            .expect("uom time should convert to minutes"),
        measurement::Time::new(Decimal::new(2, 0), unit::Time::Minute),
    );
    assert_eq!(
        measurement::Area::from_uom(area, unit::Area::SquareCentimeter)
            .expect("uom area should convert to square centimeters"),
        measurement::Area::new(
            Decimal::new(10000, 0),
            unit::Area::SquareCentimeter
        ),
    );
    assert_eq!(
        measurement::Volume::from_uom(volume, unit::Volume::Milliliter)
            .expect("uom volume should convert to milliliters"),
        measurement::Volume::new(
            Decimal::new(1000, 0),
            unit::Volume::Milliliter
        ),
    );
}

#[test]
fn test_new_quantity_families_from_uom_use_target_unit() {
    let pressure = UomPressure::new::<pascal>(1_000.0);
    let energy = UomEnergy::new::<joule>(3_600.0);
    let power = UomPower::new::<watt>(2_000.0);
    let velocity = UomVelocity::new::<meter_per_second>(10.0);
    let frequency = UomFrequency::new::<hertz>(2_000.0);
    let density = UomMassDensity::new::<kilogram_per_cubic_meter>(1_000.0);
    let temperature = UomTemperature::new::<kelvin>(273.15);
    let interval = UomTemperatureInterval::new::<kelvin_interval>(10.0);

    assert_eq!(
        measurement::Pressure::from_uom(pressure, unit::Pressure::Kilopascal)
            .expect("uom pressure should convert to kilopascals"),
        measurement::Pressure::new(Decimal::ONE, unit::Pressure::Kilopascal),
    );
    assert_eq!(
        measurement::Energy::from_uom(energy, unit::Energy::WattHour)
            .expect("uom energy should convert to watt hours"),
        measurement::Energy::new(Decimal::ONE, unit::Energy::WattHour),
    );
    assert_eq!(
        measurement::Power::from_uom(power, unit::Power::Kilowatt)
            .expect("uom power should convert to kilowatts"),
        measurement::Power::new(Decimal::new(2, 0), unit::Power::Kilowatt),
    );
    assert_eq!(
        measurement::Velocity::from_uom(
            velocity,
            unit::Velocity::KilometerPerHour
        )
        .expect("uom velocity should convert to kilometers per hour"),
        measurement::Velocity::new(
            Decimal::new(36, 0),
            unit::Velocity::KilometerPerHour
        ),
    );
    assert_eq!(
        measurement::Frequency::from_uom(frequency, unit::Frequency::Kilohertz)
            .expect("uom frequency should convert to kilohertz"),
        measurement::Frequency::new(
            Decimal::new(2, 0),
            unit::Frequency::Kilohertz
        ),
    );
    assert_eq!(
        measurement::MassDensity::from_uom(
            density,
            unit::MassDensity::GramPerCubicCentimeter
        )
        .expect(
            "uom mass density should convert to grams per cubic centimeter"
        ),
        measurement::MassDensity::new(
            Decimal::ONE,
            unit::MassDensity::GramPerCubicCentimeter
        ),
    );
    assert_eq!(
        measurement::Temperature::from_uom(
            temperature,
            unit::Temperature::Celsius
        )
        .expect("uom temperature should convert to Celsius"),
        measurement::Temperature::new(
            Decimal::ZERO,
            unit::Temperature::Celsius
        ),
    );
    assert_eq!(
        measurement::TemperatureInterval::from_uom(
            interval,
            unit::TemperatureInterval::Celsius
        )
        .expect("uom temperature interval should convert to Celsius"),
        measurement::TemperatureInterval::new(
            Decimal::new(10, 0),
            unit::TemperatureInterval::Celsius
        ),
    );
}

#[test]
fn test_electrical_measurements_from_uom_use_target_unit() {
    let current = UomElectricCurrent::new::<ampere>(2.5);
    let potential = UomElectricPotential::new::<volt>(12.0);

    assert_eq!(
        measurement::ElectricCurrent::from_uom(
            current,
            unit::ElectricCurrent::Milliampere
        )
        .expect("uom current should convert to milliamperes"),
        measurement::ElectricCurrent::new(
            Decimal::new(2500, 0),
            unit::ElectricCurrent::Milliampere
        ),
    );
    assert_eq!(
        measurement::Voltage::from_uom(
            potential,
            unit::ElectricPotential::Volt
        )
        .expect("uom electric potential should convert to volts"),
        measurement::Voltage::new(
            Decimal::new(12, 0),
            unit::ElectricPotential::Volt
        ),
    );
}

#[test]
fn test_measurement_from_uom_rejects_nan() {
    let length = UomLength::new::<meter>(f64::NAN);

    let error = measurement::Length::from_uom(length, unit::Length::Meter)
        .expect_err("NaN should not become Decimal");

    assert_eq!(error, MeasurementError::DecimalConversion("NaN".to_owned()));
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

    assert_eq!(format_measurement(measurement), "50.0 cm");
}
