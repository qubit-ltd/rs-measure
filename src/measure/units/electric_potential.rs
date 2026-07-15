// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted electric potential measurements.

use crate::define_unit_family;
use uom::si::electric_potential::{
    kilovolt,
    megavolt,
    microvolt,
    millivolt,
    nanovolt,
    volt,
};
use uom::si::f64::ElectricPotential as UomElectricPotential;

define_unit_family! {
    /// Units for persisted `uom` electric potential quantities.
    ///
    /// Electric potential is the SI quantity commonly called voltage.
    pub enum ElectricPotential for "electric_potential", uom = UomElectricPotential {
        /// Nanovolt (`nV`).
        Nanovolt => { symbol: "nV"; coefficient: 1 / 1000000000; uom: nanovolt; }
        /// Microvolt (`µV`).
        Microvolt => { symbol: "µV"; coefficient: 1 / 1000000; aliases: ["uV", "μV"]; uom: microvolt; }
        /// Millivolt (`mV`).
        Millivolt => { symbol: "mV"; coefficient: 1 / 1000; uom: millivolt; }
        /// Volt (`V`).
        Volt => { symbol: "V"; coefficient: 1; aliases: ["volt"]; uom: volt; }
        /// Kilovolt (`kV`).
        Kilovolt => { symbol: "kV"; coefficient: 1000; uom: kilovolt; }
        /// Megavolt (`MV`).
        Megavolt => { symbol: "MV"; coefficient: 1000000; uom: megavolt; }
    }
}
