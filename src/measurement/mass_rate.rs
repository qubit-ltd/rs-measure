// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted mass rate measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted mass rate measurement.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::MassRate;
/// use rust_decimal::Decimal;
///
/// let value = MassRate::parse_strict("1 kg/s")?;
/// assert_eq!(value.value, Decimal::ONE);
/// assert_eq!(value.to_string(), "1 kg/s");
/// # Ok::<(), qubit_measure::MeasurementError>(())
/// ```
pub type MassRate = Measurement<unit::MassRate>;
