// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted typed measurements, unit families, and `uom` adapters.

mod measurement;
mod measurement_error;
mod unit;
mod units;

pub use measurement::Measurement;
pub use measurement_error::MeasurementError;
pub use unit::Unit;
pub use units::{
    Acceleration,
    AmountOfSubstance,
    Angle,
    AngularVelocity,
    Area,
    Capacitance,
    CatalyticActivity,
    CatalyticActivityConcentration,
    DynamicViscosity,
    ElectricCharge,
    ElectricCurrent,
    ElectricCurrentDensity,
    ElectricField,
    ElectricPotential,
    ElectricalConductance,
    ElectricalConductivity,
    ElectricalResistance,
    ElectricalResistivity,
    Energy,
    Force,
    Frequency,
    HeatCapacity,
    HeatFluxDensity,
    Illuminance,
    Inductance,
    KinematicViscosity,
    Length,
    Luminance,
    LuminousIntensity,
    MagneticFieldStrength,
    MagneticFlux,
    MagneticFluxDensity,
    Mass,
    MassConcentration,
    MassDensity,
    MassRate,
    Molality,
    MolarConcentration,
    MolarMass,
    MolarVolume,
    Power,
    Pressure,
    Radioactivity,
    SolidAngle,
    SpecificHeatCapacity,
    SpecificRadioactivity,
    SurfaceTension,
    Temperature,
    TemperatureInterval,
    ThermalConductivity,
    ThermalResistance,
    Time,
    Torque,
    Velocity,
    Volume,
    VolumeRate,
};
