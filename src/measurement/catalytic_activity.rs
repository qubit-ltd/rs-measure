// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted catalytic activity measurement alias.

use crate::Measurement;
use crate::unit;

/// A catalytic activity value whose decimal magnitude is interpreted in its
/// stored unit.
///
/// # Examples
///
/// ```
/// use qubit_measure::unit;
/// use qubit_measure::measurement::CatalyticActivity;
/// use rust_decimal::Decimal;
///
/// let activity = CatalyticActivity::new(Decimal::ONE, unit::CatalyticActivity::Katal);
/// assert_eq!(activity.value, Decimal::ONE);
/// assert_eq!(activity.unit, unit::CatalyticActivity::Katal);
/// ```
pub type CatalyticActivity = Measurement<unit::CatalyticActivity>;
