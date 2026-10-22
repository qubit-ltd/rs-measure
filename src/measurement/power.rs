// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted power measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted power value that keeps its exact decimal and typed unit.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::Power;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let power = Power::new(Decimal::ONE, unit::Power::Watt);
/// assert_eq!(power.unit, unit::Power::Watt);
/// ```
pub type Power = Measurement<unit::Power>;
