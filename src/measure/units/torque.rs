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
use crate::impl_uom_unit;
#[cfg(feature = "uom")]
use uom::si::f64::Torque as UomTorque;
#[cfg(feature = "uom")]
use uom::si::torque::newton_meter;

define_unit_family! {
    /// Units for persisted torque measurements.
    pub enum Torque for "torque" {
        /// Millinewton meter (`mN · m`).
        MillinewtonMeter => { symbol: "mN · m"; definition: crate::consts::torque::MILLINEWTON_METER; aliases: ["mN m", "mN*m"]; }
        /// Newton meter (`N · m`).
        NewtonMeter => { symbol: "N · m"; definition: crate::consts::torque::NEWTON_METER; aliases: ["N m", "N*m", "Nm"]; }
        /// Kilonewton meter (`kN · m`).
        KilonewtonMeter => { symbol: "kN · m"; definition: crate::consts::torque::KILONEWTON_METER; aliases: ["kN m", "kN*m", "kNm"]; }
        /// Pound-force foot (`lbf · ft`).
        PoundForceFoot => { symbol: "lbf · ft"; definition: crate::consts::torque::POUND_FORCE_FOOT; aliases: ["lbf ft"]; }
        /// Pound-force inch (`lbf · in`).
        PoundForceInch => { symbol: "lbf · in"; definition: crate::consts::torque::POUND_FORCE_INCH; aliases: ["lbf in"]; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    Torque, UomTorque {
        base: newton_meter;
    }
}
