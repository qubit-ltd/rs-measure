// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted velocity measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted velocity measurement.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::Velocity;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let speed = Velocity::new(Decimal::ONE, unit::Velocity::MeterPerSecond);
/// assert_eq!(speed.value, Decimal::ONE);
/// assert_eq!(speed.unit, unit::Velocity::MeterPerSecond);
/// ```
pub type Velocity = Measurement<unit::Velocity>;
