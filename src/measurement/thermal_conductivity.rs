// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted thermal conductivity measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted thermal conductivity value with an exact decimal and typed unit.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::ThermalConductivity;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let conductivity = ThermalConductivity::new(
///     Decimal::ONE,
///     unit::ThermalConductivity::WattPerMeterKelvin,
/// );
/// assert_eq!(conductivity.unit, unit::ThermalConductivity::WattPerMeterKelvin);
/// ```
pub type ThermalConductivity = Measurement<unit::ThermalConductivity>;
