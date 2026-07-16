// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted energy measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use uom::si::energy::{
    btu_it,
    calorie,
    electronvolt,
    joule,
    kilocalorie,
    kilojoule,
    kilowatt_hour,
    megajoule,
    watt_hour,
};
#[cfg(feature = "uom")]
use uom::si::f64::Energy as UomEnergy;

define_unit_family! {
    /// Units for persisted energy measurements.
    pub enum Energy for "energy", uom = UomEnergy {
        /// Joule (`J`).
        Joule => { symbol: "J"; definition: crate::consts::energy::JOULE; uom: joule; }
        /// Kilojoule (`kJ`).
        Kilojoule => { symbol: "kJ"; definition: crate::consts::energy::KILOJOULE; uom: kilojoule; }
        /// Megajoule (`MJ`).
        Megajoule => { symbol: "MJ"; definition: crate::consts::energy::MEGAJOULE; uom: megajoule; }
        /// Watt hour (`W · h`).
        WattHour => { symbol: "W · h"; definition: crate::consts::energy::WATT_HOUR; aliases: ["Wh"]; uom: watt_hour; }
        /// Kilowatt hour (`kW · h`).
        KilowattHour => { symbol: "kW · h"; definition: crate::consts::energy::KILOWATT_HOUR; aliases: ["kWh"]; uom: kilowatt_hour; }
        /// Electronvolt (`eV`).
        Electronvolt => { symbol: "eV"; definition: crate::consts::energy::ELECTRONVOLT; uom: electronvolt; }
        /// Thermochemical calorie with canonical symbol `cal (th)`.
        ThermochemicalCalorie => { symbol: "cal (th)"; definition: crate::consts::energy::THERMOCHEMICAL_CALORIE; aliases: ["cal"]; uom: calorie; }
        /// Thermochemical kilocalorie with canonical symbol `kcal (th)`.
        ThermochemicalKilocalorie => { symbol: "kcal (th)"; definition: crate::consts::energy::THERMOCHEMICAL_KILOCALORIE; aliases: ["kcal"]; uom: kilocalorie; }
        /// International Table British thermal unit with canonical symbol
        /// `Btu (IT)`.
        BritishThermalUnitInternationalTable => { symbol: "Btu (IT)"; definition: crate::consts::energy::BRITISH_THERMAL_UNIT_INTERNATIONAL_TABLE; aliases: ["Btu", "BTU"]; uom: btu_it; }
    }
}
