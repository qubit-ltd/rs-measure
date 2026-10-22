// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted magnetic flux density measurement alias.

use crate::Measurement;
use crate::unit;

/// A magnetic flux density value paired with the unit used to express it.
///
/// The exact decimal value and its unit are retained when the measurement is
/// serialized, so values in teslas and gauss remain distinguishable.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::MagneticFluxDensity;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let density = MagneticFluxDensity::new(Decimal::ONE, unit::MagneticFluxDensity::Tesla);
/// assert_eq!(density.value, Decimal::ONE);
/// assert_eq!(density.unit, unit::MagneticFluxDensity::Tesla);
/// ```
pub type MagneticFluxDensity = Measurement<unit::MagneticFluxDensity>;
