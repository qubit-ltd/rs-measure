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

/// A persisted area measurement.
pub type Area = Measurement<unit::Area>;

/// A persisted energy measurement.
pub type Energy = Measurement<unit::Energy>;

/// A persisted frequency measurement.
pub type Frequency = Measurement<unit::Frequency>;

/// A persisted length measurement.
pub type Length = Measurement<unit::Length>;

/// A persisted mass measurement.
pub type Mass = Measurement<unit::Mass>;

/// A persisted mass density measurement.
pub type MassDensity = Measurement<unit::MassDensity>;

/// A persisted power measurement.
pub type Power = Measurement<unit::Power>;

/// A persisted pressure measurement.
pub type Pressure = Measurement<unit::Pressure>;

/// A persisted thermodynamic temperature measurement.
pub type Temperature = Measurement<unit::Temperature>;

/// A persisted temperature interval measurement.
pub type TemperatureInterval = Measurement<unit::TemperatureInterval>;

/// A persisted time measurement.
pub type Time = Measurement<unit::Time>;

/// A persisted velocity measurement.
pub type Velocity = Measurement<unit::Velocity>;

/// A persisted volume measurement.
pub type Volume = Measurement<unit::Volume>;
