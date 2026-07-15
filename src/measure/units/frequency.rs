// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted frequency measurements.

use crate::define_unit_family;
use uom::si::f64::Frequency as UomFrequency;
use uom::si::frequency::{
    gigahertz,
    hertz,
    kilohertz,
    megahertz,
};

define_unit_family! {
    /// Units for persisted `uom` frequency quantities.
    pub enum Frequency for "frequency", uom = UomFrequency {
        /// Hertz (`Hz`).
        Hertz => { symbol: "Hz"; definition: crate::consts::frequency::HERTZ; uom: hertz; }
        /// Kilohertz (`kHz`).
        Kilohertz => { symbol: "kHz"; definition: crate::consts::frequency::KILOHERTZ; uom: kilohertz; }
        /// Megahertz (`MHz`).
        Megahertz => { symbol: "MHz"; definition: crate::consts::frequency::MEGAHERTZ; uom: megahertz; }
        /// Gigahertz (`GHz`).
        Gigahertz => { symbol: "GHz"; definition: crate::consts::frequency::GIGAHERTZ; uom: gigahertz; }
    }
}
