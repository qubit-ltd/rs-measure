// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted mass measurements.

use crate::define_unit_family;
use uom::si::f64::Mass as UomMass;
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
    /// Units for persisted `uom` mass quantities.
    pub enum Mass for "mass", uom = UomMass {
        /// Microgram (`µg`).
        Microgram => { symbol: "µg"; coefficient: 1 / 1000000000; aliases: ["ug", "μg"]; uom: microgram; }
        /// Milligram (`mg`).
        Milligram => { symbol: "mg"; coefficient: 1 / 1000000; uom: milligram; }
        /// Gram (`g`).
        Gram => { symbol: "g"; coefficient: 1 / 1000; uom: gram; }
        /// Kilogram (`kg`).
        Kilogram => { symbol: "kg"; coefficient: 1; uom: kilogram; }
        /// Metric tonne (`t`).
        Tonne => { symbol: "t"; coefficient: 1000; uom: ton; }
        /// Carat (`ct`).
        Carat => { symbol: "ct"; coefficient: 1 / 5000; uom: carat; }
        /// Ounce (`oz`).
        Ounce => { symbol: "oz"; coefficient: 45359237 / 1600000000; uom: ounce; }
        /// Pound (`lb`).
        Pound => { symbol: "lb"; coefficient: 45359237 / 100000000; uom: pound; }
        /// Short ton (`2000 lb`).
        TonShort => { symbol: "2000 lb"; coefficient: 45359237 / 50000; uom: ton_short; }
        /// Long ton (`2240 lb`).
        TonLong => { symbol: "2240 lb"; coefficient: 317514659 / 312500; uom: ton_long; }
    }
}
