// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted electrical conductance measurements.

use super::define_measurement_unit;
use uom::si::electrical_conductance::{
    microsiemens,
    millisiemens,
    siemens,
};
use uom::si::f64::ElectricalConductance as UomElectricalConductance;

define_measurement_unit! {
    /// Units for persisted `uom` electrical conductance quantities.
    pub enum ElectricalConductance for UomElectricalConductance, "electrical conductance" {
        /// Microsiemens (`µS`).
        Microsiemens => "µS" | "uS" | "μS", microsiemens;
        /// Millisiemens (`mS`).
        Millisiemens => "mS", millisiemens;
        /// Siemens (`S`).
        Siemens => "S", siemens;
    }
}
