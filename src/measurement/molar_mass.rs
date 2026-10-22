// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted molar mass measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted molar mass value that keeps its exact decimal and typed unit.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::MolarMass;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let mass = MolarMass::new(Decimal::ONE, unit::MolarMass::GramPerMole);
/// assert_eq!(mass.unit, unit::MolarMass::GramPerMole);
/// ```
pub type MolarMass = Measurement<unit::MolarMass>;
