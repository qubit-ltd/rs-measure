// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted magnetic field strength measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use uom::si::f64::MagneticFieldStrength as UomMagneticFieldStrength;
#[cfg(feature = "uom")]
use uom::si::magnetic_field_strength::{
    ampere_per_centimeter,
    ampere_per_meter,
    oersted,
};

define_unit_family! {
    /// Units for persisted magnetic field strength measurements.
    pub enum MagneticFieldStrength for "magnetic_field_strength", uom = UomMagneticFieldStrength {
        /// Ampere per meter (`A/m`).
        AmperePerMeter => { symbol: "A/m"; definition: crate::consts::magnetic_field_strength::AMPERE_PER_METER; uom: ampere_per_meter; }
        /// Ampere per centimeter (`A/cm`).
        AmperePerCentimeter => { symbol: "A/cm"; definition: crate::consts::magnetic_field_strength::AMPERE_PER_CENTIMETER; uom: ampere_per_centimeter; }
        /// Oersted (`Oe`).
        Oersted => { symbol: "Oe"; definition: crate::consts::magnetic_field_strength::OERSTED; uom: oersted; }
    }
}
