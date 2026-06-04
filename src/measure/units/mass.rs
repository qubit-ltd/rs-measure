/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Units for persisted mass measurements.

use super::define_measurement_unit;
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

define_measurement_unit! {
    /// Units for persisted `uom` mass quantities.
    pub enum Mass for UomMass, "mass" {
        /// Microgram (`µg`).
        Microgram => "µg" | "ug" | "μg", microgram;
        /// Milligram (`mg`).
        Milligram => "mg", milligram;
        /// Gram (`g`).
        Gram => "g", gram;
        /// Kilogram (`kg`).
        Kilogram => "kg", kilogram;
        /// Metric tonne (`t`).
        Tonne => "t", ton;
        /// Carat (`ct`).
        Carat => "ct", carat;
        /// Ounce (`oz`).
        Ounce => "oz", ounce;
        /// Pound (`lb`).
        Pound => "lb", pound;
        /// Short ton (`2000 lb`).
        TonShort => "2000 lb", ton_short;
        /// Long ton (`2240 lb`).
        TonLong => "2240 lb", ton_long;
    }
}
