// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_measure::Measurement;
use qubit_measure::MeasurementError;
use qubit_measure::Unit;
use qubit_measure::UnitDefinition;
use qubit_measure::UomUnit;
use qubit_measure::measurement;
use qubit_measure::unit;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use uom::si::area::square_meter;
use uom::si::electric_current::ampere;
use uom::si::electric_potential::volt;
use uom::si::energy::joule;
use uom::si::f64::Acceleration as UomAcceleration;
use uom::si::f64::AmountOfSubstance as UomAmountOfSubstance;
use uom::si::f64::Angle as UomAngle;
use uom::si::f64::AngularVelocity as UomAngularVelocity;
use uom::si::f64::Area as UomArea;
use uom::si::f64::Capacitance as UomCapacitance;
use uom::si::f64::CatalyticActivity as UomCatalyticActivity;
use uom::si::f64::CatalyticActivityConcentration as UomCatalyticActivityConcentration;
use uom::si::f64::DynamicViscosity as UomDynamicViscosity;
use uom::si::f64::ElectricCharge as UomElectricCharge;
use uom::si::f64::ElectricCurrent as UomElectricCurrent;
use uom::si::f64::ElectricCurrentDensity as UomElectricCurrentDensity;
use uom::si::f64::ElectricField as UomElectricField;
use uom::si::f64::ElectricPotential as UomElectricPotential;
use uom::si::f64::ElectricalConductance as UomElectricalConductance;
use uom::si::f64::ElectricalConductivity as UomElectricalConductivity;
use uom::si::f64::ElectricalResistance as UomElectricalResistance;
use uom::si::f64::ElectricalResistivity as UomElectricalResistivity;
use uom::si::f64::Energy as UomEnergy;
use uom::si::f64::Force as UomForce;
use uom::si::f64::Frequency as UomFrequency;
use uom::si::f64::HeatCapacity as UomHeatCapacity;
use uom::si::f64::HeatFluxDensity as UomHeatFluxDensity;
use uom::si::f64::Illuminance as UomIlluminance;
use uom::si::f64::Inductance as UomInductance;
use uom::si::f64::Information as UomInformation;
use uom::si::f64::KinematicViscosity as UomKinematicViscosity;
use uom::si::f64::Length as UomLength;
use uom::si::f64::Luminance as UomLuminance;
use uom::si::f64::LuminousIntensity as UomLuminousIntensity;
use uom::si::f64::MagneticFieldStrength as UomMagneticFieldStrength;
use uom::si::f64::MagneticFlux as UomMagneticFlux;
use uom::si::f64::MagneticFluxDensity as UomMagneticFluxDensity;
use uom::si::f64::Mass as UomMass;
use uom::si::f64::MassConcentration as UomMassConcentration;
use uom::si::f64::MassDensity as UomMassDensity;
use uom::si::f64::MassRate as UomMassRate;
use uom::si::f64::Molality as UomMolality;
use uom::si::f64::MolarConcentration as UomMolarConcentration;
use uom::si::f64::MolarMass as UomMolarMass;
use uom::si::f64::MolarVolume as UomMolarVolume;
use uom::si::f64::Power as UomPower;
use uom::si::f64::Pressure as UomPressure;
use uom::si::f64::Radioactivity as UomRadioactivity;
use uom::si::f64::SolidAngle as UomSolidAngle;
use uom::si::f64::SpecificHeatCapacity as UomSpecificHeatCapacity;
use uom::si::f64::SpecificRadioactivity as UomSpecificRadioactivity;
use uom::si::f64::SurfaceTension as UomSurfaceTension;
use uom::si::f64::TemperatureInterval as UomTemperatureInterval;
use uom::si::f64::ThermalConductivity as UomThermalConductivity;
use uom::si::f64::ThermalResistance as UomThermalResistance;
use uom::si::f64::ThermodynamicTemperature as UomTemperature;
use uom::si::f64::Time as UomTime;
use uom::si::f64::Torque as UomTorque;
use uom::si::f64::Velocity as UomVelocity;
use uom::si::f64::Volume as UomVolume;
use uom::si::f64::VolumeRate as UomVolumeRate;
use uom::si::frequency::hertz;
use uom::si::heat_capacity::joule_per_kelvin;
use uom::si::information::byte;
use uom::si::length::meter;
use uom::si::mass::kilogram;
use uom::si::mass_density::kilogram_per_cubic_meter;
use uom::si::power::watt;
use uom::si::pressure::pascal;
use uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;
use uom::si::temperature_interval::kelvin as kelvin_interval;
use uom::si::thermodynamic_temperature::kelvin;
use uom::si::time::second;
use uom::si::velocity::meter_per_second;
use uom::si::volume::liter;

