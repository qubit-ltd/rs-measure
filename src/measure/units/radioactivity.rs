// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted radioactivity measurements.

use crate::define_unit_family;
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

define_unit_family! {
    /// Units for persisted `uom` radioactivity quantities.
    pub enum Radioactivity for "radioactivity", uom = UomRadioactivity {
        /// Becquerel (`Bq`).
        Becquerel => { symbol: "Bq"; coefficient: 1; uom: becquerel; }
        /// Kilobecquerel (`kBq`).
        Kilobecquerel => { symbol: "kBq"; coefficient: 1000; uom: kilobecquerel; }
        /// Megabecquerel (`MBq`).
        Megabecquerel => { symbol: "MBq"; coefficient: 1000000; uom: megabecquerel; }
        /// Curie (`Ci`).
        Curie => { symbol: "Ci"; coefficient: 37000000000; uom: curie; }
        /// Millicurie (`mCi`).
        Millicurie => { symbol: "mCi"; coefficient: 37000000; uom: millicurie; }
        /// Microcurie (`µCi`).
        Microcurie => { symbol: "µCi"; coefficient: 37000; aliases: ["uCi", "μCi"]; uom: microcurie; }
        /// Disintegrations per minute (`dpm`).
        DisintegrationsPerMinute => { symbol: "dpm"; coefficient: 1 / 60; uom: disintegrations_per_minute; }
    }
}
