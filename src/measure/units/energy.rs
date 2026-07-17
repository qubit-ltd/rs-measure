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
use crate::impl_uom_unit;
#[cfg(feature = "uom")]
use uom::si::energy::joule;
#[cfg(feature = "uom")]
use uom::si::f64::Energy as UomEnergy;

define_unit_family! {
    /// Units for persisted energy measurements.
    pub enum Energy for "energy" {
        /// Joule (`J`).
        Joule => { symbol: "J"; definition: crate::consts::energy::JOULE; }
        /// Kilojoule (`kJ`).
        Kilojoule => { symbol: "kJ"; definition: crate::consts::energy::KILOJOULE; }
        /// Megajoule (`MJ`).
        Megajoule => { symbol: "MJ"; definition: crate::consts::energy::MEGAJOULE; }
        /// Watt hour (`W · h`).
        WattHour => { symbol: "W · h"; definition: crate::consts::energy::WATT_HOUR; aliases: ["Wh"]; }
        /// Kilowatt hour (`kW · h`).
        KilowattHour => { symbol: "kW · h"; definition: crate::consts::energy::KILOWATT_HOUR; aliases: ["kWh"]; }
        /// Electronvolt (`eV`).
        Electronvolt => { symbol: "eV"; definition: crate::consts::energy::ELECTRONVOLT; }
        /// Thermochemical calorie with canonical symbol `cal (th)`.
        ThermochemicalCalorie => { symbol: "cal (th)"; definition: crate::consts::energy::THERMOCHEMICAL_CALORIE; aliases: ["cal"]; }
        /// Thermochemical kilocalorie with canonical symbol `kcal (th)`.
        ThermochemicalKilocalorie => { symbol: "kcal (th)"; definition: crate::consts::energy::THERMOCHEMICAL_KILOCALORIE; aliases: ["kcal"]; }
        /// International Table British thermal unit with canonical symbol
        /// `Btu (IT)`.
        BritishThermalUnitInternationalTable => { symbol: "Btu (IT)"; definition: crate::consts::energy::BRITISH_THERMAL_UNIT_INTERNATIONAL_TABLE; aliases: ["Btu", "BTU"]; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    Energy, UomEnergy {
        base: joule;
    }
}
