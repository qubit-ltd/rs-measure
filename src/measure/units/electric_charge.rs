/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Units for persisted electric charge measurements.

use super::define_measurement_unit;
use uom::si::electric_charge::{
    ampere_hour,
    coulomb,
    kilocoulomb,
    microcoulomb,
    milliampere_hour,
    millicoulomb,
};
use uom::si::f64::ElectricCharge as UomElectricCharge;

define_measurement_unit! {
    /// Units for persisted `uom` electric charge quantities.
    pub enum ElectricCharge for UomElectricCharge, "electric charge" {
        /// Microcoulomb (`µC`).
        Microcoulomb => "µC" | "uC" | "μC", microcoulomb;
        /// Millicoulomb (`mC`).
        Millicoulomb => "mC", millicoulomb;
        /// Coulomb (`C`).
        Coulomb => "C", coulomb;
        /// Kilocoulomb (`kC`).
        Kilocoulomb => "kC", kilocoulomb;
        /// Ampere hour (`A · h`).
        AmpereHour => "A · h" | "Ah" | "A h", ampere_hour;
        /// Milliampere hour (`mA · h`).
        MilliampereHour => "mA · h" | "mAh" | "mA h", milliampere_hour;
    }
}
