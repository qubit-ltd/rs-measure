// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted catalytic activity measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;
#[cfg(feature = "uom")]
use uom::si::catalytic_activity::{
    enzyme_unit,
    katal,
    microkatal,
    milli_enzyme_unit,
    millikatal,
};
#[cfg(feature = "uom")]
use uom::si::f64::CatalyticActivity as UomCatalyticActivity;

define_unit_family! {
    /// Units for persisted catalytic activity measurements.
    pub enum CatalyticActivity for "catalytic_activity" {
        /// Microkatal (`µkat`).
        Microkatal => { symbol: "µkat"; definition: crate::consts::catalytic_activity::MICROKATAL; aliases: ["ukat", "μkat"]; }
        /// Millikatal (`mkat`).
        Millikatal => { symbol: "mkat"; definition: crate::consts::catalytic_activity::MILLIKATAL; }
        /// Katal (`kat`).
        Katal => { symbol: "kat"; definition: crate::consts::catalytic_activity::KATAL; }
        /// Enzyme unit (`U`).
        EnzymeUnit => { symbol: "U"; definition: crate::consts::catalytic_activity::ENZYME_UNIT; }
        /// Milli enzyme unit (`mU`).
        MilliEnzymeUnit => { symbol: "mU"; definition: crate::consts::catalytic_activity::MILLI_ENZYME_UNIT; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    CatalyticActivity, UomCatalyticActivity {
        Microkatal => microkatal;
        Millikatal => millikatal;
        Katal => katal;
        EnzymeUnit => enzyme_unit;
        MilliEnzymeUnit => milli_enzyme_unit;
    }
}
