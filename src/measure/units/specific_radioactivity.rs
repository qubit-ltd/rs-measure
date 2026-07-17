// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted specific radioactivity measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;
#[cfg(feature = "uom")]
use uom::si::f64::SpecificRadioactivity as UomSpecificRadioactivity;
#[cfg(feature = "uom")]
use uom::si::specific_radioactivity::becquerel_per_kilogram;

define_unit_family! {
    /// Units for persisted specific radioactivity measurements.
    pub enum SpecificRadioactivity for "specific_radioactivity" {
        /// Becquerel per kilogram (`Bq/kg`).
        BecquerelPerKilogram => { symbol: "Bq/kg"; definition: crate::consts::specific_radioactivity::BECQUEREL_PER_KILOGRAM; }
        /// Curie per kilogram (`Ci/kg`).
        CuriePerKilogram => { symbol: "Ci/kg"; definition: crate::consts::specific_radioactivity::CURIE_PER_KILOGRAM; }
        /// Disintegrations per minute per kilogram (`dpm/kg`).
        DisintegrationsPerMinutePerKilogram => { symbol: "dpm/kg"; definition: crate::consts::specific_radioactivity::DISINTEGRATIONS_PER_MINUTE_PER_KILOGRAM; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    SpecificRadioactivity, UomSpecificRadioactivity {
        base: becquerel_per_kilogram;
    }
}
