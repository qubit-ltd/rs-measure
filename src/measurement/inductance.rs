// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted inductance measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted inductance measurement with an exact decimal value.
///
/// The unit identifies the inductance scale, such as henries, and is
/// preserved with the value when the measurement is serialized.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::Inductance;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let inductance = Inductance::new(Decimal::ONE, unit::Inductance::Henry);
/// assert_eq!(inductance.value, Decimal::ONE);
/// assert_eq!(inductance.unit, unit::Inductance::Henry);
/// ```
pub type Inductance = Measurement<unit::Inductance>;
