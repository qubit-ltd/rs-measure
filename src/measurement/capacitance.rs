// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted capacitance measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted capacitance measurement.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::Capacitance;
/// use rust_decimal::Decimal;
///
/// let value = Capacitance::parse_strict("1 F")?;
/// assert_eq!(value.value, Decimal::ONE);
/// assert_eq!(value.to_string(), "1 F");
/// # Ok::<(), qubit_measure::MeasurementError>(())
/// ```
pub type Capacitance = Measurement<unit::Capacitance>;
