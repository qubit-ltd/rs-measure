// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted torque measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted torque value that keeps its exact decimal and typed unit.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::Torque;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let torque = Torque::new(Decimal::ONE, unit::Torque::NewtonMeter);
/// assert_eq!(torque.unit, unit::Torque::NewtonMeter);
/// ```
pub type Torque = Measurement<unit::Torque>;
