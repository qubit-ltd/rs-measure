// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted length measurements.

use super::define_measurement_unit;
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

define_measurement_unit! {
    /// Units for persisted `uom` length quantities.
    pub enum Length for UomLength, "length" {
        /// Nanometer (`nm`).
        Nanometer => "nm", nanometer;
        /// Micrometer (`µm`).
        Micrometer => "µm" | "um" | "μm", micrometer;
        /// Millimeter (`mm`).
        Millimeter => "mm", millimeter;
        /// Centimeter (`cm`).
        Centimeter => "cm", centimeter;
        /// Meter (`m`).
        Meter => "m", meter;
        /// Kilometer (`km`).
        Kilometer => "km", kilometer;
        /// Inch (`in`).
        Inch => "in", inch;
        /// Foot (`ft`).
        Foot => "ft", foot;
        /// Yard (`yd`).
        Yard => "yd", yard;
        /// Mile (`mi`).
        Mile => "mi", mile;
    }
}
