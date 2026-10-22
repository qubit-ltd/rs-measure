// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted electrical resistance measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted electrical resistance measurement.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::ElectricalResistance;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let resistance = ElectricalResistance::new(
///     Decimal::ONE,
///     unit::ElectricalResistance::Ohm,
/// );
/// assert_eq!(resistance.value, Decimal::ONE);
/// assert_eq!(resistance.unit, unit::ElectricalResistance::Ohm);
/// ```
pub type ElectricalResistance = Measurement<unit::ElectricalResistance>;
