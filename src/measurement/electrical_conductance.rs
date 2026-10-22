// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted electrical conductance measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted electrical conductance measurement.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::ElectricalConductance;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let conductance = ElectricalConductance::new(
///     Decimal::ONE,
///     unit::ElectricalConductance::Siemens,
/// );
/// assert_eq!(conductance.value, Decimal::ONE);
/// assert_eq!(conductance.unit, unit::ElectricalConductance::Siemens);
/// ```
pub type ElectricalConductance = Measurement<unit::ElectricalConductance>;
