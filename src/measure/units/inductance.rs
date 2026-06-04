/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Units for persisted inductance measurements.

use super::define_measurement_unit;
use uom::si::f64::Inductance as UomInductance;
use uom::si::inductance::{
    henry,
    microhenry,
    millihenry,
    nanohenry,
};

define_measurement_unit! {
    /// Units for persisted `uom` inductance quantities.
    pub enum Inductance for UomInductance, "inductance" {
        /// Nanohenry (`nH`).
        Nanohenry => "nH", nanohenry;
        /// Microhenry (`µH`).
        Microhenry => "µH" | "uH" | "μH", microhenry;
        /// Millihenry (`mH`).
        Millihenry => "mH", millihenry;
        /// Henry (`H`).
        Henry => "H", henry;
    }
}
