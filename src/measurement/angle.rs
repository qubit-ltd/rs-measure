// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted angle measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted angle measurement.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::Angle;
/// use rust_decimal::Decimal;
///
/// let value = Angle::parse_strict("1 rad")?;
/// assert_eq!(value.value, Decimal::ONE);
/// assert_eq!(value.to_string(), "1 rad");
/// # Ok::<(), qubit_measure::MeasurementError>(())
/// ```
pub type Angle = Measurement<unit::Angle>;
