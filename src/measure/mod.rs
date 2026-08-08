// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted typed measurements, exact unit families, and optional adapters.

mod conversion_factor;
mod conversion_options;
mod decimal_conversion;
mod information_integer;
mod internal;
mod measurement;
mod measurement_error;
mod measurement_parse_options;
mod time_duration;
mod unit;
mod unit_definition;
mod units;
#[cfg(feature = "uom")]
mod uom_unit;

pub use conversion_factor::ConversionFactor;
pub use conversion_options::ConversionOptions;
pub use measurement::Measurement;
pub use measurement_error::MeasurementError;
pub use measurement_parse_options::MeasurementParseOptions;
pub use unit::Unit;
pub use unit::assert_unit_family_valid;
pub use unit_definition::UnitDefinition;
pub use units::Acceleration;
pub use units::AmountOfSubstance;
pub use units::Angle;
pub use units::AngularVelocity;
pub use units::Area;
pub use units::Capacitance;
pub use units::CatalyticActivity;
pub use units::CatalyticActivityConcentration;
pub use units::DynamicViscosity;
pub use units::ElectricCharge;
pub use units::ElectricCurrent;
pub use units::ElectricCurrentDensity;
pub use units::ElectricField;
pub use units::ElectricPotential;
pub use units::ElectricalConductance;
pub use units::ElectricalConductivity;
pub use units::ElectricalResistance;
pub use units::ElectricalResistivity;
pub use units::Energy;
pub use units::Force;
pub use units::Frequency;
pub use units::HeatCapacity;
pub use units::HeatFluxDensity;
pub use units::Illuminance;
pub use units::Inductance;
pub use units::Information;
pub use units::KinematicViscosity;
pub use units::Length;
pub use units::Luminance;
pub use units::LuminousIntensity;
pub use units::MagneticFieldStrength;
pub use units::MagneticFlux;
pub use units::MagneticFluxDensity;
pub use units::Mass;
pub use units::MassConcentration;
pub use units::MassDensity;
pub use units::MassRate;
pub use units::Molality;
pub use units::MolarConcentration;
pub use units::MolarMass;
pub use units::MolarVolume;
pub use units::Power;
pub use units::Pressure;
pub use units::Radioactivity;
pub use units::SolidAngle;
pub use units::SpecificHeatCapacity;
pub use units::SpecificRadioactivity;
pub use units::SurfaceTension;
pub use units::Temperature;
pub use units::TemperatureInterval;
pub use units::ThermalConductivity;
pub use units::ThermalResistance;
pub use units::Time;
pub use units::Torque;
pub use units::Velocity;
pub use units::Volume;
pub use units::VolumeRate;
#[cfg(feature = "uom")]
pub use uom_unit::UomUnit;
