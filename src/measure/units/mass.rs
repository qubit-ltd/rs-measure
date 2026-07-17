// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted mass measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;
#[cfg(feature = "uom")]
use uom::si::f64::Mass as UomMass;
#[cfg(feature = "uom")]
use uom::si::mass::kilogram;

define_unit_family! {
    /// Units for persisted mass measurements.
    pub enum Mass for "mass" {
        /// Microgram (`µg`).
        Microgram => { symbol: "µg"; definition: crate::consts::mass::MICROGRAM; aliases: ["ug", "μg"]; }
        /// Milligram (`mg`).
        Milligram => { symbol: "mg"; definition: crate::consts::mass::MILLIGRAM; }
        /// Gram (`g`).
        Gram => { symbol: "g"; definition: crate::consts::mass::GRAM; }
        /// Kilogram (`kg`).
        Kilogram => { symbol: "kg"; definition: crate::consts::mass::KILOGRAM; }
        /// Metric tonne (`t`).
        Tonne => { symbol: "t"; definition: crate::consts::mass::TONNE; }
        /// Carat (`ct`).
        Carat => { symbol: "ct"; definition: crate::consts::mass::CARAT; }
        /// Ounce (`oz`).
        Ounce => { symbol: "oz"; definition: crate::consts::mass::OUNCE; }
        /// Pound (`lb`).
        Pound => { symbol: "lb"; definition: crate::consts::mass::POUND; }
        /// Short ton (`2000 lb`).
        TonShort => { symbol: "2000 lb"; definition: crate::consts::mass::TON_SHORT; }
        /// Long ton (`2240 lb`).
        TonLong => { symbol: "2240 lb"; definition: crate::consts::mass::TON_LONG; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    Mass, UomMass {
        base: kilogram;
    }
}
