// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted pressure measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted pressure value that keeps its exact decimal and typed unit.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::Pressure;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let pressure = Pressure::new(Decimal::ONE, unit::Pressure::Pascal);
/// assert_eq!(pressure.unit, unit::Pressure::Pascal);
/// ```
pub type Pressure = Measurement<unit::Pressure>;
