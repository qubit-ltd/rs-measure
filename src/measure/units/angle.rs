/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Units for persisted angle measurements.

use super::define_measurement_unit;
use uom::si::angle::{
    degree,
    minute,
    radian,
    revolution,
    second,
};
use uom::si::f64::Angle as UomAngle;

define_measurement_unit! {
    /// Units for persisted `uom` angle quantities.
    pub enum Angle for UomAngle, "angle" {
        /// Radian (`rad`).
        Radian => "rad", radian;
        /// Degree (`°`).
        Degree => "°" | "deg", degree;
        /// Revolution (`r`).
        Revolution => "r" | "rev", revolution;
        /// Arcminute (`′`).
        Minute => "′" | "arcmin", minute;
        /// Arcsecond (`″`).
        Second => "″" | "arcsec", second;
    }
}
