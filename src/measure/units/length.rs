// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted length measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;
#[cfg(feature = "uom")]
use uom::si::f64::Length as UomLength;
#[cfg(feature = "uom")]
use uom::si::length::meter;

define_unit_family! {
    /// Units for persisted length measurements.
    pub enum Length for "length" {
        /// Nanometer (`nm`).
        Nanometer => { symbol: "nm"; definition: crate::consts::length::NANOMETER; }
        /// Micrometer (`µm`).
        Micrometer => { symbol: "µm"; definition: crate::consts::length::MICROMETER; aliases: ["um", "μm"]; }
        /// Millimeter (`mm`).
        Millimeter => { symbol: "mm"; definition: crate::consts::length::MILLIMETER; }
        /// Centimeter (`cm`).
        Centimeter => { symbol: "cm"; definition: crate::consts::length::CENTIMETER; }
        /// Meter (`m`).
        Meter => { symbol: "m"; definition: crate::consts::length::METER; }
        /// Kilometer (`km`).
        Kilometer => { symbol: "km"; definition: crate::consts::length::KILOMETER; }
        /// Inch (`in`).
        Inch => { symbol: "in"; definition: crate::consts::length::INCH; }
        /// Foot (`ft`).
        Foot => { symbol: "ft"; definition: crate::consts::length::FOOT; }
        /// Yard (`yd`).
        Yard => { symbol: "yd"; definition: crate::consts::length::YARD; }
        /// Mile (`mi`).
        Mile => { symbol: "mi"; definition: crate::consts::length::MILE; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    Length, UomLength {
        base: meter;
    }
}
