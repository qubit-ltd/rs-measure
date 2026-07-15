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
        BecquerelPerKilogram => { symbol: "Bq/kg"; definition: crate::consts::specific_radioactivity::BECQUEREL_PER_KILOGRAM; uom: becquerel_per_kilogram; }
        /// Curie per kilogram (`Ci/kg`).
        CuriePerKilogram => { symbol: "Ci/kg"; definition: crate::consts::specific_radioactivity::CURIE_PER_KILOGRAM; uom: curie_per_kilogram; }
        /// Disintegrations per minute per kilogram (`dpm/kg`).
        DisintegrationsPerMinutePerKilogram => { symbol: "dpm/kg"; definition: crate::consts::specific_radioactivity::DISINTEGRATIONS_PER_MINUTE_PER_KILOGRAM; uom: disintegrations_per_minute_per_kilogram; }
    }
}
