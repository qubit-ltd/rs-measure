// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted acceleration measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted acceleration measurement.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::Acceleration;
/// use rust_decimal::Decimal;
///
/// let value = Acceleration::parse_strict("1 m/s²")?;
/// assert_eq!(value.value, Decimal::ONE);
/// assert_eq!(value.to_string(), "1 m/s²");
/// # Ok::<(), qubit_measure::MeasurementError>(())
/// ```
pub type Acceleration = Measurement<unit::Acceleration>;
