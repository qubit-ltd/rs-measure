// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted luminous intensity measurements.

#[cfg(feature = "uom")]
use uom::si::f64::LuminousIntensity as UomLuminousIntensity;
#[cfg(feature = "uom")]
use uom::si::luminous_intensity::candela;

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;

define_unit_family! {
    /// Units for persisted luminous intensity measurements.
    pub enum LuminousIntensity for "luminous_intensity" {
        /// Millicandela (`mcd`).
        Millicandela => { symbol: "mcd"; definition: crate::consts::luminous_intensity::MILLICANDELA; }
        /// Candela (`cd`).
        Candela => { symbol: "cd"; definition: crate::consts::luminous_intensity::CANDELA; }
        /// Kilocandela (`kcd`).
        Kilocandela => { symbol: "kcd"; definition: crate::consts::luminous_intensity::KILOCANDELA; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    LuminousIntensity, UomLuminousIntensity {
        base: candela;
    }
}
