// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted kinematic viscosity measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted kinematic viscosity measurement with an exact decimal value.
///
/// The typed unit identifies the kinematic viscosity scale, such as square
/// meters per second or centistokes, and is preserved with the value when the
/// measurement is serialized.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::KinematicViscosity;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let viscosity = KinematicViscosity::new(
///     Decimal::ONE,
///     unit::KinematicViscosity::SquareMeterPerSecond,
/// );
/// assert_eq!(viscosity.value, Decimal::ONE);
/// assert_eq!(
///     viscosity.unit,
///     unit::KinematicViscosity::SquareMeterPerSecond,
/// );
/// ```
pub type KinematicViscosity = Measurement<unit::KinematicViscosity>;