use crate::measure::fixtures::FallibleUomUnit;
use crate::measure::fixtures::TryOnlyUomUnit;

/// Maximum relative error allowed by the independent SI base oracle.
const UOM_ORACLE_RELATIVE_TOLERANCE: f64 = 1.0E-12;

/// Requires a unit family to expose the expected strongly typed uom quantity.
///
/// # Type Parameters
///
/// * `U` - Unit family whose associated quantity is checked.
/// * `Q` - Expected `uom/f64` quantity type.
fn assert_uom_quantity_type<U, Q>()
where
    U: UomUnit<Quantity = Q>,
{
}

#[test]
fn test_all_uom_unit_families_use_expected_quantity_types() {
    assert_uom_quantity_type::<unit::Acceleration, UomAcceleration>();
    assert_uom_quantity_type::<unit::AmountOfSubstance, UomAmountOfSubstance>();
    assert_uom_quantity_type::<unit::Angle, UomAngle>();
    assert_uom_quantity_type::<unit::AngularVelocity, UomAngularVelocity>();
    assert_uom_quantity_type::<unit::Area, UomArea>();
    assert_uom_quantity_type::<unit::Capacitance, UomCapacitance>();
    assert_uom_quantity_type::<unit::CatalyticActivity, UomCatalyticActivity>();
    assert_uom_quantity_type::<
        unit::CatalyticActivityConcentration,
        UomCatalyticActivityConcentration,
    >();
    assert_uom_quantity_type::<unit::DynamicViscosity, UomDynamicViscosity>();
    assert_uom_quantity_type::<unit::ElectricCharge, UomElectricCharge>();
    assert_uom_quantity_type::<unit::ElectricCurrent, UomElectricCurrent>();
    assert_uom_quantity_type::<
        unit::ElectricCurrentDensity,
        UomElectricCurrentDensity,
    >();
    assert_uom_quantity_type::<unit::ElectricField, UomElectricField>();
    assert_uom_quantity_type::<unit::ElectricPotential, UomElectricPotential>();
    assert_uom_quantity_type::<
        unit::ElectricalConductance,
        UomElectricalConductance,
    >();
    assert_uom_quantity_type::<
        unit::ElectricalConductivity,
        UomElectricalConductivity,
    >();
    assert_uom_quantity_type::<
        unit::ElectricalResistance,
        UomElectricalResistance,
    >();
    assert_uom_quantity_type::<
        unit::ElectricalResistivity,
        UomElectricalResistivity,
    >();
    assert_uom_quantity_type::<unit::Energy, UomEnergy>();
    assert_uom_quantity_type::<unit::Force, UomForce>();
    assert_uom_quantity_type::<unit::Frequency, UomFrequency>();
    assert_uom_quantity_type::<unit::HeatCapacity, UomHeatCapacity>();
    assert_uom_quantity_type::<unit::HeatFluxDensity, UomHeatFluxDensity>();
    assert_uom_quantity_type::<unit::Illuminance, UomIlluminance>();
    assert_uom_quantity_type::<unit::Inductance, UomInductance>();
    assert_uom_quantity_type::<unit::Information, UomInformation>();
    assert_uom_quantity_type::<unit::KinematicViscosity, UomKinematicViscosity>(
    );
    assert_uom_quantity_type::<unit::Length, UomLength>();
    assert_uom_quantity_type::<unit::Luminance, UomLuminance>();
    assert_uom_quantity_type::<unit::LuminousIntensity, UomLuminousIntensity>();
    assert_uom_quantity_type::<
        unit::MagneticFieldStrength,
        UomMagneticFieldStrength,
    >();
    assert_uom_quantity_type::<unit::MagneticFlux, UomMagneticFlux>();
    assert_uom_quantity_type::<unit::MagneticFluxDensity, UomMagneticFluxDensity>(
    );
    assert_uom_quantity_type::<unit::Mass, UomMass>();
    assert_uom_quantity_type::<unit::MassConcentration, UomMassConcentration>();
    assert_uom_quantity_type::<unit::MassDensity, UomMassDensity>();
    assert_uom_quantity_type::<unit::MassRate, UomMassRate>();
    assert_uom_quantity_type::<unit::Molality, UomMolality>();
    assert_uom_quantity_type::<unit::MolarConcentration, UomMolarConcentration>(
    );
    assert_uom_quantity_type::<unit::MolarMass, UomMolarMass>();
    assert_uom_quantity_type::<unit::MolarVolume, UomMolarVolume>();
    assert_uom_quantity_type::<unit::Power, UomPower>();
    assert_uom_quantity_type::<unit::Pressure, UomPressure>();
    assert_uom_quantity_type::<unit::Radioactivity, UomRadioactivity>();
    assert_uom_quantity_type::<unit::SolidAngle, UomSolidAngle>();
    assert_uom_quantity_type::<
        unit::SpecificHeatCapacity,
        UomSpecificHeatCapacity,
    >();
    assert_uom_quantity_type::<
        unit::SpecificRadioactivity,
        UomSpecificRadioactivity,
    >();
    assert_uom_quantity_type::<unit::SurfaceTension, UomSurfaceTension>();
    assert_uom_quantity_type::<unit::Temperature, UomTemperature>();
    assert_uom_quantity_type::<unit::TemperatureInterval, UomTemperatureInterval>(
    );
    assert_uom_quantity_type::<unit::ThermalConductivity, UomThermalConductivity>(
    );
    assert_uom_quantity_type::<unit::ThermalResistance, UomThermalResistance>();
    assert_uom_quantity_type::<unit::Time, UomTime>();
    assert_uom_quantity_type::<unit::Torque, UomTorque>();
    assert_uom_quantity_type::<unit::Velocity, UomVelocity>();
    assert_uom_quantity_type::<unit::Volume, UomVolume>();
    assert_uom_quantity_type::<unit::VolumeRate, UomVolumeRate>();
}

