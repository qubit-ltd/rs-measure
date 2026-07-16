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
use uom::si::f64::Mass as UomMass;
#[cfg(feature = "uom")]
use uom::si::mass::{
    carat,
    gram,
    kilogram,
    microgram,
    milligram,
    ounce,
    pound,
    ton,
    ton_long,
    ton_short,
};

define_unit_family! {
    /// Units for persisted mass measurements.
    pub enum Mass for "mass", uom = UomMass {
        /// Microgram (`µg`).
        Microgram => { symbol: "µg"; definition: crate::consts::mass::MICROGRAM; aliases: ["ug", "μg"]; uom: microgram; }
        /// Milligram (`mg`).
        Milligram => { symbol: "mg"; definition: crate::consts::mass::MILLIGRAM; uom: milligram; }
        /// Gram (`g`).
        Gram => { symbol: "g"; definition: crate::consts::mass::GRAM; uom: gram; }
        /// Kilogram (`kg`).
        Kilogram => { symbol: "kg"; definition: crate::consts::mass::KILOGRAM; uom: kilogram; }
        /// Metric tonne (`t`).
        Tonne => { symbol: "t"; definition: crate::consts::mass::TONNE; uom: ton; }
        /// Carat (`ct`).
        Carat => { symbol: "ct"; definition: crate::consts::mass::CARAT; uom: carat; }
        /// Ounce (`oz`).
        Ounce => { symbol: "oz"; definition: crate::consts::mass::OUNCE; uom: ounce; }
        /// Pound (`lb`).
        Pound => { symbol: "lb"; definition: crate::consts::mass::POUND; uom: pound; }
        /// Short ton (`2000 lb`).
        TonShort => { symbol: "2000 lb"; definition: crate::consts::mass::TON_SHORT; uom: ton_short; }
        /// Long ton (`2240 lb`).
        TonLong => { symbol: "2240 lb"; definition: crate::consts::mass::TON_LONG; uom: ton_long; }
    }
}
