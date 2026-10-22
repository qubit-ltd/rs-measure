// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted amount of substance measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted amount of substance measurement with an exact decimal value.
///
/// The unit identifies the amount scale, such as moles or micromoles, and is
/// preserved with the value when the measurement is serialized.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::AmountOfSubstance;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let amount = AmountOfSubstance::new(Decimal::ONE, unit::AmountOfSubstance::Mole);
/// assert_eq!(amount.value, Decimal::ONE);
/// assert_eq!(amount.unit, unit::AmountOfSubstance::Mole);
/// ```
pub type AmountOfSubstance = Measurement<unit::AmountOfSubstance>;