/// Checks two floating-point values with a relative tolerance.
///
/// # Parameters
///
/// * `actual` - The bridge result.
/// * `expected` - The SI oracle value.
///
/// # Panics
///
/// Panics when the values differ by more than one part in 10^12.
fn assert_approx_eq(actual: f64, expected: f64) {
    let tolerance = expected.abs().max(1.0) * 1.0E-12;
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {actual} to approximately equal {expected}",
    );
}

/// Checks an oracle result with a caller-selected relative tolerance.
///
/// # Parameters
///
/// * `actual` - The bridge result.
/// * `expected` - The independently computed SI oracle value.
/// * `tolerance` - Maximum allowed relative error.
/// * `quantity` - Quantity identifier used in failure diagnostics.
/// * `symbol` - Unit symbol used in failure diagnostics.
/// * `sample` - Source value used in failure diagnostics.
///
/// # Panics
///
/// Panics when either value is non-finite, the expected value is zero, or the
/// relative error exceeds `tolerance`.
fn assert_uom_oracle_relative_eq(
    actual: f64,
    expected: f64,
    tolerance: f64,
    quantity: &str,
    symbol: &str,
    sample: f64,
) {
    assert!(actual.is_finite(), "actual value must be finite: {actual}");
    assert!(
        expected.is_finite(),
        "expected value must be finite: {expected}",
    );
    assert_ne!(expected, 0.0, "expected value must be non-zero");

    let relative_error = (actual - expected).abs() / expected.abs();
    assert!(
        relative_error <= tolerance,
        "uom oracle mismatch for {quantity} unit {symbol:?} at {sample}: \
         expected {actual} to approximately equal {expected}; relative error \
         {relative_error} exceeds {tolerance}",
    );
}

