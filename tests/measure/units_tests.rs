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

fn assert_unit_symbols_parse_display_and_serde_round_trip<U>()
where
    U: Unit + serde::Serialize + for<'de> serde::Deserialize<'de> + std::fmt::Debug,
{
    for unit in U::all() {
        assert_eq!(U::from_str(unit.symbol()).expect("unit symbol should parse"), *unit,);
        assert_eq!(unit.to_string(), unit.symbol());

        let value = serde_json::to_value(unit).expect("unit should serialize");

        assert_eq!(value, json!(unit.symbol()));
        assert_eq!(
            serde_json::from_value::<U>(value).expect("unit should deserialize"),
            *unit,
        );
    }
}

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
fn test_electrical_quantity_family_units_are_available() {
    let current_symbols: Vec<&str> = unit::ElectricCurrent::all().iter().map(|unit| unit.symbol()).collect();
    let potential_symbols: Vec<&str> = unit::ElectricPotential::all()
        .iter()
        .map(|unit| unit.symbol())
        .collect();
    let charge_symbols: Vec<&str> = unit::ElectricCharge::all().iter().map(|unit| unit.symbol()).collect();
    let capacitance_symbols: Vec<&str> = unit::Capacitance::all().iter().map(|unit| unit.symbol()).collect();
    let resistance_symbols: Vec<&str> = unit::ElectricalResistance::all()
        .iter()
        .map(|unit| unit.symbol())
        .collect();
    let conductance_symbols: Vec<&str> = unit::ElectricalConductance::all()
        .iter()
        .map(|unit| unit.symbol())
        .collect();
    let inductance_symbols: Vec<&str> = unit::Inductance::all().iter().map(|unit| unit.symbol()).collect();

    assert_eq!(current_symbols, vec!["pA", "nA", "µA", "mA", "A", "kA", "MA"]);
    assert_eq!(potential_symbols, vec!["nV", "µV", "mV", "V", "kV", "MV"]);
    assert_eq!(charge_symbols, vec!["µC", "mC", "C", "kC", "A · h", "mA · h"],);
    assert_eq!(capacitance_symbols, vec!["pF", "nF", "µF", "mF", "F"]);
    assert_eq!(resistance_symbols, vec!["µΩ", "mΩ", "Ω", "kΩ", "MΩ", "GΩ"]);
    assert_eq!(conductance_symbols, vec!["µS", "mS", "S"]);
    assert_eq!(inductance_symbols, vec!["nH", "µH", "mH", "H"]);
}

#[test]
fn test_mechanical_and_process_quantity_family_units_are_available() {
    let force_symbols: Vec<&str> = unit::Force::all().iter().map(|unit| unit.symbol()).collect();
    let acceleration_symbols: Vec<&str> = unit::Acceleration::all().iter().map(|unit| unit.symbol()).collect();
    let torque_symbols: Vec<&str> = unit::Torque::all().iter().map(|unit| unit.symbol()).collect();
    let angle_symbols: Vec<&str> = unit::Angle::all().iter().map(|unit| unit.symbol()).collect();
    let angular_velocity_symbols: Vec<&str> = unit::AngularVelocity::all().iter().map(|unit| unit.symbol()).collect();
    let volume_rate_symbols: Vec<&str> = unit::VolumeRate::all().iter().map(|unit| unit.symbol()).collect();
    let mass_rate_symbols: Vec<&str> = unit::MassRate::all().iter().map(|unit| unit.symbol()).collect();
    let dynamic_viscosity_symbols: Vec<&str> = unit::DynamicViscosity::all().iter().map(|unit| unit.symbol()).collect();
    let kinematic_viscosity_symbols: Vec<&str> = unit::KinematicViscosity::all()
        .iter()
        .map(|unit| unit.symbol())
        .collect();

    assert_eq!(force_symbols, vec!["mN", "N", "kN", "MN", "gf", "kgf", "lbf"]);
    assert_eq!(acceleration_symbols, vec!["mm/s²", "m/s²", "ft/s²", "g₀"]);
    assert_eq!(
        torque_symbols,
        vec!["mN · m", "N · m", "kN · m", "lbf · ft", "lbf · in"]
    );
    assert_eq!(angle_symbols, vec!["rad", "°", "r", "′", "″"]);
    assert_eq!(angular_velocity_symbols, vec!["rad/s", "°/s", "rps", "rpm"]);
    assert_eq!(
        volume_rate_symbols,
        vec!["m³/s", "m³/h", "mL/s", "L/s", "L/min", "gal/min"],
    );
    assert_eq!(mass_rate_symbols, vec!["mg/s", "g/s", "kg/s", "kg/h", "t/h", "lb/h"]);
    assert_eq!(
        dynamic_viscosity_symbols,
        vec!["µPa · s", "mPa · s", "Pa · s", "P", "cP"]
    );
    assert_eq!(kinematic_viscosity_symbols, vec!["mm²/s", "m²/s", "St", "cSt"]);
}

