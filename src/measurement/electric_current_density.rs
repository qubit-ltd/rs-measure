// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted electric current density measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted electric current density measurement with an exact decimal
/// value.
///
/// The typed unit identifies the area-normalized current scale and is preserved
/// with the value when the measurement is serialized.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::ElectricCurrentDensity;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let density = ElectricCurrentDensity::new(
///     Decimal::ONE,
///     unit::ElectricCurrentDensity::AmperePerSquareMeter,
/// );
/// assert_eq!(density.value, Decimal::ONE);
/// assert_eq!(density.unit, unit::ElectricCurrentDensity::AmperePerSquareMeter);
/// ```
pub type ElectricCurrentDensity = Measurement<unit::ElectricCurrentDensity>;
