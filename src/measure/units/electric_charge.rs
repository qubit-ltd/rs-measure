// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted electric charge measurements.

use crate::define_unit_family;
use uom::si::electric_charge::{
    ampere_hour,
    coulomb,
    kilocoulomb,
    microcoulomb,
    milliampere_hour,
    millicoulomb,
};
use uom::si::f64::ElectricCharge as UomElectricCharge;

define_unit_family! {
    /// Units for persisted `uom` electric charge quantities.
    pub enum ElectricCharge for "electric_charge", uom = UomElectricCharge {
        /// Microcoulomb (`µC`).
        Microcoulomb => { symbol: "µC"; definition: crate::consts::electric_charge::MICROCOULOMB; aliases: ["uC", "μC"]; uom: microcoulomb; }
        /// Millicoulomb (`mC`).
        Millicoulomb => { symbol: "mC"; definition: crate::consts::electric_charge::MILLICOULOMB; uom: millicoulomb; }
        /// Coulomb (`C`).
        Coulomb => { symbol: "C"; definition: crate::consts::electric_charge::COULOMB; uom: coulomb; }
        /// Kilocoulomb (`kC`).
        Kilocoulomb => { symbol: "kC"; definition: crate::consts::electric_charge::KILOCOULOMB; uom: kilocoulomb; }
        /// Ampere hour (`A · h`).
        AmpereHour => { symbol: "A · h"; definition: crate::consts::electric_charge::AMPERE_HOUR; aliases: ["Ah", "A h"]; uom: ampere_hour; }
        /// Milliampere hour (`mA · h`).
        MilliampereHour => { symbol: "mA · h"; definition: crate::consts::electric_charge::MILLIAMPERE_HOUR; aliases: ["mAh", "mA h"]; uom: milliampere_hour; }
    }
}
