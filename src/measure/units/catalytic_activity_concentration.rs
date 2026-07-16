// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted catalytic activity concentration measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use uom::si::catalytic_activity_concentration::{
    enzyme_unit_per_liter,
    katal_per_cubic_meter,
    milli_enzyme_unit_per_milliliter,
};
#[cfg(feature = "uom")]
use uom::si::f64::CatalyticActivityConcentration as UomCatalyticActivityConcentration;

define_unit_family! {
    /// Units for persisted catalytic activity concentration measurements.
    pub enum CatalyticActivityConcentration for "catalytic_activity_concentration", uom = UomCatalyticActivityConcentration {
        /// Katal per cubic meter (`kat/m³`).
        KatalPerCubicMeter => { symbol: "kat/m³"; definition: crate::consts::catalytic_activity_concentration::KATAL_PER_CUBIC_METER; aliases: ["kat/m3", "kat/m^3"]; uom: katal_per_cubic_meter; }
        /// Enzyme unit per liter (`U/L`).
        EnzymeUnitPerLiter => { symbol: "U/L"; definition: crate::consts::catalytic_activity_concentration::ENZYME_UNIT_PER_LITER; uom: enzyme_unit_per_liter; }
        /// Milli enzyme unit per milliliter (`mU/mL`).
        MilliEnzymeUnitPerMilliliter => { symbol: "mU/mL"; definition: crate::consts::catalytic_activity_concentration::MILLI_ENZYME_UNIT_PER_MILLILITER; uom: milli_enzyme_unit_per_milliliter; }
    }
}
