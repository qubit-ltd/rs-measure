// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted voltage measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted voltage value that keeps its exact decimal and typed unit.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::Voltage;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let voltage = Voltage::new(Decimal::ONE, unit::ElectricPotential::Volt);
/// assert_eq!(voltage.unit, unit::ElectricPotential::Volt);
/// ```
pub type Voltage = Measurement<unit::ElectricPotential>;
