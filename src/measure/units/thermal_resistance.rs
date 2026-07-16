// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted thermal resistance measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use uom::si::f64::ThermalResistance as UomThermalResistance;
#[cfg(feature = "uom")]
use uom::si::thermal_resistance::{
    kelvin_per_kilowatt,
    kelvin_per_milliwatt,
    kelvin_per_watt,
};

define_unit_family! {
    /// Units for persisted thermal resistance measurements.
    pub enum ThermalResistance for "thermal_resistance", uom = UomThermalResistance {
        /// Kelvin per milliwatt (`K/mW`).
        KelvinPerMilliwatt => { symbol: "K/mW"; definition: crate::consts::thermal_resistance::KELVIN_PER_MILLIWATT; uom: kelvin_per_milliwatt; }
        /// Kelvin per watt (`K/W`).
        KelvinPerWatt => { symbol: "K/W"; definition: crate::consts::thermal_resistance::KELVIN_PER_WATT; uom: kelvin_per_watt; }
        /// Kelvin per kilowatt (`K/kW`).
        KelvinPerKilowatt => { symbol: "K/kW"; definition: crate::consts::thermal_resistance::KELVIN_PER_KILOWATT; aliases: ["K/kw"]; uom: kelvin_per_kilowatt; }
    }
}
