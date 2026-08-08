// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted electric potential measurements.

#[cfg(feature = "uom")]
use uom::si::electric_potential::volt;
#[cfg(feature = "uom")]
use uom::si::f64::ElectricPotential as UomElectricPotential;

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;

define_unit_family! {
    /// Units for persisted electric potential measurements.
    ///
    /// Electric potential is the SI quantity commonly called voltage.
    pub enum ElectricPotential for "electric_potential" {
        /// Nanovolt (`nV`).
        Nanovolt => { symbol: "nV"; definition: crate::consts::electric_potential::NANOVOLT; }
        /// Microvolt (`µV`).
        Microvolt => { symbol: "µV"; definition: crate::consts::electric_potential::MICROVOLT; aliases: ["uV", "μV"]; }
        /// Millivolt (`mV`).
        Millivolt => { symbol: "mV"; definition: crate::consts::electric_potential::MILLIVOLT; }
        /// Volt (`V`).
        Volt => { symbol: "V"; definition: crate::consts::electric_potential::VOLT; aliases: ["volt"]; }
        /// Kilovolt (`kV`).
        Kilovolt => { symbol: "kV"; definition: crate::consts::electric_potential::KILOVOLT; }
        /// Megavolt (`MV`).
        Megavolt => { symbol: "MV"; definition: crate::consts::electric_potential::MEGAVOLT; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    ElectricPotential, UomElectricPotential {
        base: volt;
    }
}
