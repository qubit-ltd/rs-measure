/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Units for persisted mass density measurements.

use super::define_measurement_unit;
use uom::si::f64::MassDensity as UomMassDensity;
use uom::si::mass_density::{
    gram_per_cubic_centimeter,
    gram_per_cubic_meter,
    kilogram_per_cubic_meter,
    pound_per_cubic_foot,
    pound_per_gallon,
};

define_measurement_unit! {
    /// Units for persisted `uom` mass density quantities.
    pub enum MassDensity for UomMassDensity, "mass density" {
        /// Kilogram per cubic meter (`kg/m³`).
        KilogramPerCubicMeter => "kg/m³", kilogram_per_cubic_meter;
        /// Gram per cubic meter (`g/m³`).
        GramPerCubicMeter => "g/m³", gram_per_cubic_meter;
        /// Gram per cubic centimeter (`g/cm³`).
        GramPerCubicCentimeter => "g/cm³", gram_per_cubic_centimeter;
        /// Pound per cubic foot (`lb/ft³`).
        PoundPerCubicFoot => "lb/ft³", pound_per_cubic_foot;
        /// Pound per gallon (`lb/gal`).
        PoundPerGallon => "lb/gal", pound_per_gallon;
    }
}
