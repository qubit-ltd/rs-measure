// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted electrical resistivity measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted electrical resistivity measurement.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::ElectricalResistivity;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let resistivity = ElectricalResistivity::new(
///     Decimal::ONE,
///     unit::ElectricalResistivity::OhmMeter,
/// );
/// assert_eq!(resistivity.value, Decimal::ONE);
/// assert_eq!(resistivity.unit, unit::ElectricalResistivity::OhmMeter);
/// ```
pub type ElectricalResistivity = Measurement<unit::ElectricalResistivity>;
