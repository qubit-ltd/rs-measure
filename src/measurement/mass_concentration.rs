// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted mass concentration measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted mass concentration measurement.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::MassConcentration;
/// use rust_decimal::Decimal;
///
/// let value = MassConcentration::parse_strict("1 kg/m³")?;
/// assert_eq!(value.value, Decimal::ONE);
/// assert_eq!(value.to_string(), "1 kg/m³");
/// # Ok::<(), qubit_measure::MeasurementError>(())
/// ```
pub type MassConcentration = Measurement<unit::MassConcentration>;
