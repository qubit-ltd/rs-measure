/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Units for persisted capacitance measurements.

use super::define_measurement_unit;
use uom::si::capacitance::{
    farad,
    microfarad,
    millifarad,
    nanofarad,
    picofarad,
};
use uom::si::f64::Capacitance as UomCapacitance;

define_measurement_unit! {
    /// Units for persisted `uom` capacitance quantities.
    pub enum Capacitance for UomCapacitance, "capacitance" {
        /// Picofarad (`pF`).
        Picofarad => "pF", picofarad;
        /// Nanofarad (`nF`).
        Nanofarad => "nF", nanofarad;
        /// Microfarad (`µF`).
        Microfarad => "µF" | "uF" | "μF", microfarad;
        /// Millifarad (`mF`).
        Millifarad => "mF", millifarad;
        /// Farad (`F`).
        Farad => "F", farad;
    }
}
