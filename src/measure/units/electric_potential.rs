// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted electric potential measurements.

use super::define_measurement_unit;
use uom::si::electric_potential::{
    kilovolt,
    megavolt,
    microvolt,
    millivolt,
    nanovolt,
    volt,
};
use uom::si::f64::ElectricPotential as UomElectricPotential;

define_measurement_unit! {
    /// Units for persisted `uom` electric potential quantities.
    ///
    /// Electric potential is the SI quantity commonly called voltage.
    pub enum ElectricPotential for UomElectricPotential, "electric potential" {
        /// Nanovolt (`nV`).
        Nanovolt => "nV", nanovolt;
        /// Microvolt (`µV`).
        Microvolt => "µV" | "uV" | "μV", microvolt;
        /// Millivolt (`mV`).
        Millivolt => "mV", millivolt;
        /// Volt (`V`).
        Volt => "V" | "volt", volt;
        /// Kilovolt (`kV`).
        Kilovolt => "kV", kilovolt;
        /// Megavolt (`MV`).
        Megavolt => "MV", megavolt;
    }
}
