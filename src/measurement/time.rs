// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted time measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted time value that keeps its exact decimal and typed unit.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::Time;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let elapsed = Time::new(Decimal::ONE, unit::Time::Second);
/// assert_eq!(elapsed.unit, unit::Time::Second);
/// ```
pub type Time = Measurement<unit::Time>;
