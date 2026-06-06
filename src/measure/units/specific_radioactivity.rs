// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted specific radioactivity measurements.

use super::define_measurement_unit;
use uom::si::f64::SpecificRadioactivity as UomSpecificRadioactivity;
use uom::si::specific_radioactivity::{
    becquerel_per_kilogram,
    curie_per_kilogram,
    disintegrations_per_minute_per_kilogram,
};

define_measurement_unit! {
    /// Units for persisted `uom` specific radioactivity quantities.
    pub enum SpecificRadioactivity for UomSpecificRadioactivity, "specific radioactivity" {
        /// Becquerel per kilogram (`Bq/kg`).
        BecquerelPerKilogram => "Bq/kg", becquerel_per_kilogram;
        /// Curie per kilogram (`Ci/kg`).
        CuriePerKilogram => "Ci/kg", curie_per_kilogram;
        /// Disintegrations per minute per kilogram (`dpm/kg`).
        DisintegrationsPerMinutePerKilogram => "dpm/kg", disintegrations_per_minute_per_kilogram;
    }
}
