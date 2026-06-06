// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted frequency measurements.

use super::define_measurement_unit;
use uom::si::f64::Frequency as UomFrequency;
use uom::si::frequency::{
    gigahertz,
    hertz,
    kilohertz,
    megahertz,
};

define_measurement_unit! {
    /// Units for persisted `uom` frequency quantities.
    pub enum Frequency for UomFrequency, "frequency" {
        /// Hertz (`Hz`).
        Hertz => "Hz", hertz;
        /// Kilohertz (`kHz`).
        Kilohertz => "kHz", kilohertz;
        /// Megahertz (`MHz`).
        Megahertz => "MHz", megahertz;
        /// Gigahertz (`GHz`).
        Gigahertz => "GHz", gigahertz;
    }
}
