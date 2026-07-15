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
        Nanometer => { symbol: "nm"; coefficient: 1 / 1000000000; uom: nanometer; }
        /// Micrometer (`µm`).
        Micrometer => { symbol: "µm"; coefficient: 1 / 1000000; aliases: ["um", "μm"]; uom: micrometer; }
        /// Millimeter (`mm`).
        Millimeter => { symbol: "mm"; coefficient: 1 / 1000; uom: millimeter; }
        /// Centimeter (`cm`).
        Centimeter => { symbol: "cm"; coefficient: 1 / 100; uom: centimeter; }
        /// Meter (`m`).
        Meter => { symbol: "m"; coefficient: 1; uom: meter; }
        /// Kilometer (`km`).
        Kilometer => { symbol: "km"; coefficient: 1000; uom: kilometer; }
        /// Inch (`in`).
        Inch => { symbol: "in"; coefficient: 127 / 5000; uom: inch; }
        /// Foot (`ft`).
        Foot => { symbol: "ft"; coefficient: 381 / 1250; uom: foot; }
        /// Yard (`yd`).
        Yard => { symbol: "yd"; coefficient: 1143 / 1250; uom: yard; }
        /// Mile (`mi`).
        Mile => { symbol: "mi"; coefficient: 201168 / 125; uom: mile; }
    }
}