/// Checks every variant against an independently computed SI base value.
macro_rules! assert_unit_family_matches_uom_base {
    ($unit:ty) => {{
        let identity_unit = <$unit>::all()
            .iter()
            .copied()
            .find(|unit| {
                unit.definition().expect("unit definition should be valid")
                    == UnitDefinition::base()
            })
            .expect("unit family should contain an identity definition");

        for unit in <$unit>::all() {
            for (sample_decimal, sample) in
                [(Decimal::ONE, 1.0), (Decimal::new(2, 0), 2.0)]
            {
                let source = Measurement::<$unit>::new(sample_decimal, *unit);
                let definition =
                    unit.definition().expect("unit definition should be valid");
                let factor = definition.factor();
                let offset = definition
                    .offset()
                    .to_f64()
                    .expect("unit offset should fit f64 for the oracle");
                let numerator = factor
                    .numerator()
                    .to_f64()
                    .expect("factor numerator should fit f64 for the oracle");
                let denominator = factor
                    .denominator()
                    .to_f64()
                    .expect("factor denominator should fit f64 for the oracle");
                let expected_base = (sample + offset) * numerator / denominator;
                let quantity = source.to_uom_approx();

                assert_uom_oracle_relative_eq(
                    quantity.value,
                    expected_base,
                    UOM_ORACLE_RELATIVE_TOLERANCE,
                    <$unit>::QUANTITY,
                    unit.symbol(),
                    sample,
                );

                let mut independent_base =
                    Measurement::<$unit>::new(Decimal::ZERO, identity_unit)
                        .to_uom_approx();
                independent_base.value = expected_base;
                let round_trip = Measurement::<$unit>::from_uom_approx(
                    independent_base,
                    *unit,
                )
                .expect("independent SI base value should convert to the unit");
                assert_uom_oracle_relative_eq(
                    round_trip
                        .value
                        .to_f64()
                        .expect("round-trip Decimal should fit f64"),
                    sample,
                    UOM_ORACLE_RELATIVE_TOLERANCE,
                    <$unit>::QUANTITY,
                    unit.symbol(),
                    sample,
                );
            }
        }
    }};
}

#[test]
fn test_energy_btu_it_uom_mapping_uses_si_oracle() {
    let measurement = measurement::Energy::new(
        Decimal::ONE,
        unit::Energy::BritishThermalUnitInternationalTable,
    );

    assert_approx_eq(measurement.to_uom_approx().get::<joule>(), 1_055.056);
}

#[test]
fn test_heat_capacity_btu_it_uom_mapping_uses_si_oracle() {
    let measurement = measurement::HeatCapacity::new(
        Decimal::ONE,
        unit::HeatCapacity::BritishThermalUnitInternationalTablePerDegreeFahrenheit,
    );

    assert_approx_eq(
        measurement.to_uom_approx().get::<joule_per_kelvin>(),
        1_899.100_8,
    );
}

#[test]
fn test_specific_heat_capacity_btu_it_uom_mapping_uses_si_oracle() {
    let measurement = measurement::SpecificHeatCapacity::new(
        Decimal::ONE,
        unit::SpecificHeatCapacity::BritishThermalUnitInternationalTablePerPoundDegreeFahrenheit,
    );

    assert_approx_eq(
        measurement
            .to_uom_approx()
            .get::<joule_per_kilogram_kelvin>(),
        189_910_080_000.0 / 45_359_237.0,
    );
}

#[test]
fn test_millimeter_of_mercury_uom_mapping_uses_exact_torr_oracle() {
    let measurement = measurement::Pressure::new(
        Decimal::ONE,
        unit::Pressure::MillimeterOfMercury,
    );

    assert_approx_eq(
        measurement.to_uom_approx().get::<pascal>(),
        101_325.0 / 760.0,
    );
    assert_eq!(
        measurement::Pressure::from_uom_approx(
            measurement.to_uom_approx(),
            unit::Pressure::MillimeterOfMercury,
        )
        .expect("exact Torr-equivalent pressure should convert back"),
        measurement,
    );
}

