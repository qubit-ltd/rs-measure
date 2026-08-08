// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted electric field measurements.

#[cfg(feature = "uom")]
use uom::si::electric_field::volt_per_meter;
#[cfg(feature = "uom")]
use uom::si::f64::ElectricField as UomElectricField;

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;

define_unit_family! {
    /// Units for persisted electric field measurements.
    pub enum ElectricField for "electric_field" {
        /// Volt per meter (`V/m`).
        VoltPerMeter => { symbol: "V/m"; definition: crate::consts::electric_field::VOLT_PER_METER; }
        /// Volt per centimeter (`V/cm`).
        VoltPerCentimeter => { symbol: "V/cm"; definition: crate::consts::electric_field::VOLT_PER_CENTIMETER; }
        /// Volt per millimeter (`V/mm`).
        VoltPerMillimeter => { symbol: "V/mm"; definition: crate::consts::electric_field::VOLT_PER_MILLIMETER; }
        /// Volt per micrometer (`V/µm`).
        VoltPerMicrometer => { symbol: "V/µm"; definition: crate::consts::electric_field::VOLT_PER_MICROMETER; aliases: ["V/um", "V/μm"]; }
        /// Kilovolt per millimeter (`kV/mm`).
        KilovoltPerMillimeter => { symbol: "kV/mm"; definition: crate::consts::electric_field::KILOVOLT_PER_MILLIMETER; }
        /// Megavolt per meter (`MV/m`).
        MegavoltPerMeter => { symbol: "MV/m"; definition: crate::consts::electric_field::MEGAVOLT_PER_METER; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    ElectricField, UomElectricField {
        base: volt_per_meter;
    }
}
