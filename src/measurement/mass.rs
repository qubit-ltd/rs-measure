// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted mass measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted mass measurement.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::Mass;
/// use rust_decimal::Decimal;
///
/// let value = Mass::parse_strict("1 kg")?;
/// assert_eq!(value.value, Decimal::ONE);
/// assert_eq!(value.to_string(), "1 kg");
/// # Ok::<(), qubit_measure::MeasurementError>(())
/// ```
pub type Mass = Measurement<unit::Mass>;
