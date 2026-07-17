// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted frequency measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;
#[cfg(feature = "uom")]
use uom::si::f64::Frequency as UomFrequency;
#[cfg(feature = "uom")]
use uom::si::frequency::{
    gigahertz,
    hertz,
    kilohertz,
    megahertz,
};

define_unit_family! {
    /// Units for persisted frequency measurements.
    pub enum Frequency for "frequency" {
        /// Hertz (`Hz`).
        Hertz => { symbol: "Hz"; definition: crate::consts::frequency::HERTZ; }
        /// Kilohertz (`kHz`).
        Kilohertz => { symbol: "kHz"; definition: crate::consts::frequency::KILOHERTZ; }
        /// Megahertz (`MHz`).
        Megahertz => { symbol: "MHz"; definition: crate::consts::frequency::MEGAHERTZ; }
        /// Gigahertz (`GHz`).
        Gigahertz => { symbol: "GHz"; definition: crate::consts::frequency::GIGAHERTZ; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    Frequency, UomFrequency {
        Hertz => hertz;
        Kilohertz => kilohertz;
        Megahertz => megahertz;
        Gigahertz => gigahertz;
    }
}
