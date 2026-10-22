// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted dynamic viscosity measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted dynamic viscosity measurement with an exact decimal value.
///
/// The typed unit identifies the viscosity scale, such as pascal seconds or
/// centipoise, and is preserved with the value when the measurement is
/// serialized.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::DynamicViscosity;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let viscosity = DynamicViscosity::new(Decimal::ONE, unit::DynamicViscosity::PascalSecond);
/// assert_eq!(viscosity.value, Decimal::ONE);
/// assert_eq!(viscosity.unit, unit::DynamicViscosity::PascalSecond);
/// ```
pub type DynamicViscosity = Measurement<unit::DynamicViscosity>;
