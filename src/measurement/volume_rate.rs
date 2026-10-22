// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted volume rate measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted volume rate value that keeps its exact decimal and typed unit.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::VolumeRate;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let flow = VolumeRate::new(Decimal::ONE, unit::VolumeRate::LiterPerSecond);
/// assert_eq!(flow.unit, unit::VolumeRate::LiterPerSecond);
/// ```
pub type VolumeRate = Measurement<unit::VolumeRate>;
