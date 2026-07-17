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
use crate::impl_uom_unit;
#[cfg(feature = "uom")]
use uom::si::f64::KinematicViscosity as UomKinematicViscosity;
#[cfg(feature = "uom")]
use uom::si::kinematic_viscosity::square_meter_per_second;

define_unit_family! {
    /// Units for persisted kinematic viscosity measurements.
    pub enum KinematicViscosity for "kinematic_viscosity" {
        /// Square millimeter per second (`mm²/s`).
        SquareMillimeterPerSecond => { symbol: "mm²/s"; definition: crate::consts::kinematic_viscosity::SQUARE_MILLIMETER_PER_SECOND; aliases: ["mm2/s", "mm^2/s"]; }
        /// Square meter per second (`m²/s`).
        SquareMeterPerSecond => { symbol: "m²/s"; definition: crate::consts::kinematic_viscosity::SQUARE_METER_PER_SECOND; aliases: ["m2/s", "m^2/s"]; }
        /// Stokes (`St`).
        Stokes => { symbol: "St"; definition: crate::consts::kinematic_viscosity::STOKES; }
        /// Centistokes (`cSt`).
        Centistokes => { symbol: "cSt"; definition: crate::consts::kinematic_viscosity::CENTISTOKES; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    KinematicViscosity, UomKinematicViscosity {
        base: square_meter_per_second;
    }
}
