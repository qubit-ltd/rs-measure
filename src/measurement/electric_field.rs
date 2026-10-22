// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted electric field measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted electric field measurement.
///
/// # Examples
///
/// ```
/// use qubit_measure::{measurement::ElectricField, unit};
/// use rust_decimal::Decimal;
///
/// let field = ElectricField::new(Decimal::ONE, unit::ElectricField::VoltPerMeter);
/// assert_eq!(field.unit, unit::ElectricField::VoltPerMeter);
/// ```
pub type ElectricField = Measurement<unit::ElectricField>;
