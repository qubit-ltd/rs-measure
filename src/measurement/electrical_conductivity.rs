// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted electrical conductivity measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted electrical conductivity measurement.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::ElectricalConductivity;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let conductivity = ElectricalConductivity::new(
///     Decimal::ONE,
///     unit::ElectricalConductivity::SiemensPerMeter,
/// );
/// assert_eq!(conductivity.value, Decimal::ONE);
/// assert_eq!(conductivity.unit, unit::ElectricalConductivity::SiemensPerMeter);
/// ```
pub type ElectricalConductivity = Measurement<unit::ElectricalConductivity>;
