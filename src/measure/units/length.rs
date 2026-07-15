// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted length measurements.

use crate::define_unit_family;
use uom::si::f64::Length as UomLength;
use uom::si::length::{
    centimeter,
    foot,
    inch,
    kilometer,
    meter,
    micrometer,
    mile,
    millimeter,
    nanometer,
    yard,
};

define_unit_family! {
    /// Units for persisted `uom` length quantities.
    pub enum Length for "length", uom = UomLength {
        /// Nanometer (`nm`).
        Nanometer => { symbol: "nm"; definition: crate::consts::length::NANOMETER; uom: nanometer; }
        /// Micrometer (`µm`).
        Micrometer => { symbol: "µm"; definition: crate::consts::length::MICROMETER; aliases: ["um", "μm"]; uom: micrometer; }
        /// Millimeter (`mm`).
        Millimeter => { symbol: "mm"; definition: crate::consts::length::MILLIMETER; uom: millimeter; }
        /// Centimeter (`cm`).
        Centimeter => { symbol: "cm"; definition: crate::consts::length::CENTIMETER; uom: centimeter; }
        /// Meter (`m`).
        Meter => { symbol: "m"; definition: crate::consts::length::METER; uom: meter; }
        /// Kilometer (`km`).
        Kilometer => { symbol: "km"; definition: crate::consts::length::KILOMETER; uom: kilometer; }
        /// Inch (`in`).
        Inch => { symbol: "in"; definition: crate::consts::length::INCH; uom: inch; }
        /// Foot (`ft`).
        Foot => { symbol: "ft"; definition: crate::consts::length::FOOT; uom: foot; }
        /// Yard (`yd`).
        Yard => { symbol: "yd"; definition: crate::consts::length::YARD; uom: yard; }
        /// Mile (`mi`).
        Mile => { symbol: "mi"; definition: crate::consts::length::MILE; uom: mile; }
    }
}
