// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted catalytic activity concentration measurement alias.

use crate::Measurement;
use crate::unit;

/// A catalytic activity concentration value whose decimal magnitude is
/// interpreted in its stored unit.
///
/// # Examples
///
/// ```
/// use qubit_measure::unit;
/// use qubit_measure::measurement::CatalyticActivityConcentration;
/// use rust_decimal::Decimal;
///
/// let concentration = CatalyticActivityConcentration::new(
///     Decimal::ONE,
///     unit::CatalyticActivityConcentration::KatalPerCubicMeter,
/// );
/// assert_eq!(concentration.value, Decimal::ONE);
/// assert_eq!(
///     concentration.unit,
///     unit::CatalyticActivityConcentration::KatalPerCubicMeter,
/// );
/// ```
pub type CatalyticActivityConcentration = Measurement<unit::CatalyticActivityConcentration>;
