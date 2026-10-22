// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted angular velocity measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted angular velocity measurement with an exact decimal value.
///
/// The typed unit identifies the angular speed scale, such as radians per
/// second or revolutions per minute, and is preserved with the value when the
/// measurement is serialized.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::AngularVelocity;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let speed = AngularVelocity::new(Decimal::ONE, unit::AngularVelocity::RadianPerSecond);
/// assert_eq!(speed.value, Decimal::ONE);
/// assert_eq!(speed.unit, unit::AngularVelocity::RadianPerSecond);
/// ```
pub type AngularVelocity = Measurement<unit::AngularVelocity>;