#[test]
fn test_length_measurement_to_uom_approx_converts_unit() {
    let measurement =
        measurement::Length::new(Decimal::new(50, 0), unit::Length::Centimeter);
    let millimeters = measurement::Length::new(
        Decimal::new(500, 0),
        unit::Length::Millimeter,
    );
    let meters =
        measurement::Length::new(Decimal::new(2, 0), unit::Length::Meter);

    let length = measurement.to_uom_approx();

    assert_eq!(length.get::<meter>(), 0.5);
    assert_eq!(millimeters.to_uom_approx().get::<meter>(), 0.5);
    assert_eq!(meters.to_uom_approx().get::<meter>(), 2.0);
}

#[test]
fn test_try_to_uom_approx_returns_invalid_definition_error() {
    let unit = FallibleUomUnit::Invalid;
    let measurement = Measurement::new(Decimal::ONE, unit);

    assert!(matches!(
        unit.try_to_uom_approx(Decimal::ONE),
        Err(MeasurementError::InvalidUnitDefinition { reason })
            if reason == "fallible uom test definition",
    ));
    assert!(matches!(
        measurement.try_to_uom_approx(),
        Err(MeasurementError::InvalidUnitDefinition { reason })
            if reason == "fallible uom test definition",
    ));
}

#[test]
fn test_value_from_uom_approx_returns_invalid_definition_error() {
    let quantity = UomLength::new::<meter>(1.0);

    assert!(matches!(
        FallibleUomUnit::Invalid.value_from_uom_approx(quantity),
        Err(MeasurementError::InvalidUnitDefinition { reason })
            if reason == "fallible uom test definition",
    ));
}

#[test]
fn test_measurement_from_uom_approx_returns_invalid_definition_error() {
    let quantity = UomLength::new::<meter>(1.0);

    assert!(matches!(
        Measurement::<FallibleUomUnit>::from_uom_approx(
            quantity,
            FallibleUomUnit::Invalid,
        ),
        Err(MeasurementError::InvalidUnitDefinition { reason })
            if reason == "fallible uom test definition",
    ));
}

#[test]
fn test_to_uom_approx_uses_fallible_external_implementation() {
    let quantity = TryOnlyUomUnit::Valid.to_uom_approx(Decimal::new(2, 0));

    assert_eq!(quantity.get::<meter>(), 2.0);
}

#[test]
fn test_try_to_uom_approx_returns_external_definition_error() {
    assert!(matches!(
        TryOnlyUomUnit::Invalid.try_to_uom_approx(Decimal::ONE),
        Err(MeasurementError::InvalidUnitDefinition { reason })
            if reason == "try-only uom test definition",
    ));
}

#[test]
fn test_to_uom_approx_convenience_wrapper_converts_builtin_unit() {
    let quantity = unit::Length::Centimeter.to_uom_approx(Decimal::new(50, 0));

    assert_eq!(quantity.get::<meter>(), 0.5);
}

#[test]
fn test_mass_measurement_to_uom_approx_converts_unit() {
    let measurement =
        measurement::Mass::new(Decimal::new(1, 0), unit::Mass::Tonne);
    let grams = measurement::Mass::new(Decimal::new(500, 0), unit::Mass::Gram);
    let kilograms =
        measurement::Mass::new(Decimal::new(2, 0), unit::Mass::Kilogram);

    let mass = measurement.to_uom_approx();

    assert_eq!(mass.get::<kilogram>(), 1000.0);
    assert_eq!(grams.to_uom_approx().get::<kilogram>(), 0.5);
    assert_eq!(kilograms.to_uom_approx().get::<kilogram>(), 2.0);
}

#[test]
fn test_time_measurement_to_uom_approx_converts_unit() {
    let measurement =
        measurement::Time::new(Decimal::new(2, 0), unit::Time::Minute);

    let time = measurement.to_uom_approx();

    assert_eq!(time.get::<second>(), 120.0);
}

