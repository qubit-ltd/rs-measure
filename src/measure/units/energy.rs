/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Units for persisted energy measurements.

use super::define_measurement_unit;
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

define_measurement_unit! {
    /// Units for persisted `uom` energy quantities.
    pub enum Energy for UomEnergy, "energy" {
        /// Joule (`J`).
        Joule => "J", joule;
        /// Kilojoule (`kJ`).
        Kilojoule => "kJ", kilojoule;
        /// Megajoule (`MJ`).
        Megajoule => "MJ", megajoule;
        /// Watt hour (`W · h`).
        WattHour => "W · h" | "Wh", watt_hour;
        /// Kilowatt hour (`kW · h`).
        KilowattHour => "kW · h" | "kWh", kilowatt_hour;
        /// Electronvolt (`eV`).
        Electronvolt => "eV", electronvolt;
        /// Calorie (`cal`).
        Calorie => "cal", calorie;
        /// Kilocalorie (`kcal`).
        Kilocalorie => "kcal", kilocalorie;
        /// British thermal unit (`Btu`).
        BritishThermalUnit => "Btu", btu;
    }
}
