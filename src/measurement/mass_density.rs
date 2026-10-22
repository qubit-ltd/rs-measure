// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted mass density measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted mass density measurement with an exact decimal value.
///
/// The unit identifies the mass density scale, such as kilograms per cubic
/// meter, and is preserved with the value when the measurement is serialized.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::MassDensity;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let mass_density = MassDensity::new(
///     Decimal::ONE,
///     unit::MassDensity::KilogramPerCubicMeter,
/// );
/// assert_eq!(mass_density.value, Decimal::ONE);
/// assert_eq!(
///     mass_density.unit,
///     unit::MassDensity::KilogramPerCubicMeter,
/// );
/// ```
pub type MassDensity = Measurement<unit::MassDensity>;
