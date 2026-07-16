// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted kinematic viscosity measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use uom::si::f64::KinematicViscosity as UomKinematicViscosity;
#[cfg(feature = "uom")]
use uom::si::kinematic_viscosity::{
    centistokes,
    square_meter_per_second,
    square_millimeter_per_second,
    stokes,
};

define_unit_family! {
    /// Units for persisted kinematic viscosity measurements.
    pub enum KinematicViscosity for "kinematic_viscosity", uom = UomKinematicViscosity {
        /// Square millimeter per second (`mm²/s`).
        SquareMillimeterPerSecond => { symbol: "mm²/s"; definition: crate::consts::kinematic_viscosity::SQUARE_MILLIMETER_PER_SECOND; aliases: ["mm2/s", "mm^2/s"]; uom: square_millimeter_per_second; }
        /// Square meter per second (`m²/s`).
        SquareMeterPerSecond => { symbol: "m²/s"; definition: crate::consts::kinematic_viscosity::SQUARE_METER_PER_SECOND; aliases: ["m2/s", "m^2/s"]; uom: square_meter_per_second; }
        /// Stokes (`St`).
        Stokes => { symbol: "St"; definition: crate::consts::kinematic_viscosity::STOKES; uom: stokes; }
        /// Centistokes (`cSt`).
        Centistokes => { symbol: "cSt"; definition: crate::consts::kinematic_viscosity::CENTISTOKES; uom: centistokes; }
    }
}
