// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted specific radioactivity measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted specific radioactivity value with an exact decimal and a typed
/// unit.
///
/// The alias restricts [`Measurement`] to specific radioactivity units, keeping
/// the value paired with its interpretation when stored or passed through APIs.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::SpecificRadioactivity;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let activity = SpecificRadioactivity::new(
///     Decimal::ONE,
///     unit::SpecificRadioactivity::BecquerelPerKilogram,
/// );
/// assert_eq!(activity.unit, unit::SpecificRadioactivity::BecquerelPerKilogram);
/// ```
pub type SpecificRadioactivity = Measurement<unit::SpecificRadioactivity>;
