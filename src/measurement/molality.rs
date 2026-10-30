// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted molality measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted molality measurement.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::Molality;
/// use rust_decimal::Decimal;
///
/// let value = Molality::parse_strict("1 mol/kg")?;
/// assert_eq!(value.value, Decimal::ONE);
/// assert_eq!(value.to_string(), "1 mol/kg");
/// # Ok::<(), qubit_measure::MeasurementError>(())
/// ```
pub type Molality = Measurement<unit::Molality>;
