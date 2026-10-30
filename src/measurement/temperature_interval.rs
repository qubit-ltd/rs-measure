// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted temperature interval measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted temperature interval value that keeps its exact decimal and
/// typed unit.
///
/// Temperature intervals represent differences: `1 °C` converts to `1 K`
/// without the offset applied to an absolute temperature. See
/// [`Temperature`](crate::measurement::Temperature) for absolute temperature
/// values.
///
/// # Examples
///
/// ```
/// use qubit_measure::measurement::Temperature;
/// use qubit_measure::measurement::TemperatureInterval;
/// use qubit_measure::unit;
/// use rust_decimal::Decimal;
///
/// let temperature = Temperature::parse_strict("0 °C")?;
/// let interval = TemperatureInterval::parse_strict("1 °C")?;
/// assert_eq!(temperature.convert_to(unit::Temperature::Kelvin)?.value, Decimal::new(27315, 2));
/// assert_eq!(interval.convert_to(unit::TemperatureInterval::Kelvin)?.value, Decimal::ONE);
/// assert_eq!(temperature.quantity_name(), "temperature");
/// assert_eq!(interval.quantity_name(), "temperature_interval");
/// # Ok::<(), qubit_measure::MeasurementError>(())
/// ```
pub type TemperatureInterval = Measurement<unit::TemperatureInterval>;
