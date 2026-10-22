// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted volume measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted measurement whose value is expressed in a volume unit.
///
/// # Examples
///
/// ```
/// use qubit_measure::{measurement::Volume, unit};
/// use rust_decimal::Decimal;
///
/// let volume = Volume::new(Decimal::ONE, unit::Volume::Liter);
/// assert_eq!(volume.unit, unit::Volume::Liter);
/// ```
pub type Volume = Measurement<unit::Volume>;
