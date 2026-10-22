// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted electric charge measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted electric charge measurement.
///
/// # Examples
///
/// ```
/// use qubit_measure::{measurement::ElectricCharge, unit};
/// use rust_decimal::Decimal;
///
/// let charge = ElectricCharge::new(Decimal::ONE, unit::ElectricCharge::Coulomb);
/// assert_eq!(charge.unit, unit::ElectricCharge::Coulomb);
/// ```
pub type ElectricCharge = Measurement<unit::ElectricCharge>;
