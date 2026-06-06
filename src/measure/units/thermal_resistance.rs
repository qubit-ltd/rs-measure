// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted thermal resistance measurements.

use super::define_measurement_unit;
use uom::si::f64::ThermalResistance as UomThermalResistance;
use uom::si::thermal_resistance::{
    kelvin_per_kilowatt,
    kelvin_per_milliwatt,
    kelvin_per_watt,
};

define_measurement_unit! {
    /// Units for persisted `uom` thermal resistance quantities.
    pub enum ThermalResistance for UomThermalResistance, "thermal resistance" {
        /// Kelvin per milliwatt (`K/mW`).
        KelvinPerMilliwatt => "K/mW", kelvin_per_milliwatt;
        /// Kelvin per watt (`K/W`).
        KelvinPerWatt => "K/W", kelvin_per_watt;
        /// Kelvin per kilowatt (`K/kW`).
        KelvinPerKilowatt => "K/kW" | "K/kw", kelvin_per_kilowatt;
    }
}
