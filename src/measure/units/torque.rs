// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted torque measurements.

use super::define_measurement_unit;
use uom::si::f64::Torque as UomTorque;
use uom::si::torque::{
    kilonewton_meter,
    millinewton_meter,
    newton_meter,
    pound_force_foot,
    pound_force_inch,
};

define_measurement_unit! {
    /// Units for persisted `uom` torque quantities.
    pub enum Torque for UomTorque, "torque" {
        /// Millinewton meter (`mN · m`).
        MillinewtonMeter => "mN · m" | "mN m" | "mN*m", millinewton_meter;
        /// Newton meter (`N · m`).
        NewtonMeter => "N · m" | "N m" | "N*m" | "Nm", newton_meter;
        /// Kilonewton meter (`kN · m`).
        KilonewtonMeter => "kN · m" | "kN m" | "kN*m" | "kNm", kilonewton_meter;
        /// Pound-force foot (`lbf · ft`).
        PoundForceFoot => "lbf · ft" | "lbf ft", pound_force_foot;
        /// Pound-force inch (`lbf · in`).
        PoundForceInch => "lbf · in" | "lbf in", pound_force_inch;
    }
}
