// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted magnetic field strength measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted magnetic field strength measurement.
///
/// The unit records the scale used for the value, such as amperes per meter.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::MagneticFieldStrength;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let field_strength = MagneticFieldStrength::new(
///     Decimal::ONE,
///     unit::MagneticFieldStrength::AmperePerMeter,
/// );
/// assert_eq!(field_strength.value, Decimal::ONE);
/// assert_eq!(
///     field_strength.unit,
///     unit::MagneticFieldStrength::AmperePerMeter,
/// );
/// ```
pub type MagneticFieldStrength = Measurement<unit::MagneticFieldStrength>;
