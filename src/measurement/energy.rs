// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted energy measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted energy measurement with an exact decimal value.
///
/// The unit identifies the energy scale, such as joules or kilowatt hours,
/// and is preserved with the value when the measurement is serialized.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::Energy;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let energy = Energy::new(Decimal::ONE, unit::Energy::Joule);
/// assert_eq!(energy.value, Decimal::ONE);
/// assert_eq!(energy.unit, unit::Energy::Joule);
/// ```
pub type Energy = Measurement<unit::Energy>;
