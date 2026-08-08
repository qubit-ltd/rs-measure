// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted electric charge measurements.

#[cfg(feature = "uom")]
use uom::si::electric_charge::coulomb;
#[cfg(feature = "uom")]
use uom::si::f64::ElectricCharge as UomElectricCharge;

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;

define_unit_family! {
    /// Units for persisted electric charge measurements.
    pub enum ElectricCharge for "electric_charge" {
        /// Microcoulomb (`µC`).
        Microcoulomb => { symbol: "µC"; definition: crate::consts::electric_charge::MICROCOULOMB; aliases: ["uC", "μC"]; }
        /// Millicoulomb (`mC`).
        Millicoulomb => { symbol: "mC"; definition: crate::consts::electric_charge::MILLICOULOMB; }
        /// Coulomb (`C`).
        Coulomb => { symbol: "C"; definition: crate::consts::electric_charge::COULOMB; }
        /// Kilocoulomb (`kC`).
        Kilocoulomb => { symbol: "kC"; definition: crate::consts::electric_charge::KILOCOULOMB; }
        /// Ampere hour (`A · h`).
        AmpereHour => { symbol: "A · h"; definition: crate::consts::electric_charge::AMPERE_HOUR; aliases: ["Ah", "A h"]; }
        /// Milliampere hour (`mA · h`).
        MilliampereHour => { symbol: "mA · h"; definition: crate::consts::electric_charge::MILLIAMPERE_HOUR; aliases: ["mAh", "mA h"]; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    ElectricCharge, UomElectricCharge {
        base: coulomb;
    }
}
