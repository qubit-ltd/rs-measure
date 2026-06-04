/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Units for persisted radioactivity measurements.

use super::define_measurement_unit;
use uom::si::f64::Radioactivity as UomRadioactivity;
use uom::si::radioactivity::{
    becquerel,
    curie,
    disintegrations_per_minute,
    kilobecquerel,
    megabecquerel,
    microcurie,
    millicurie,
};

define_measurement_unit! {
    /// Units for persisted `uom` radioactivity quantities.
    pub enum Radioactivity for UomRadioactivity, "radioactivity" {
        /// Becquerel (`Bq`).
        Becquerel => "Bq", becquerel;
        /// Kilobecquerel (`kBq`).
        Kilobecquerel => "kBq", kilobecquerel;
        /// Megabecquerel (`MBq`).
        Megabecquerel => "MBq", megabecquerel;
        /// Curie (`Ci`).
        Curie => "Ci", curie;
        /// Millicurie (`mCi`).
        Millicurie => "mCi", millicurie;
        /// Microcurie (`µCi`).
        Microcurie => "µCi" | "uCi" | "μCi", microcurie;
        /// Disintegrations per minute (`dpm`).
        DisintegrationsPerMinute => "dpm", disintegrations_per_minute;
    }
}
