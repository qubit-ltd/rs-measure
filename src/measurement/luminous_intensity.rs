// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted luminous intensity measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted luminous intensity value paired with its typed unit.
///
/// The unit records whether the value is expressed in millicandelas,
/// candelas, or kilocandelas, so its scale remains explicit when the
/// measurement is stored or serialized.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::LuminousIntensity;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let intensity = LuminousIntensity::new(Decimal::ONE, unit::LuminousIntensity::Candela);
/// assert_eq!(intensity.value, Decimal::ONE);
/// assert_eq!(intensity.unit, unit::LuminousIntensity::Candela);
/// ```
pub type LuminousIntensity = Measurement<unit::LuminousIntensity>;
