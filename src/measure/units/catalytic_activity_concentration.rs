// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted catalytic activity concentration measurements.

use super::define_measurement_unit;
use uom::si::catalytic_activity_concentration::{
    enzyme_unit_per_liter,
    katal_per_cubic_meter,
    milli_enzyme_unit_per_milliliter,
};
use uom::si::f64::CatalyticActivityConcentration as UomCatalyticActivityConcentration;

define_measurement_unit! {
    /// Units for persisted `uom` catalytic activity concentration quantities.
    pub enum CatalyticActivityConcentration for UomCatalyticActivityConcentration, "catalytic activity concentration" {
        /// Katal per cubic meter (`kat/m³`).
        KatalPerCubicMeter => "kat/m³" | "kat/m3" | "kat/m^3", katal_per_cubic_meter;
        /// Enzyme unit per liter (`U/L`).
        EnzymeUnitPerLiter => "U/L", enzyme_unit_per_liter;
        /// Milli enzyme unit per milliliter (`mU/mL`).
        MilliEnzymeUnitPerMilliliter => "mU/mL", milli_enzyme_unit_per_milliliter;
    }
}
