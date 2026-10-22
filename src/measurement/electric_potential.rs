// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted electric potential measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted electric potential measurement.
///
/// This is the SI quantity commonly called voltage.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::ElectricPotential;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let voltage = ElectricPotential::new(Decimal::ONE, unit::ElectricPotential::Volt);
/// assert_eq!(voltage.value, Decimal::ONE);
/// assert_eq!(voltage.unit, unit::ElectricPotential::Volt);
/// ```
pub type ElectricPotential = Measurement<unit::ElectricPotential>;
