/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Units for persisted molality measurements.

use super::define_measurement_unit;
use uom::si::f64::Molality as UomMolality;
use uom::si::molality::mole_per_kilogram;

define_measurement_unit! {
    /// Units for persisted `uom` molality quantities.
    pub enum Molality for UomMolality, "molality" {
        /// Mole per kilogram (`mol/kg`).
        MolePerKilogram => "mol/kg", mole_per_kilogram;
    }
}
