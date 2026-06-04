/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Units for persisted surface tension measurements.

use super::define_measurement_unit;
use uom::si::f64::SurfaceTension as UomSurfaceTension;
use uom::si::surface_tension::{
    dyne_per_centimeter,
    joule_per_square_meter,
    millinewton_per_meter,
    newton_per_meter,
};

define_measurement_unit! {
    /// Units for persisted `uom` surface tension quantities.
    pub enum SurfaceTension for UomSurfaceTension, "surface tension" {
        /// Millinewton per meter (`mN/m`).
        MillinewtonPerMeter => "mN/m", millinewton_per_meter;
        /// Newton per meter (`N/m`).
        NewtonPerMeter => "N/m", newton_per_meter;
        /// Dyne per centimeter (`dyn/cm`).
        DynePerCentimeter => "dyn/cm", dyne_per_centimeter;
        /// Joule per square meter (`J/m²`).
        JoulePerSquareMeter => "J/m²" | "J/m2" | "J/m^2", joule_per_square_meter;
    }
}
