// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted torque measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use uom::si::f64::Torque as UomTorque;
#[cfg(feature = "uom")]
use uom::si::torque::{
    kilonewton_meter,
    millinewton_meter,
    newton_meter,
    pound_force_foot,
    pound_force_inch,
};

define_unit_family! {
    /// Units for persisted torque measurements.
    pub enum Torque for "torque", uom = UomTorque {
        /// Millinewton meter (`mN · m`).
        MillinewtonMeter => { symbol: "mN · m"; definition: crate::consts::torque::MILLINEWTON_METER; aliases: ["mN m", "mN*m"]; uom: millinewton_meter; }
        /// Newton meter (`N · m`).
        NewtonMeter => { symbol: "N · m"; definition: crate::consts::torque::NEWTON_METER; aliases: ["N m", "N*m", "Nm"]; uom: newton_meter; }
        /// Kilonewton meter (`kN · m`).
        KilonewtonMeter => { symbol: "kN · m"; definition: crate::consts::torque::KILONEWTON_METER; aliases: ["kN m", "kN*m", "kNm"]; uom: kilonewton_meter; }
        /// Pound-force foot (`lbf · ft`).
        PoundForceFoot => { symbol: "lbf · ft"; definition: crate::consts::torque::POUND_FORCE_FOOT; aliases: ["lbf ft"]; uom: pound_force_foot; }
        /// Pound-force inch (`lbf · in`).
        PoundForceInch => { symbol: "lbf · in"; definition: crate::consts::torque::POUND_FORCE_INCH; aliases: ["lbf in"]; uom: pound_force_inch; }
    }
}
