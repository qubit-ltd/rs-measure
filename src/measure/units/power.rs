/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Units for persisted power measurements.

use super::define_measurement_unit;
use uom::si::f64::Power as UomPower;
use uom::si::power::{
    horsepower,
    kilowatt,
    megawatt,
    watt,
};

define_measurement_unit! {
    /// Units for persisted `uom` power quantities.
    pub enum Power for UomPower, "power" {
        /// Watt (`W`).
        Watt => "W", watt;
        /// Kilowatt (`kW`).
        Kilowatt => "kW", kilowatt;
        /// Megawatt (`MW`).
        Megawatt => "MW", megawatt;
        /// Horsepower (`hp`).
        Horsepower => "hp", horsepower;
    }
}
