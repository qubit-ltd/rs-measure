// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted torque measurements.

use crate::define_unit_family;
use uom::si::f64::Torque as UomTorque;
use uom::si::torque::{
    kilonewton_meter,
    millinewton_meter,
    newton_meter,
    pound_force_foot,
    pound_force_inch,
};

define_unit_family! {
    /// Units for persisted `uom` torque quantities.
    pub enum Torque for "torque", uom = UomTorque {
        /// Millinewton meter (`mN · m`).
        MillinewtonMeter => { symbol: "mN · m"; coefficient: 1 / 1000; aliases: ["mN m", "mN*m"]; uom: millinewton_meter; }
        /// Newton meter (`N · m`).
        NewtonMeter => { symbol: "N · m"; coefficient: 1; aliases: ["N m", "N*m", "Nm"]; uom: newton_meter; }
        /// Kilonewton meter (`kN · m`).
        KilonewtonMeter => { symbol: "kN · m"; coefficient: 1000; aliases: ["kN m", "kN*m", "kNm"]; uom: kilonewton_meter; }
        /// Pound-force foot (`lbf · ft`).
        PoundForceFoot => { symbol: "lbf · ft"; coefficient: 3389544870828501 / 2500000000000000; aliases: ["lbf ft"]; uom: pound_force_foot; }
        /// Pound-force inch (`lbf · in`).
        PoundForceInch => { symbol: "lbf · in"; coefficient: 1129848290276167 / 10000000000000000; aliases: ["lbf in"]; uom: pound_force_inch; }
    }
}
