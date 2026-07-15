// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted surface tension measurements.

use crate::define_unit_family;
use uom::si::f64::SurfaceTension as UomSurfaceTension;
use uom::si::surface_tension::{
    dyne_per_centimeter,
    joule_per_square_meter,
    millinewton_per_meter,
    newton_per_meter,
};

define_unit_family! {
    /// Units for persisted `uom` surface tension quantities.
    pub enum SurfaceTension for "surface_tension", uom = UomSurfaceTension {
        /// Millinewton per meter (`mN/m`).
        MillinewtonPerMeter => { symbol: "mN/m"; coefficient: 1 / 1000; uom: millinewton_per_meter; }
        /// Newton per meter (`N/m`).
        NewtonPerMeter => { symbol: "N/m"; coefficient: 1; uom: newton_per_meter; }
        /// Dyne per centimeter (`dyn/cm`).
        DynePerCentimeter => { symbol: "dyn/cm"; coefficient: 1 / 1000; uom: dyne_per_centimeter; }
        /// Joule per square meter (`J/m²`).
        JoulePerSquareMeter => { symbol: "J/m²"; coefficient: 1; aliases: ["J/m2", "J/m^2"]; uom: joule_per_square_meter; }
    }
}
