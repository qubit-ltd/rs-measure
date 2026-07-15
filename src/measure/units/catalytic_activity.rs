// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted catalytic activity measurements.

use crate::define_unit_family;
use uom::si::catalytic_activity::{
    enzyme_unit,
    katal,
    microkatal,
    milli_enzyme_unit,
    millikatal,
};
use uom::si::f64::CatalyticActivity as UomCatalyticActivity;

define_unit_family! {
    /// Units for persisted `uom` catalytic activity quantities.
    pub enum CatalyticActivity for "catalytic_activity", uom = UomCatalyticActivity {
        /// Microkatal (`µkat`).
        Microkatal => { symbol: "µkat"; coefficient: 1 / 1000000; aliases: ["ukat", "μkat"]; uom: microkatal; }
        /// Millikatal (`mkat`).
        Millikatal => { symbol: "mkat"; coefficient: 1 / 1000; uom: millikatal; }
        /// Katal (`kat`).
        Katal => { symbol: "kat"; coefficient: 1; uom: katal; }
        /// Enzyme unit (`U`).
        EnzymeUnit => { symbol: "U"; coefficient: 1 / 60000000; uom: enzyme_unit; }
        /// Milli enzyme unit (`mU`).
        MilliEnzymeUnit => { symbol: "mU"; coefficient: 1 / 60000000000; uom: milli_enzyme_unit; }
    }
}
