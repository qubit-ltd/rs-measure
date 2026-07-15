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
        Becquerel => { symbol: "Bq"; definition: crate::consts::radioactivity::BECQUEREL; uom: becquerel; }
        /// Kilobecquerel (`kBq`).
        Kilobecquerel => { symbol: "kBq"; definition: crate::consts::radioactivity::KILOBECQUEREL; uom: kilobecquerel; }
        /// Megabecquerel (`MBq`).
        Megabecquerel => { symbol: "MBq"; definition: crate::consts::radioactivity::MEGABECQUEREL; uom: megabecquerel; }
        /// Curie (`Ci`).
        Curie => { symbol: "Ci"; definition: crate::consts::radioactivity::CURIE; uom: curie; }
        /// Millicurie (`mCi`).
        Millicurie => { symbol: "mCi"; definition: crate::consts::radioactivity::MILLICURIE; uom: millicurie; }
        /// Microcurie (`µCi`).
        Microcurie => { symbol: "µCi"; definition: crate::consts::radioactivity::MICROCURIE; aliases: ["uCi", "μCi"]; uom: microcurie; }
        /// Disintegrations per minute (`dpm`).
        DisintegrationsPerMinute => { symbol: "dpm"; definition: crate::consts::radioactivity::DISINTEGRATIONS_PER_MINUTE; uom: disintegrations_per_minute; }
    }
}
