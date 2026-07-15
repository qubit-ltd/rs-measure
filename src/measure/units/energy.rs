// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted energy measurements.

use crate::define_unit_family;
use uom::si::energy::{
    btu,
    calorie,
    electronvolt,
    joule,
    kilocalorie,
    kilojoule,
    kilowatt_hour,
    megajoule,
    watt_hour,
};
use uom::si::f64::Energy as UomEnergy;

define_unit_family! {
    /// Units for persisted `uom` energy quantities.
    pub enum Energy for "energy", uom = UomEnergy {
        /// Joule (`J`).
        Joule => { symbol: "J"; coefficient: 1; uom: joule; }
        /// Kilojoule (`kJ`).
        Kilojoule => { symbol: "kJ"; coefficient: 1000; uom: kilojoule; }
        /// Megajoule (`MJ`).
        Megajoule => { symbol: "MJ"; coefficient: 1000000; uom: megajoule; }
        /// Watt hour (`W · h`).
        WattHour => { symbol: "W · h"; coefficient: 3600; aliases: ["Wh"]; uom: watt_hour; }
        /// Kilowatt hour (`kW · h`).
        KilowattHour => { symbol: "kW · h"; coefficient: 3600000; aliases: ["kWh"]; uom: kilowatt_hour; }
        /// Electronvolt (`eV`).
        Electronvolt => { symbol: "eV"; coefficient: 801088317 / 5000000000000000000000000000; uom: electronvolt; }
        /// Calorie (`cal`).
        ThermochemicalCalorie => { symbol: "cal (th)"; coefficient: 523 / 125; aliases: ["cal"]; uom: calorie; }
        /// Kilocalorie (`kcal`).
        ThermochemicalKilocalorie => { symbol: "kcal (th)"; coefficient: 4184; aliases: ["kcal"]; uom: kilocalorie; }
        /// British thermal unit (`Btu`).
        BritishThermalUnitInternationalTable => { symbol: "Btu (IT)"; coefficient: 131882 / 125; aliases: ["Btu", "BTU"]; uom: btu; }
    }
}
