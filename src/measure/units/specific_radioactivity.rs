// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted specific radioactivity measurements.

use crate::define_unit_family;
use uom::si::f64::SpecificRadioactivity as UomSpecificRadioactivity;
use uom::si::specific_radioactivity::{
    becquerel_per_kilogram,
    curie_per_kilogram,
    disintegrations_per_minute_per_kilogram,
};

define_unit_family! {
    /// Units for persisted `uom` specific radioactivity quantities.
    pub enum SpecificRadioactivity for "specific_radioactivity", uom = UomSpecificRadioactivity {
        /// Becquerel per kilogram (`Bq/kg`).
        BecquerelPerKilogram => { symbol: "Bq/kg"; coefficient: 1; uom: becquerel_per_kilogram; }
        /// Curie per kilogram (`Ci/kg`).
        CuriePerKilogram => { symbol: "Ci/kg"; coefficient: 37000000000; uom: curie_per_kilogram; }
        /// Disintegrations per minute per kilogram (`dpm/kg`).
        DisintegrationsPerMinutePerKilogram => { symbol: "dpm/kg"; coefficient: 1 / 60; uom: disintegrations_per_minute_per_kilogram; }
    }
}