#[test]
fn test_chemical_medical_and_biological_quantity_family_units_are_available() {
    let amount_symbols: Vec<&str> = unit::AmountOfSubstance::all()
        .iter()
        .map(|unit| unit.symbol())
        .collect();
    let molar_concentration_symbols: Vec<&str> = unit::MolarConcentration::all()
        .iter()
        .map(|unit| unit.symbol())
        .collect();
    let mass_concentration_symbols: Vec<&str> = unit::MassConcentration::all()
        .iter()
        .map(|unit| unit.symbol())
        .collect();
    let catalytic_activity_symbols: Vec<&str> = unit::CatalyticActivity::all()
        .iter()
        .map(|unit| unit.symbol())
        .collect();
    let radioactivity_symbols: Vec<&str> = unit::Radioactivity::all().iter().map(|unit| unit.symbol()).collect();

    assert_eq!(amount_symbols, vec!["µmol", "mmol", "mol", "kmol", "particle"]);
    assert_eq!(
        molar_concentration_symbols,
        vec!["nmol/L", "µmol/L", "mmol/L", "mol/L", "mol/m³", "particle/mL"],
    );
    assert_eq!(
        mass_concentration_symbols,
        vec!["µg/L", "mg/L", "g/L", "kg/m³", "mg/dL", "g/dL"],
    );
    assert_eq!(catalytic_activity_symbols, vec!["µkat", "mkat", "kat", "U", "mU"]);
    assert_eq!(
        radioactivity_symbols,
        vec!["Bq", "kBq", "MBq", "Ci", "mCi", "µCi", "dpm"]
    );
}