#[test]
fn test_area_and_volume_measurements_to_uom_approx_convert_units() {
    let area = measurement::Area::new(
        Decimal::new(10000, 0),
        unit::Area::SquareCentimeter,
    );
    let volume =
        measurement::Volume::new(Decimal::new(1, 0), unit::Volume::Liter);

    assert_eq!(area.to_uom_approx().get::<square_meter>(), 1.0);
    assert_eq!(volume.to_uom_approx().get::<liter>(), 1.0);
}

#[test]
fn test_new_quantity_families_to_uom_approx_convert_units() {
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

    assert_approx_eq(pressure.to_uom_approx().get::<pascal>(), 101_300.0);
    assert_approx_eq(millipascal.to_uom_approx().get::<pascal>(), 2.5);
    assert_approx_eq(energy.to_uom_approx().get::<joule>(), 3_600_000.0);
    assert_approx_eq(power.to_uom_approx().get::<watt>(), 2_500.0);
    assert_approx_eq(milliwatt.to_uom_approx().get::<watt>(), 2.5);
    assert_approx_eq(velocity.to_uom_approx().get::<meter_per_second>(), 10.0);
    assert_approx_eq(
        centimeters_per_second
            .to_uom_approx()
            .get::<meter_per_second>(),
        1.0,
    );
    assert_approx_eq(frequency.to_uom_approx().get::<hertz>(), 2_500.0);
    assert_approx_eq(
        density.to_uom_approx().get::<kilogram_per_cubic_meter>(),
        1_000.0,
    );
    assert_approx_eq(temperature.to_uom_approx().get::<kelvin>(), 273.15);
    assert_approx_eq(interval.to_uom_approx().get::<kelvin_interval>(), 10.0);
}

#[test]
fn test_electrical_measurements_to_uom_approx_convert_units() {
    let current = measurement::ElectricCurrent::new(
        Decimal::new(2500, 0),
        unit::ElectricCurrent::Milliampere,
    );
    let voltage = measurement::Voltage::new(
        Decimal::new(12, 0),
        unit::ElectricPotential::Volt,
    );

    assert_approx_eq(current.to_uom_approx().get::<ampere>(), 2.5);
    assert_approx_eq(voltage.to_uom_approx().get::<volt>(), 12.0);
    assert_eq!(voltage.quantity_name(), "electric_potential");
}

