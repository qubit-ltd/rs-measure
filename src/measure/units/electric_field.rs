// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted electric field measurements.

use crate::define_unit_family;
use uom::si::electric_field::{
    kilovolt_per_millimeter,
    megavolt_per_meter,
    volt_per_centimeter,
    volt_per_meter,
    volt_per_micrometer,
    volt_per_millimeter,
};
use uom::si::f64::ElectricField as UomElectricField;

define_unit_family! {
    /// Units for persisted `uom` electric field quantities.
    pub enum ElectricField for "electric_field", uom = UomElectricField {
        /// Volt per meter (`V/m`).
        VoltPerMeter => { symbol: "V/m"; coefficient: 1; uom: volt_per_meter; }
        /// Volt per centimeter (`V/cm`).
        VoltPerCentimeter => { symbol: "V/cm"; coefficient: 100; uom: volt_per_centimeter; }
        /// Volt per millimeter (`V/mm`).
        VoltPerMillimeter => { symbol: "V/mm"; coefficient: 1000; uom: volt_per_millimeter; }
        /// Volt per micrometer (`V/µm`).
        VoltPerMicrometer => { symbol: "V/µm"; coefficient: 1000000; aliases: ["V/um", "V/μm"]; uom: volt_per_micrometer; }
        /// Kilovolt per millimeter (`kV/mm`).
        KilovoltPerMillimeter => { symbol: "kV/mm"; coefficient: 1000000; uom: kilovolt_per_millimeter; }
        /// Megavolt per meter (`MV/m`).
        MegavoltPerMeter => { symbol: "MV/m"; coefficient: 1000000; uom: megavolt_per_meter; }
    }
}
