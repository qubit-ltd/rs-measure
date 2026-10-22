// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted luminance measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted luminance value paired with its typed unit.
///
/// The unit records whether the value is expressed in candelas per square
/// meter, square centimeter, or square foot, foot-lamberts, or stilbs, so the
/// scale remains explicit when the measurement is stored or serialized.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::Luminance;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let luminance = Luminance::new(Decimal::ONE, unit::Luminance::CandelaPerSquareMeter);
/// assert_eq!(luminance.value, Decimal::ONE);
/// assert_eq!(luminance.unit, unit::Luminance::CandelaPerSquareMeter);
/// ```
pub type Luminance = Measurement<unit::Luminance>;
