// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted luminous intensity measurements.

use crate::define_unit_family;
use uom::si::f64::LuminousIntensity as UomLuminousIntensity;
use uom::si::luminous_intensity::{
    candela,
    kilocandela,
    millicandela,
};

define_unit_family! {
    /// Units for persisted `uom` luminous intensity quantities.
    pub enum LuminousIntensity for "luminous_intensity", uom = UomLuminousIntensity {
        /// Millicandela (`mcd`).
        Millicandela => { symbol: "mcd"; coefficient: 1 / 1000; uom: millicandela; }
        /// Candela (`cd`).
        Candela => { symbol: "cd"; coefficient: 1; uom: candela; }
        /// Kilocandela (`kcd`).
        Kilocandela => { symbol: "kcd"; coefficient: 1000; uom: kilocandela; }
    }
}
