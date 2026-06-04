/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Units for persisted kinematic viscosity measurements.

use super::define_measurement_unit;
use uom::si::f64::KinematicViscosity as UomKinematicViscosity;
use uom::si::kinematic_viscosity::{
    centistokes,
    square_meter_per_second,
    square_millimeter_per_second,
    stokes,
};

define_measurement_unit! {
    /// Units for persisted `uom` kinematic viscosity quantities.
    pub enum KinematicViscosity for UomKinematicViscosity, "kinematic viscosity" {
        /// Square millimeter per second (`mm²/s`).
        SquareMillimeterPerSecond => "mm²/s" | "mm2/s" | "mm^2/s", square_millimeter_per_second;
        /// Square meter per second (`m²/s`).
        SquareMeterPerSecond => "m²/s" | "m2/s" | "m^2/s", square_meter_per_second;
        /// Stokes (`St`).
        Stokes => "St", stokes;
        /// Centistokes (`cSt`).
        Centistokes => "cSt", centistokes;
    }
}
