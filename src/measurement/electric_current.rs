// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted electric current measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted electric current measurement.
///
/// # Examples
///
/// ```
/// use qubit_measure::{measurement::ElectricCurrent, unit};
/// use rust_decimal::Decimal;
///
/// let current = ElectricCurrent::new(Decimal::ONE, unit::ElectricCurrent::Ampere);
/// assert_eq!(current.unit, unit::ElectricCurrent::Ampere);
/// ```
pub type ElectricCurrent = Measurement<unit::ElectricCurrent>;
