// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted area measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted area measurement.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::Area;
/// use rust_decimal::Decimal;
///
/// let value = Area::parse_strict("1 m²")?;
/// assert_eq!(value.value, Decimal::ONE);
/// assert_eq!(value.to_string(), "1 m²");
/// # Ok::<(), qubit_measure::MeasurementError>(())
/// ```
pub type Area = Measurement<unit::Area>;