#[test]
fn test_all_supported_unit_variants_bridge_through_uom() {
    assert_unit_family_matches_uom_base!(unit::Length);
    assert_unit_family_matches_uom_base!(unit::Area);
    assert_unit_family_matches_uom_base!(unit::Volume);
    assert_unit_family_matches_uom_base!(unit::Mass);
    assert_unit_family_matches_uom_base!(unit::Time);
    assert_unit_family_matches_uom_base!(unit::Pressure);
    assert_unit_family_matches_uom_base!(unit::Energy);
    assert_unit_family_matches_uom_base!(unit::Power);
    assert_unit_family_matches_uom_base!(unit::Velocity);
    assert_unit_family_matches_uom_base!(unit::Frequency);
    assert_unit_family_matches_uom_base!(unit::MassDensity);
    assert_unit_family_matches_uom_base!(unit::Temperature);
    assert_unit_family_matches_uom_base!(unit::TemperatureInterval);
    assert_unit_family_matches_uom_base!(unit::ElectricCurrent);
    assert_unit_family_matches_uom_base!(unit::ElectricPotential);
    assert_unit_family_matches_uom_base!(unit::ElectricCharge);
    assert_unit_family_matches_uom_base!(unit::Capacitance);
    assert_unit_family_matches_uom_base!(unit::ElectricalResistance);
    assert_unit_family_matches_uom_base!(unit::ElectricalConductance);
    assert_unit_family_matches_uom_base!(unit::Inductance);
    assert_unit_family_matches_uom_base!(unit::Information);
    assert_unit_family_matches_uom_base!(unit::Force);
    assert_unit_family_matches_uom_base!(unit::Acceleration);
    assert_unit_family_matches_uom_base!(unit::Torque);
    assert_unit_family_matches_uom_base!(unit::Angle);
    assert_unit_family_matches_uom_base!(unit::AngularVelocity);
    assert_unit_family_matches_uom_base!(unit::VolumeRate);
    assert_unit_family_matches_uom_base!(unit::MassRate);
    assert_unit_family_matches_uom_base!(unit::DynamicViscosity);
    assert_unit_family_matches_uom_base!(unit::KinematicViscosity);
    assert_unit_family_matches_uom_base!(unit::AmountOfSubstance);
    assert_unit_family_matches_uom_base!(unit::MolarConcentration);
    assert_unit_family_matches_uom_base!(unit::MassConcentration);
    assert_unit_family_matches_uom_base!(unit::CatalyticActivity);
    assert_unit_family_matches_uom_base!(unit::Radioactivity);
    assert_unit_family_matches_uom_base!(unit::ElectricField);
    assert_unit_family_matches_uom_base!(unit::ElectricCurrentDensity);
    assert_unit_family_matches_uom_base!(unit::ElectricalConductivity);
    assert_unit_family_matches_uom_base!(unit::ElectricalResistivity);
    assert_unit_family_matches_uom_base!(unit::MagneticFluxDensity);
    assert_unit_family_matches_uom_base!(unit::MagneticFlux);
    assert_unit_family_matches_uom_base!(unit::MagneticFieldStrength);
    assert_unit_family_matches_uom_base!(unit::HeatCapacity);
    assert_unit_family_matches_uom_base!(unit::SpecificHeatCapacity);
    assert_unit_family_matches_uom_base!(unit::ThermalConductivity);
    assert_unit_family_matches_uom_base!(unit::ThermalResistance);
    assert_unit_family_matches_uom_base!(unit::HeatFluxDensity);
    assert_unit_family_matches_uom_base!(unit::SurfaceTension);
    assert_unit_family_matches_uom_base!(unit::LuminousIntensity);
    assert_unit_family_matches_uom_base!(unit::Illuminance);
    assert_unit_family_matches_uom_base!(unit::Luminance);
    assert_unit_family_matches_uom_base!(unit::SolidAngle);
    assert_unit_family_matches_uom_base!(unit::Molality);
    assert_unit_family_matches_uom_base!(unit::MolarMass);
    assert_unit_family_matches_uom_base!(unit::MolarVolume);
    assert_unit_family_matches_uom_base!(unit::CatalyticActivityConcentration);
    assert_unit_family_matches_uom_base!(unit::SpecificRadioactivity);
}

#[test]
fn test_information_uom_bridge_uses_byte_base() {
    let information = measurement::Information::new(
        Decimal::new(2, 0),
        unit::Information::Kibibyte,
    );

    assert_approx_eq(information.to_uom_approx().get::<byte>(), 2_048.0);
}

#[test]
fn test_length_measurement_from_uom_approx_uses_target_unit() {
    let length = UomLength::new::<meter>(0.5);

    let measurement =
        measurement::Length::from_uom_approx(length, unit::Length::Centimeter)
            .expect("uom length should convert to centimeter measurement");

    assert_eq!(
        measurement,
        measurement::Length::new(Decimal::new(50, 0), unit::Length::Centimeter),
    );
    assert_eq!(
        measurement::Length::from_uom_approx(length, unit::Length::Millimeter)
            .expect("uom length should convert to millimeter measurement"),
        measurement::Length::new(
            Decimal::new(500, 0),
            unit::Length::Millimeter
        ),
    );
}

#[test]
fn test_mass_measurement_from_uom_approx_uses_target_unit() {
    let mass = UomMass::new::<kilogram>(1.0);

    assert_eq!(
        measurement::Mass::from_uom_approx(mass, unit::Mass::Gram)
            .expect("uom mass should convert to gram measurement"),
        measurement::Mass::new(Decimal::new(1000, 0), unit::Mass::Gram),
    );
    assert_eq!(
        measurement::Mass::from_uom_approx(mass, unit::Mass::Kilogram)
            .expect("uom mass should convert to kilogram measurement"),
        measurement::Mass::new(Decimal::ONE, unit::Mass::Kilogram),
    );
    assert_eq!(
        measurement::Mass::from_uom_approx(mass, unit::Mass::Tonne)
            .expect("uom mass should convert to tonne measurement"),
        measurement::Mass::new(Decimal::new(1, 3), unit::Mass::Tonne),
    );
}

