/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Units for persisted length measurements.

use super::define_measurement_unit;
use uom::si::f64::Length;
use uom::si::length::{
    centimeter,
    foot,
    inch,
    kilometer,
    meter,
    millimeter,
    yard,
};

define_measurement_unit! {
    /// Units for persisted `uom` length quantities.
    pub enum LengthUnit for Length, "length" {
        /// Millimeter (`mm`).
        Millimeter => "mm", millimeter;
        /// Centimeter (`cm`).
        Centimeter => "cm", centimeter;
        /// Meter (`m`).
        Meter => "m", meter;
        /// Kilometer (`km`).
        Kilometer => "km", kilometer;
        /// Inch (`in`).
        Inch => "in", inch;
        /// Foot (`ft`).
        Foot => "ft", foot;
        /// Yard (`yd`).
        Yard => "yd", yard;
    }
}
