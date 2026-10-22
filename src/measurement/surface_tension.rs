// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted surface tension measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted surface tension value with an exact decimal and a typed unit.
///
/// The alias restricts [`Measurement`] to surface tension units, keeping the
/// value paired with its interpretation when stored or passed through APIs.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::SurfaceTension;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let tension = SurfaceTension::new(
///     Decimal::ONE,
///     unit::SurfaceTension::NewtonPerMeter,
/// );
/// assert_eq!(tension.unit, unit::SurfaceTension::NewtonPerMeter);
/// ```
pub type SurfaceTension = Measurement<unit::SurfaceTension>;