#[test]
fn test_electromagnetic_thermal_optical_and_chemical_extension_units_are_available() {
    let electric_field_symbols: Vec<&str> = unit::ElectricField::all().iter().map(|unit| unit.symbol()).collect();
    let current_density_symbols: Vec<&str> = unit::ElectricCurrentDensity::all()
        .iter()
        .map(|unit| unit.symbol())
        .collect();
    let conductivity_symbols: Vec<&str> = unit::ElectricalConductivity::all()
        .iter()
        .map(|unit| unit.symbol())
        .collect();
    let resistivity_symbols: Vec<&str> = unit::ElectricalResistivity::all()
        .iter()
        .map(|unit| unit.symbol())
        .collect();
    let magnetic_flux_density_symbols: Vec<&str> = unit::MagneticFluxDensity::all()
        .iter()
        .map(|unit| unit.symbol())
        .collect();
    let magnetic_flux_symbols: Vec<&str> = unit::MagneticFlux::all().iter().map(|unit| unit.symbol()).collect();
    let magnetic_field_strength_symbols: Vec<&str> = unit::MagneticFieldStrength::all()
        .iter()
        .map(|unit| unit.symbol())
        .collect();
    let heat_capacity_symbols: Vec<&str> = unit::HeatCapacity::all().iter().map(|unit| unit.symbol()).collect();
    let specific_heat_capacity_symbols: Vec<&str> = unit::SpecificHeatCapacity::all()
        .iter()
        .map(|unit| unit.symbol())
        .collect();
    let thermal_conductivity_symbols: Vec<&str> = unit::ThermalConductivity::all()
        .iter()
        .map(|unit| unit.symbol())
        .collect();
    let thermal_resistance_symbols: Vec<&str> = unit::ThermalResistance::all()
        .iter()
        .map(|unit| unit.symbol())
        .collect();
    let heat_flux_density_symbols: Vec<&str> = unit::HeatFluxDensity::all().iter().map(|unit| unit.symbol()).collect();
    let surface_tension_symbols: Vec<&str> = unit::SurfaceTension::all().iter().map(|unit| unit.symbol()).collect();
    let luminous_intensity_symbols: Vec<&str> = unit::LuminousIntensity::all()
        .iter()
        .map(|unit| unit.symbol())
        .collect();
    let illuminance_symbols: Vec<&str> = unit::Illuminance::all().iter().map(|unit| unit.symbol()).collect();
    let luminance_symbols: Vec<&str> = unit::Luminance::all().iter().map(|unit| unit.symbol()).collect();
    let solid_angle_symbols: Vec<&str> = unit::SolidAngle::all().iter().map(|unit| unit.symbol()).collect();
    let molality_symbols: Vec<&str> = unit::Molality::all().iter().map(|unit| unit.symbol()).collect();
    let molar_mass_symbols: Vec<&str> = unit::MolarMass::all().iter().map(|unit| unit.symbol()).collect();
    let molar_volume_symbols: Vec<&str> = unit::MolarVolume::all().iter().map(|unit| unit.symbol()).collect();
    let catalytic_activity_concentration_symbols: Vec<&str> = unit::CatalyticActivityConcentration::all()
        .iter()
        .map(|unit| unit.symbol())
        .collect();
    let specific_radioactivity_symbols: Vec<&str> = unit::SpecificRadioactivity::all()
        .iter()
        .map(|unit| unit.symbol())
        .collect();

    assert_eq!(
        electric_field_symbols,
        vec!["V/m", "V/cm", "V/mm", "V/µm", "kV/mm", "MV/m"],
    );
    assert_eq!(current_density_symbols, vec!["A/m²", "A/cm²", "A/mm²"]);
    assert_eq!(conductivity_symbols, vec!["S/m", "S/cm"]);
    assert_eq!(resistivity_symbols, vec!["mΩ · m", "Ω · m", "Ω · cm", "Ω · mm²/m"]);
    assert_eq!(magnetic_flux_density_symbols, vec!["nT", "µT", "mT", "T", "G"]);
    assert_eq!(magnetic_flux_symbols, vec!["µWb", "mWb", "Wb", "Mx"]);
    assert_eq!(magnetic_field_strength_symbols, vec!["A/m", "A/cm", "Oe"]);
    assert_eq!(heat_capacity_symbols, vec!["J/K", "kJ/K", "J/°C", "cal/K", "Btu/°F"]);
    assert_eq!(
        specific_heat_capacity_symbols,
        vec![
            "J/(kg · K)",
            "kJ/(kg · K)",
            "J/(g · °C)",
            "cal/(g · K)",
            "Btu/(lb · °F)"
        ],
    );
    assert_eq!(
        thermal_conductivity_symbols,
        vec!["mW/(m · K)", "W/(m · K)", "kW/(m · K)", "W/(m · °C)"],
    );
    assert_eq!(thermal_resistance_symbols, vec!["K/mW", "K/W", "K/kW"]);
    assert_eq!(heat_flux_density_symbols, vec!["mW/m²", "W/m²", "kW/m²", "W/cm²"]);
    assert_eq!(surface_tension_symbols, vec!["mN/m", "N/m", "dyn/cm", "J/m²"]);
    assert_eq!(luminous_intensity_symbols, vec!["mcd", "cd", "kcd"]);
    assert_eq!(illuminance_symbols, vec!["lx", "klx", "fc"]);
    assert_eq!(luminance_symbols, vec!["cd/m²", "cd/cm²", "cd/ft²", "fl", "sb"]);
    assert_eq!(solid_angle_symbols, vec!["sr", "sp", "°²"]);
    assert_eq!(molality_symbols, vec!["mol/kg"]);
    assert_eq!(molar_mass_symbols, vec!["mg/mol", "g/mol", "kg/mol"]);
    assert_eq!(molar_volume_symbols, vec!["cm³/mol", "dm³/mol", "m³/mol"]);
    assert_eq!(catalytic_activity_concentration_symbols, vec!["kat/m³", "U/L", "mU/mL"]);
    assert_eq!(specific_radioactivity_symbols, vec!["Bq/kg", "Ci/kg", "dpm/kg"]);
}

