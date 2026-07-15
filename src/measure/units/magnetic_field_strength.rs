// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted magnetic field strength measurements.

use crate::define_unit_family;
use uom::si::f64::MagneticFieldStrength as UomMagneticFieldStrength;
use uom::si::magnetic_field_strength::{
    ampere_per_centimeter,
    ampere_per_meter,
    oersted,
};

define_unit_family! {
    /// Units for persisted `uom` magnetic field strength quantities.
    pub enum MagneticFieldStrength for "magnetic_field_strength", uom = UomMagneticFieldStrength {
        /// Ampere per meter (`A/m`).
        AmperePerMeter => { symbol: "A/m"; coefficient: 1; uom: ampere_per_meter; }
        /// Ampere per centimeter (`A/cm`).
        AmperePerCentimeter => { symbol: "A/cm"; coefficient: 100; uom: ampere_per_centimeter; }
        /// Oersted (`Oe`).
        Oersted => { symbol: "Oe"; coefficient: 7957747154594767 / 100000000000000; uom: oersted; }
    }
}
