// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted molar volume measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted molar volume value that keeps its exact decimal and typed unit.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::MolarVolume;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let volume = MolarVolume::new(Decimal::ONE, unit::MolarVolume::CubicMeterPerMole);
/// assert_eq!(volume.unit, unit::MolarVolume::CubicMeterPerMole);
/// ```
pub type MolarVolume = Measurement<unit::MolarVolume>;