#[test]
fn test_unit_symbols_parse_display_and_serde_round_trip() {
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::Length>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::Area>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::Volume>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::Mass>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::Time>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::Pressure>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::Energy>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::Power>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::Velocity>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::Frequency>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::MassDensity>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::Temperature>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::TemperatureInterval>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::ElectricCurrent>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::ElectricPotential>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::ElectricCharge>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::Capacitance>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::ElectricalResistance>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::ElectricalConductance>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::Inductance>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::Force>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::Acceleration>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::Torque>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::Angle>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::AngularVelocity>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::VolumeRate>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::MassRate>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::DynamicViscosity>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::KinematicViscosity>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::AmountOfSubstance>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::MolarConcentration>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::MassConcentration>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::CatalyticActivity>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::Radioactivity>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::ElectricField>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::ElectricCurrentDensity>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::ElectricalConductivity>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::ElectricalResistivity>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::MagneticFluxDensity>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::MagneticFlux>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::MagneticFieldStrength>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::HeatCapacity>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::SpecificHeatCapacity>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::ThermalConductivity>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::ThermalResistance>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::HeatFluxDensity>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::SurfaceTension>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::LuminousIntensity>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::Illuminance>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::Luminance>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::SolidAngle>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::Molality>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::MolarMass>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::MolarVolume>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::CatalyticActivityConcentration>();
    assert_unit_symbols_parse_display_and_serde_round_trip::<unit::SpecificRadioactivity>();
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
    assert_eq!(
        unit::ElectricCurrent::from_str("uA").expect("ASCII microampere should parse"),
        unit::ElectricCurrent::Microampere
    );
    assert_eq!(
        unit::Capacitance::from_str("uF").expect("ASCII microfarad should parse"),
        unit::Capacitance::Microfarad
    );
    assert_eq!(
        unit::Inductance::from_str("uH").expect("ASCII microhenry should parse"),
        unit::Inductance::Microhenry
    );
    assert_eq!(
        unit::MagneticFluxDensity::from_str("uT").expect("ASCII microtesla should parse"),
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
        unit::MassDensity::from_str("kg/m3").expect("ASCII kilogram per cubic meter should parse"),
        unit::MassDensity::KilogramPerCubicMeter
    );
    assert_eq!(
        unit::MassDensity::from_str("g/cm^3").expect("ASCII gram per cubic centimeter should parse"),
        unit::MassDensity::GramPerCubicCentimeter
    );
    assert_eq!(
        unit::Pressure::from_str("mmHg").expect("millimeter mercury alias should parse"),
        unit::Pressure::MillimeterOfMercury
    );
    assert_eq!(
        unit::Velocity::from_str("mph").expect("mile per hour alias should parse"),
        unit::Velocity::MilePerHour
    );
    assert_eq!(
        unit::Velocity::from_str("kph").expect("kilometer per hour alias should parse"),
        unit::Velocity::KilometerPerHour
    );
    assert_eq!(
        unit::Time::from_str("year").expect("year alias should parse"),
        unit::Time::Year
    );
    assert_eq!(
        unit::Time::from_str("yr").expect("year abbreviation should parse"),
        unit::Time::Year
    );
    assert_eq!(
        unit::ElectricPotential::from_str("volt").expect("voltage name should parse"),
        unit::ElectricPotential::Volt
    );
    assert_eq!(
        unit::ElectricCharge::from_str("mAh").expect("battery charge alias should parse"),
        unit::ElectricCharge::MilliampereHour
    );
    assert_eq!(
        unit::ElectricalResistance::from_str("kOhm").expect("ASCII kiloohm should parse"),
        unit::ElectricalResistance::Kiloohm
    );
    assert_eq!(
        unit::Acceleration::from_str("m/s2").expect("ASCII acceleration should parse"),
        unit::Acceleration::MeterPerSecondSquared
    );
    assert_eq!(
        unit::Torque::from_str("Nm").expect("compact newton meter should parse"),
        unit::Torque::NewtonMeter
    );
    assert_eq!(
        unit::Angle::from_str("deg").expect("degree alias should parse"),
        unit::Angle::Degree
    );
    assert_eq!(
        unit::VolumeRate::from_str("m3/h").expect("ASCII cubic meter per hour should parse"),
        unit::VolumeRate::CubicMeterPerHour
    );
    assert_eq!(
        unit::MolarConcentration::from_str("M").expect("molar concentration alias should parse"),
        unit::MolarConcentration::MolePerLiter
    );
    assert_eq!(
        unit::ElectricField::from_str("V/um").expect("ASCII electric field alias should parse"),
        unit::ElectricField::VoltPerMicrometer
    );
    assert_eq!(
        unit::Luminance::from_str("cd/m2").expect("ASCII luminance alias should parse"),
        unit::Luminance::CandelaPerSquareMeter
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
fn test_unit_deserialize_rejects_non_string() {
    assert!(serde_json::from_value::<unit::Length>(json!(123)).is_err());
}
