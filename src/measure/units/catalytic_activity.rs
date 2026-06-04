/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Units for persisted catalytic activity measurements.

use super::define_measurement_unit;
use uom::si::catalytic_activity::{
    enzyme_unit,
    katal,
    microkatal,
    milli_enzyme_unit,
    millikatal,
};
use uom::si::f64::CatalyticActivity as UomCatalyticActivity;

define_measurement_unit! {
    /// Units for persisted `uom` catalytic activity quantities.
    pub enum CatalyticActivity for UomCatalyticActivity, "catalytic activity" {
        /// Microkatal (`µkat`).
        Microkatal => "µkat" | "ukat" | "μkat", microkatal;
        /// Millikatal (`mkat`).
        Millikatal => "mkat", millikatal;
        /// Katal (`kat`).
        Katal => "kat", katal;
        /// Enzyme unit (`U`).
        EnzymeUnit => "U", enzyme_unit;
        /// Milli enzyme unit (`mU`).
        MilliEnzymeUnit => "mU", milli_enzyme_unit;
    }
}
