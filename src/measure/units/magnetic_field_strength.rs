/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Units for persisted magnetic field strength measurements.

use super::define_measurement_unit;
use uom::si::f64::MagneticFieldStrength as UomMagneticFieldStrength;
use uom::si::magnetic_field_strength::{
    ampere_per_centimeter,
    ampere_per_meter,
    oersted,
};

define_measurement_unit! {
    /// Units for persisted `uom` magnetic field strength quantities.
    pub enum MagneticFieldStrength for UomMagneticFieldStrength, "magnetic field strength" {
        /// Ampere per meter (`A/m`).
        AmperePerMeter => "A/m", ampere_per_meter;
        /// Ampere per centimeter (`A/cm`).
        AmperePerCentimeter => "A/cm", ampere_per_centimeter;
        /// Oersted (`Oe`).
        Oersted => "Oe", oersted;
    }
}
