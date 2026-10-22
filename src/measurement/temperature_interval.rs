// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted temperature interval measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted temperature interval value that keeps its exact decimal and
/// typed unit.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::TemperatureInterval;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let interval = TemperatureInterval::new(Decimal::ONE, unit::TemperatureInterval::Kelvin);
/// assert_eq!(interval.unit, unit::TemperatureInterval::Kelvin);
/// ```
pub type TemperatureInterval = Measurement<unit::TemperatureInterval>;
