// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted specific heat capacity measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted specific heat capacity measurement.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::SpecificHeatCapacity;
/// use rust_decimal::Decimal;
///
/// let value = SpecificHeatCapacity::parse_strict("1 J/(kg · K)")?;
/// assert_eq!(value.value, Decimal::ONE);
/// assert_eq!(value.to_string(), "1 J/(kg · K)");
/// # Ok::<(), qubit_measure::MeasurementError>(())
/// ```
pub type SpecificHeatCapacity = Measurement<unit::SpecificHeatCapacity>;
