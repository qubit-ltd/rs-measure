/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Persisted measurement aliases for supported `uom` quantity families.

use crate::Measurement;
use crate::unit;

/// A persisted acceleration measurement.
pub type Acceleration = Measurement<unit::Acceleration>;

/// A persisted amount of substance measurement.
pub type AmountOfSubstance = Measurement<unit::AmountOfSubstance>;

/// A persisted angle measurement.
pub type Angle = Measurement<unit::Angle>;

/// A persisted angular velocity measurement.
pub type AngularVelocity = Measurement<unit::AngularVelocity>;

/// A persisted area measurement.
pub type Area = Measurement<unit::Area>;

/// A persisted capacitance measurement.
pub type Capacitance = Measurement<unit::Capacitance>;

/// A persisted catalytic activity measurement.
pub type CatalyticActivity = Measurement<unit::CatalyticActivity>;

/// A persisted catalytic activity concentration measurement.
pub type CatalyticActivityConcentration = Measurement<unit::CatalyticActivityConcentration>;

/// A persisted dynamic viscosity measurement.
pub type DynamicViscosity = Measurement<unit::DynamicViscosity>;

/// A persisted electric charge measurement.
pub type ElectricCharge = Measurement<unit::ElectricCharge>;

/// A persisted electric current measurement.
pub type ElectricCurrent = Measurement<unit::ElectricCurrent>;

/// A persisted electric current density measurement.
pub type ElectricCurrentDensity = Measurement<unit::ElectricCurrentDensity>;

/// A persisted electric field measurement.
pub type ElectricField = Measurement<unit::ElectricField>;

/// A persisted electric potential measurement.
///
/// This is the SI quantity commonly called voltage.
pub type ElectricPotential = Measurement<unit::ElectricPotential>;

/// A persisted voltage measurement.
///
/// This is an ergonomic alias for [`ElectricPotential`].
pub type Voltage = ElectricPotential;

/// A persisted electrical conductance measurement.
pub type ElectricalConductance = Measurement<unit::ElectricalConductance>;

/// A persisted electrical conductivity measurement.
pub type ElectricalConductivity = Measurement<unit::ElectricalConductivity>;

/// A persisted electrical resistance measurement.
pub type ElectricalResistance = Measurement<unit::ElectricalResistance>;

/// A persisted electrical resistivity measurement.
pub type ElectricalResistivity = Measurement<unit::ElectricalResistivity>;

/// A persisted energy measurement.
pub type Energy = Measurement<unit::Energy>;

/// A persisted frequency measurement.
pub type Frequency = Measurement<unit::Frequency>;

/// A persisted force measurement.
pub type Force = Measurement<unit::Force>;

/// A persisted heat capacity measurement.
pub type HeatCapacity = Measurement<unit::HeatCapacity>;

/// A persisted heat flux density measurement.
pub type HeatFluxDensity = Measurement<unit::HeatFluxDensity>;

/// A persisted illuminance measurement.
pub type Illuminance = Measurement<unit::Illuminance>;

/// A persisted inductance measurement.
pub type Inductance = Measurement<unit::Inductance>;

/// A persisted kinematic viscosity measurement.
pub type KinematicViscosity = Measurement<unit::KinematicViscosity>;

/// A persisted length measurement.
pub type Length = Measurement<unit::Length>;

/// A persisted luminance measurement.
pub type Luminance = Measurement<unit::Luminance>;

/// A persisted luminous intensity measurement.
pub type LuminousIntensity = Measurement<unit::LuminousIntensity>;

/// A persisted magnetic field strength measurement.
pub type MagneticFieldStrength = Measurement<unit::MagneticFieldStrength>;

/// A persisted magnetic flux measurement.
pub type MagneticFlux = Measurement<unit::MagneticFlux>;

/// A persisted magnetic flux density measurement.
pub type MagneticFluxDensity = Measurement<unit::MagneticFluxDensity>;

/// A persisted mass measurement.
pub type Mass = Measurement<unit::Mass>;

/// A persisted mass concentration measurement.
pub type MassConcentration = Measurement<unit::MassConcentration>;

/// A persisted mass density measurement.
pub type MassDensity = Measurement<unit::MassDensity>;

/// A persisted mass rate measurement.
pub type MassRate = Measurement<unit::MassRate>;

/// A persisted molality measurement.
pub type Molality = Measurement<unit::Molality>;

/// A persisted molar concentration measurement.
pub type MolarConcentration = Measurement<unit::MolarConcentration>;

/// A persisted molar mass measurement.
pub type MolarMass = Measurement<unit::MolarMass>;

/// A persisted molar volume measurement.
pub type MolarVolume = Measurement<unit::MolarVolume>;

/// A persisted power measurement.
pub type Power = Measurement<unit::Power>;

/// A persisted pressure measurement.
pub type Pressure = Measurement<unit::Pressure>;

/// A persisted radioactivity measurement.
pub type Radioactivity = Measurement<unit::Radioactivity>;

/// A persisted solid angle measurement.
pub type SolidAngle = Measurement<unit::SolidAngle>;

/// A persisted specific heat capacity measurement.
pub type SpecificHeatCapacity = Measurement<unit::SpecificHeatCapacity>;

/// A persisted specific radioactivity measurement.
pub type SpecificRadioactivity = Measurement<unit::SpecificRadioactivity>;

/// A persisted surface tension measurement.
pub type SurfaceTension = Measurement<unit::SurfaceTension>;

/// A persisted thermodynamic temperature measurement.
pub type Temperature = Measurement<unit::Temperature>;

/// A persisted temperature interval measurement.
pub type TemperatureInterval = Measurement<unit::TemperatureInterval>;

/// A persisted thermal conductivity measurement.
pub type ThermalConductivity = Measurement<unit::ThermalConductivity>;

/// A persisted thermal resistance measurement.
pub type ThermalResistance = Measurement<unit::ThermalResistance>;

/// A persisted time measurement.
pub type Time = Measurement<unit::Time>;

/// A persisted torque measurement.
pub type Torque = Measurement<unit::Torque>;

/// A persisted velocity measurement.
pub type Velocity = Measurement<unit::Velocity>;

/// A persisted volume measurement.
pub type Volume = Measurement<unit::Volume>;

/// A persisted volume rate measurement.
pub type VolumeRate = Measurement<unit::VolumeRate>;
