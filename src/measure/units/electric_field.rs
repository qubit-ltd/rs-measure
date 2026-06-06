// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted electric field measurements.

use super::define_measurement_unit;
use uom::si::electric_field::{
    kilovolt_per_millimeter,
    megavolt_per_meter,
    volt_per_centimeter,
    volt_per_meter,
    volt_per_micrometer,
    volt_per_millimeter,
};
use uom::si::f64::ElectricField as UomElectricField;

define_measurement_unit! {
    /// Units for persisted `uom` electric field quantities.
    pub enum ElectricField for UomElectricField, "electric field" {
        /// Volt per meter (`V/m`).
        VoltPerMeter => "V/m", volt_per_meter;
        /// Volt per centimeter (`V/cm`).
        VoltPerCentimeter => "V/cm", volt_per_centimeter;
        /// Volt per millimeter (`V/mm`).
        VoltPerMillimeter => "V/mm", volt_per_millimeter;
        /// Volt per micrometer (`V/µm`).
        VoltPerMicrometer => "V/µm" | "V/um" | "V/μm", volt_per_micrometer;
        /// Kilovolt per millimeter (`kV/mm`).
        KilovoltPerMillimeter => "kV/mm", kilovolt_per_millimeter;
        /// Megavolt per meter (`MV/m`).
        MegavoltPerMeter => "MV/m", megavolt_per_meter;
    }
}
