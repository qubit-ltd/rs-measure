// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted heat flux density measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted heat flux density measurement with an exact decimal value.
///
/// The unit identifies the heat flux density scale, such as watts per square
/// meter, and is preserved with the value when the measurement is serialized.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::HeatFluxDensity;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let heat_flux_density = HeatFluxDensity::new(
///     Decimal::ONE,
///     unit::HeatFluxDensity::WattPerSquareMeter,
/// );
/// assert_eq!(heat_flux_density.value, Decimal::ONE);
/// assert_eq!(
///     heat_flux_density.unit,
///     unit::HeatFluxDensity::WattPerSquareMeter,
/// );
/// ```
pub type HeatFluxDensity = Measurement<unit::HeatFluxDensity>;
