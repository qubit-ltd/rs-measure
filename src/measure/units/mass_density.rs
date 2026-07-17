// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted mass density measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;
#[cfg(feature = "uom")]
use uom::si::f64::MassDensity as UomMassDensity;
#[cfg(feature = "uom")]
use uom::si::mass_density::{
    gram_per_cubic_centimeter,
    gram_per_cubic_meter,
    kilogram_per_cubic_meter,
    pound_per_cubic_foot,
    pound_per_gallon,
};

define_unit_family! {
    /// Units for persisted mass density measurements.
    pub enum MassDensity for "mass_density" {
        /// Kilogram per cubic meter (`kg/m³`).
        KilogramPerCubicMeter => { symbol: "kg/m³"; definition: crate::consts::mass_density::KILOGRAM_PER_CUBIC_METER; aliases: ["kg/m3", "kg/m^3"]; }
        /// Gram per cubic meter (`g/m³`).
        GramPerCubicMeter => { symbol: "g/m³"; definition: crate::consts::mass_density::GRAM_PER_CUBIC_METER; aliases: ["g/m3", "g/m^3"]; }
        /// Gram per cubic centimeter (`g/cm³`).
        GramPerCubicCentimeter => { symbol: "g/cm³"; definition: crate::consts::mass_density::GRAM_PER_CUBIC_CENTIMETER; aliases: ["g/cm3", "g/cm^3"]; }
        /// Pound per cubic foot (`lb/ft³`).
        PoundPerCubicFoot => { symbol: "lb/ft³"; definition: crate::consts::mass_density::POUND_PER_CUBIC_FOOT; aliases: ["lb/ft3", "lb/ft^3"]; }
        /// Pound per US liquid gallon with canonical symbol `lb/gal (US)`.
        PoundPerUsGallon => { symbol: "lb/gal (US)"; definition: crate::consts::mass_density::POUND_PER_US_GALLON; aliases: ["lb/gal"]; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    MassDensity, UomMassDensity {
        KilogramPerCubicMeter => kilogram_per_cubic_meter;
        GramPerCubicMeter => gram_per_cubic_meter;
        GramPerCubicCentimeter => gram_per_cubic_centimeter;
        PoundPerCubicFoot => pound_per_cubic_foot;
        PoundPerUsGallon => pound_per_gallon;
    }
}
