// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted radioactivity measurements.

#[cfg(feature = "uom")]
use uom::si::f64::Radioactivity as UomRadioactivity;
#[cfg(feature = "uom")]
use uom::si::radioactivity::becquerel;

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;

define_unit_family! {
    /// Units for persisted radioactivity measurements.
    pub enum Radioactivity for "radioactivity" {
        /// Becquerel (`Bq`).
        Becquerel => { symbol: "Bq"; definition: crate::consts::radioactivity::BECQUEREL; }
        /// Kilobecquerel (`kBq`).
        Kilobecquerel => { symbol: "kBq"; definition: crate::consts::radioactivity::KILOBECQUEREL; }
        /// Megabecquerel (`MBq`).
        Megabecquerel => { symbol: "MBq"; definition: crate::consts::radioactivity::MEGABECQUEREL; }
        /// Curie (`Ci`).
        Curie => { symbol: "Ci"; definition: crate::consts::radioactivity::CURIE; }
        /// Millicurie (`mCi`).
        Millicurie => { symbol: "mCi"; definition: crate::consts::radioactivity::MILLICURIE; }
        /// Microcurie (`µCi`).
        Microcurie => { symbol: "µCi"; definition: crate::consts::radioactivity::MICROCURIE; aliases: ["uCi", "μCi"]; }
        /// Disintegrations per minute (`dpm`).
        DisintegrationsPerMinute => { symbol: "dpm"; definition: crate::consts::radioactivity::DISINTEGRATIONS_PER_MINUTE; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    Radioactivity, UomRadioactivity {
        base: becquerel;
    }
}
