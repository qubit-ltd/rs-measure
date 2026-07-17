// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted power measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;
#[cfg(feature = "uom")]
use uom::si::f64::Power as UomPower;
#[cfg(feature = "uom")]
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
    /// Units for persisted power measurements.
    pub enum Power for "power" {
        /// Nanowatt (`nW`).
        Nanowatt => { symbol: "nW"; definition: crate::consts::power::NANOWATT; }
        /// Microwatt (`µW`).
        Microwatt => { symbol: "µW"; definition: crate::consts::power::MICROWATT; aliases: ["uW", "μW"]; }
        /// Milliwatt (`mW`).
        Milliwatt => { symbol: "mW"; definition: crate::consts::power::MILLIWATT; }
        /// Watt (`W`).
        Watt => { symbol: "W"; definition: crate::consts::power::WATT; }
        /// Kilowatt (`kW`).
        Kilowatt => { symbol: "kW"; definition: crate::consts::power::KILOWATT; }
        /// Megawatt (`MW`).
        Megawatt => { symbol: "MW"; definition: crate::consts::power::MEGAWATT; }
        /// Mechanical horsepower with canonical symbol `hp (mechanical)`.
        MechanicalHorsepower => { symbol: "hp (mechanical)"; definition: crate::consts::power::MECHANICAL_HORSEPOWER; aliases: ["hp"]; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    Power, UomPower {
        Nanowatt => nanowatt;
        Microwatt => microwatt;
        Milliwatt => milliwatt;
        Watt => watt;
        Kilowatt => kilowatt;
        Megawatt => megawatt;
        MechanicalHorsepower => horsepower;
    }
}
