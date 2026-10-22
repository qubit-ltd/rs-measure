// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted measurements for solid angles.

use crate::Measurement;
use crate::unit;

/// A persisted decimal measurement whose unit is a solid angle.
///
/// The alias keeps the numeric value and its [`unit::SolidAngle`] together,
/// so unit conversions and serialization retain the selected solid angle unit.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::SolidAngle;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let measurement = SolidAngle::new(Decimal::ONE, unit::SolidAngle::Steradian);
/// assert_eq!(measurement.unit, unit::SolidAngle::Steradian);
/// ```
pub type SolidAngle = Measurement<unit::SolidAngle>;
