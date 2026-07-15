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
        Nanowatt => { symbol: "nW"; definition: crate::consts::power::NANOWATT; uom: nanowatt; }
        /// Microwatt (`µW`).
        Microwatt => { symbol: "µW"; definition: crate::consts::power::MICROWATT; aliases: ["uW", "μW"]; uom: microwatt; }
        /// Milliwatt (`mW`).
        Milliwatt => { symbol: "mW"; definition: crate::consts::power::MILLIWATT; uom: milliwatt; }
        /// Watt (`W`).
        Watt => { symbol: "W"; definition: crate::consts::power::WATT; uom: watt; }
        /// Kilowatt (`kW`).
        Kilowatt => { symbol: "kW"; definition: crate::consts::power::KILOWATT; uom: kilowatt; }
        /// Megawatt (`MW`).
        Megawatt => { symbol: "MW"; definition: crate::consts::power::MEGAWATT; uom: megawatt; }
        /// Horsepower (`hp`).
        MechanicalHorsepower => { symbol: "hp (mechanical)"; definition: crate::consts::power::MECHANICAL_HORSEPOWER; aliases: ["hp"]; uom: horsepower; }
    }
}
