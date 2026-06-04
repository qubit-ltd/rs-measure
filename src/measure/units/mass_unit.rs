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
use uom::si::f64::Mass;
use uom::si::mass::{
    gram,
    kilogram,
    milligram,
    ounce,
    pound,
    ton,
};

define_measurement_unit! {
    /// Units for persisted `uom` mass quantities.
    pub enum MassUnit for Mass, "mass" {
        /// Milligram (`mg`).
        Milligram => "mg", milligram;
        /// Gram (`g`).
        Gram => "g", gram;
        /// Kilogram (`kg`).
        Kilogram => "kg", kilogram;
        /// Metric tonne (`t`).
        Tonne => "t", ton;
        /// Ounce (`oz`).
        Ounce => "oz", ounce;
        /// Pound (`lb`).
        Pound => "lb", pound;
    }
}
