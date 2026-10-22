// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted force measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted force measurement with an exact decimal value.
///
/// The unit identifies the force scale, such as newtons, and is preserved
/// with the value when the measurement is serialized.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::Force;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let force = Force::new(Decimal::ONE, unit::Force::Newton);
/// assert_eq!(force.value, Decimal::ONE);
/// assert_eq!(force.unit, unit::Force::Newton);
/// ```
pub type Force = Measurement<unit::Force>;
