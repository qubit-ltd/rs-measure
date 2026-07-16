// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted surface tension measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use uom::si::f64::SurfaceTension as UomSurfaceTension;
#[cfg(feature = "uom")]
use uom::si::surface_tension::{
    dyne_per_centimeter,
    joule_per_square_meter,
    millinewton_per_meter,
    newton_per_meter,
};

define_unit_family! {
    /// Units for persisted surface tension measurements.
    pub enum SurfaceTension for "surface_tension", uom = UomSurfaceTension {
        /// Millinewton per meter (`mN/m`).
        MillinewtonPerMeter => { symbol: "mN/m"; definition: crate::consts::surface_tension::MILLINEWTON_PER_METER; uom: millinewton_per_meter; }
        /// Newton per meter (`N/m`).
        NewtonPerMeter => { symbol: "N/m"; definition: crate::consts::surface_tension::NEWTON_PER_METER; uom: newton_per_meter; }
        /// Dyne per centimeter (`dyn/cm`).
        DynePerCentimeter => { symbol: "dyn/cm"; definition: crate::consts::surface_tension::DYNE_PER_CENTIMETER; uom: dyne_per_centimeter; }
        /// Joule per square meter (`J/m²`).
        JoulePerSquareMeter => { symbol: "J/m²"; definition: crate::consts::surface_tension::JOULE_PER_SQUARE_METER; aliases: ["J/m2", "J/m^2"]; uom: joule_per_square_meter; }
    }
}
