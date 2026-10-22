// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted measurements whose values represent radioactivity.

use crate::Measurement;
use crate::unit;

/// A persisted radioactivity value with an exact decimal and a typed unit.
///
/// The alias restricts [`Measurement`] to radioactivity units, so the value
/// and its interpretation remain paired when stored or passed through APIs.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::Radioactivity;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let activity = Radioactivity::new(Decimal::ONE, unit::Radioactivity::Becquerel);
/// assert_eq!(activity.unit, unit::Radioactivity::Becquerel);
/// ```
pub type Radioactivity = Measurement<unit::Radioactivity>;
