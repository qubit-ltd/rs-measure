// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted magnetic flux measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted magnetic flux measurement with an exact decimal value.
///
/// The unit identifies the magnetic flux scale, such as webers, and is
/// preserved with the value when the measurement is serialized.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::MagneticFlux;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let magnetic_flux = MagneticFlux::new(Decimal::ONE, unit::MagneticFlux::Weber);
/// assert_eq!(magnetic_flux.value, Decimal::ONE);
/// assert_eq!(magnetic_flux.unit, unit::MagneticFlux::Weber);
/// ```
pub type MagneticFlux = Measurement<unit::MagneticFlux>;
