/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Units for persisted volume measurements.

use super::define_measurement_unit;
use uom::si::f64::Volume;
use uom::si::volume::{
    cubic_centimeter,
    cubic_inch,
    cubic_meter,
    cubic_millimeter,
    gallon,
    liter,
    milliliter,
};

define_measurement_unit! {
    /// Units for persisted `uom` volume quantities.
    pub enum VolumeUnit for Volume, "volume" {
        /// Cubic millimeter (`mm³`).
        CubicMillimeter => "mm³", cubic_millimeter;
        /// Cubic centimeter (`cm³`).
        CubicCentimeter => "cm³", cubic_centimeter;
        /// Cubic meter (`m³`).
        CubicMeter => "m³", cubic_meter;
        /// Milliliter (`mL`).
        Milliliter => "mL", milliliter;
        /// Liter (`L`).
        Liter => "L", liter;
        /// Cubic inch (`in³`).
        CubicInch => "in³", cubic_inch;
        /// Gallon (`gal`).
        Gallon => "gal", gallon;
    }
}
