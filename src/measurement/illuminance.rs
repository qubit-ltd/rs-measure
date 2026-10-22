// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted illuminance measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted illuminance value paired with its typed unit.
///
/// The unit records whether the value is expressed in lux, kilolux, or
/// foot-candles, so the scale remains explicit when the measurement is stored
/// or serialized.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::Illuminance;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let illuminance = Illuminance::new(Decimal::ONE, unit::Illuminance::Lux);
/// assert_eq!(illuminance.value, Decimal::ONE);
/// assert_eq!(illuminance.unit, unit::Illuminance::Lux);
/// ```
pub type Illuminance = Measurement<unit::Illuminance>;
