// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted frequency measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted frequency measurement with an exact decimal value.
///
/// The unit identifies the frequency scale, such as hertz or kilohertz, and
/// is preserved with the value when the measurement is serialized.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::Frequency;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let frequency = Frequency::new(Decimal::ONE, unit::Frequency::Hertz);
/// assert_eq!(frequency.value, Decimal::ONE);
/// assert_eq!(frequency.unit, unit::Frequency::Hertz);
/// ```
pub type Frequency = Measurement<unit::Frequency>;
