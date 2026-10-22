// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted thermal resistance measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted thermal resistance value with an exact decimal and typed unit.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::ThermalResistance;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let resistance = ThermalResistance::new(
///     Decimal::ONE,
///     unit::ThermalResistance::KelvinPerWatt,
/// );
/// assert_eq!(resistance.unit, unit::ThermalResistance::KelvinPerWatt);
/// ```
pub type ThermalResistance = Measurement<unit::ThermalResistance>;
