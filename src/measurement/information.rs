// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted information measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted measurement of digital information, such as bits or bytes.
///
/// The alias retains the exact decimal value together with its typed
/// information unit, so equivalent values expressed in different units remain
/// distinguishable in storage.
///
/// # Examples
///
/// ```
/// use qubit_measure::{measurement::Information, unit};
/// use rust_decimal::Decimal;
///
/// let size = Information::new(Decimal::from(8), unit::Information::Bit);
/// assert_eq!(size.value, Decimal::from(8));
/// assert_eq!(size.unit, unit::Information::Bit);
/// ```
pub type Information = Measurement<unit::Information>;
