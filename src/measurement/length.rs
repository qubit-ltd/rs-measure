// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted length measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted length measurement.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::Length;
/// use rust_decimal::Decimal;
///
/// let value = Length::parse_strict("1 m")?;
/// assert_eq!(value.value, Decimal::ONE);
/// assert_eq!(value.to_string(), "1 m");
/// # Ok::<(), qubit_measure::MeasurementError>(())
/// ```
pub type Length = Measurement<unit::Length>;
