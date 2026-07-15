// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted thermal resistance measurements.

use crate::define_unit_family;
use uom::si::f64::ThermalResistance as UomThermalResistance;
use uom::si::thermal_resistance::{
    kelvin_per_kilowatt,
    kelvin_per_milliwatt,
    kelvin_per_watt,
};

define_unit_family! {
    /// Units for persisted `uom` thermal resistance quantities.
    pub enum ThermalResistance for "thermal_resistance", uom = UomThermalResistance {
        /// Kelvin per milliwatt (`K/mW`).
        KelvinPerMilliwatt => { symbol: "K/mW"; coefficient: 1000; uom: kelvin_per_milliwatt; }
        /// Kelvin per watt (`K/W`).
        KelvinPerWatt => { symbol: "K/W"; coefficient: 1; uom: kelvin_per_watt; }
        /// Kelvin per kilowatt (`K/kW`).
        KelvinPerKilowatt => { symbol: "K/kW"; coefficient: 1 / 1000; aliases: ["K/kw"]; uom: kelvin_per_kilowatt; }
    }
}
