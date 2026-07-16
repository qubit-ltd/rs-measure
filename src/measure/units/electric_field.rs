// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted electric field measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use uom::si::electric_field::{
    kilovolt_per_millimeter,
    megavolt_per_meter,
    volt_per_centimeter,
    volt_per_meter,
    volt_per_micrometer,
    volt_per_millimeter,
};
#[cfg(feature = "uom")]
use uom::si::f64::ElectricField as UomElectricField;

define_unit_family! {
    /// Units for persisted electric field measurements.
    pub enum ElectricField for "electric_field", uom = UomElectricField {
        /// Volt per meter (`V/m`).
        VoltPerMeter => { symbol: "V/m"; definition: crate::consts::electric_field::VOLT_PER_METER; uom: volt_per_meter; }
        /// Volt per centimeter (`V/cm`).
        VoltPerCentimeter => { symbol: "V/cm"; definition: crate::consts::electric_field::VOLT_PER_CENTIMETER; uom: volt_per_centimeter; }
        /// Volt per millimeter (`V/mm`).
        VoltPerMillimeter => { symbol: "V/mm"; definition: crate::consts::electric_field::VOLT_PER_MILLIMETER; uom: volt_per_millimeter; }
        /// Volt per micrometer (`V/µm`).
        VoltPerMicrometer => { symbol: "V/µm"; definition: crate::consts::electric_field::VOLT_PER_MICROMETER; aliases: ["V/um", "V/μm"]; uom: volt_per_micrometer; }
        /// Kilovolt per millimeter (`kV/mm`).
        KilovoltPerMillimeter => { symbol: "kV/mm"; definition: crate::consts::electric_field::KILOVOLT_PER_MILLIMETER; uom: kilovolt_per_millimeter; }
        /// Megavolt per meter (`MV/m`).
        MegavoltPerMeter => { symbol: "MV/m"; definition: crate::consts::electric_field::MEGAVOLT_PER_METER; uom: megavolt_per_meter; }
    }
}