#[test]
fn test_time_area_and_volume_measurements_from_uom_approx_use_target_unit() {
    let time = UomTime::new::<second>(120.0);
    let area = UomArea::new::<square_meter>(1.0);
    let volume = UomVolume::new::<liter>(1.0);

    assert_eq!(
        measurement::Time::from_uom_approx(time, unit::Time::Minute)
            .expect("uom time should convert to minutes"),
        measurement::Time::new(Decimal::new(2, 0), unit::Time::Minute),
    );
    assert_eq!(
        measurement::Area::from_uom_approx(area, unit::Area::SquareCentimeter)
            .expect("uom area should convert to square centimeters"),
        measurement::Area::new(
            Decimal::new(10000, 0),
            unit::Area::SquareCentimeter
        ),
    );
    assert_eq!(
        measurement::Volume::from_uom_approx(volume, unit::Volume::Milliliter)
            .expect("uom volume should convert to milliliters"),
        measurement::Volume::new(
            Decimal::new(1000, 0),
            unit::Volume::Milliliter
        ),
    );
}

#[test]
fn test_new_quantity_families_from_uom_approx_use_target_unit() {
    let pressure = UomPressure::new::<pascal>(1_000.0);
    let energy = UomEnergy::new::<joule>(3_600.0);
    let power = UomPower::new::<watt>(2_000.0);
    let velocity = UomVelocity::new::<meter_per_second>(10.0);
    let frequency = UomFrequency::new::<hertz>(2_000.0);
    let density = UomMassDensity::new::<kilogram_per_cubic_meter>(1_000.0);
    let temperature = UomTemperature::new::<kelvin>(273.15);
    let interval = UomTemperatureInterval::new::<kelvin_interval>(10.0);

    assert_eq!(
        measurement::Pressure::from_uom_approx(
            pressure,
            unit::Pressure::Kilopascal
        )
        .expect("uom pressure should convert to kilopascals"),
        measurement::Pressure::new(Decimal::ONE, unit::Pressure::Kilopascal),
    );
    assert_eq!(
        measurement::Energy::from_uom_approx(energy, unit::Energy::WattHour)
            .expect("uom energy should convert to watt hours"),
        measurement::Energy::new(Decimal::ONE, unit::Energy::WattHour),
    );
    assert_eq!(
        measurement::Power::from_uom_approx(power, unit::Power::Kilowatt)
            .expect("uom power should convert to kilowatts"),
        measurement::Power::new(Decimal::new(2, 0), unit::Power::Kilowatt),
    );
    assert_eq!(
        measurement::Velocity::from_uom_approx(
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
        measurement::Frequency::from_uom_approx(
            frequency,
            unit::Frequency::Kilohertz
        )
        .expect("uom frequency should convert to kilohertz"),
        measurement::Frequency::new(
            Decimal::new(2, 0),
            unit::Frequency::Kilohertz
        ),
    );
    assert_eq!(
        measurement::MassDensity::from_uom_approx(
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
        measurement::Temperature::from_uom_approx(
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
        measurement::TemperatureInterval::from_uom_approx(
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
fn test_electrical_measurements_from_uom_approx_use_target_unit() {
    let current = UomElectricCurrent::new::<ampere>(2.5);
    let potential = UomElectricPotential::new::<volt>(12.0);

    assert_eq!(
        measurement::ElectricCurrent::from_uom_approx(
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
        measurement::Voltage::from_uom_approx(
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
fn test_measurement_from_uom_approx_rejects_nan() {
    let length = UomLength::new::<meter>(f64::NAN);

    let error =
        measurement::Length::from_uom_approx(length, unit::Length::Meter)
            .expect_err("NaN should not become Decimal");

    assert_eq!(error, MeasurementError::DecimalConversion("NaN".to_owned()));
}
