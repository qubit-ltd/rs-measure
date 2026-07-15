// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted power measurements.

use crate::define_unit_family;
use uom::si::f64::Power as UomPower;
use uom::si::power::{
    horsepower,
    kilowatt,
    megawatt,
    microwatt,
    milliwatt,
    nanowatt,
    watt,
};

define_unit_family! {
    /// Units for persisted `uom` power quantities.
    pub enum Power for "power", uom = UomPower {
        /// Nanowatt (`nW`).
        Nanowatt => { symbol: "nW"; coefficient: 1 / 1000000000; uom: nanowatt; }
        /// Microwatt (`µW`).
        Microwatt => { symbol: "µW"; coefficient: 1 / 1000000; aliases: ["uW", "μW"]; uom: microwatt; }
        /// Milliwatt (`mW`).
        Milliwatt => { symbol: "mW"; coefficient: 1 / 1000; uom: milliwatt; }
        /// Watt (`W`).
        Watt => { symbol: "W"; coefficient: 1; uom: watt; }
        /// Kilowatt (`kW`).
        Kilowatt => { symbol: "kW"; coefficient: 1000; uom: kilowatt; }
        /// Megawatt (`MW`).
        Megawatt => { symbol: "MW"; coefficient: 1000000; uom: megawatt; }
        /// Horsepower (`hp`).
        MechanicalHorsepower => { symbol: "hp (mechanical)"; coefficient: 37284993579113511 / 50000000000000; aliases: ["hp"]; uom: horsepower; }
    }
}
