// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted molar concentration measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted molar concentration measurement.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::MolarConcentration;
/// use rust_decimal::Decimal;
///
/// let value = MolarConcentration::parse_strict("1 mol/m³")?;
/// assert_eq!(value.value, Decimal::ONE);
/// assert_eq!(value.to_string(), "1 mol/m³");
/// # Ok::<(), qubit_measure::MeasurementError>(())
/// ```
pub type MolarConcentration = Measurement<unit::MolarConcentration>;
