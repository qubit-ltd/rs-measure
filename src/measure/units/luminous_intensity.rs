// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted luminous intensity measurements.

use super::define_measurement_unit;
use uom::si::f64::LuminousIntensity as UomLuminousIntensity;
use uom::si::luminous_intensity::{
    candela,
    kilocandela,
    millicandela,
};

define_measurement_unit! {
    /// Units for persisted `uom` luminous intensity quantities.
    pub enum LuminousIntensity for UomLuminousIntensity, "luminous intensity" {
        /// Millicandela (`mcd`).
        Millicandela => "mcd", millicandela;
        /// Candela (`cd`).
        Candela => "cd", candela;
        /// Kilocandela (`kcd`).
        Kilocandela => "kcd", kilocandela;
    }
}
