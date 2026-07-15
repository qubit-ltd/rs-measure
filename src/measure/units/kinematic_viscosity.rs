// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted kinematic viscosity measurements.

use crate::define_unit_family;
use uom::si::f64::KinematicViscosity as UomKinematicViscosity;
use uom::si::kinematic_viscosity::{
    centistokes,
    square_meter_per_second,
    square_millimeter_per_second,
    stokes,
};

define_unit_family! {
    /// Units for persisted `uom` kinematic viscosity quantities.
    pub enum KinematicViscosity for "kinematic_viscosity", uom = UomKinematicViscosity {
        /// Square millimeter per second (`mm²/s`).
        SquareMillimeterPerSecond => { symbol: "mm²/s"; coefficient: 1 / 1000000; aliases: ["mm2/s", "mm^2/s"]; uom: square_millimeter_per_second; }
        /// Square meter per second (`m²/s`).
        SquareMeterPerSecond => { symbol: "m²/s"; coefficient: 1; aliases: ["m2/s", "m^2/s"]; uom: square_meter_per_second; }
        /// Stokes (`St`).
        Stokes => { symbol: "St"; coefficient: 1 / 10000; uom: stokes; }
        /// Centistokes (`cSt`).
        Centistokes => { symbol: "cSt"; coefficient: 1 / 1000000; uom: centistokes; }
    }
}
