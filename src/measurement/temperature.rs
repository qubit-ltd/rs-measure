// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted thermodynamic temperature measurement alias.

use crate::Measurement;
use crate::unit;

/// A persisted thermodynamic temperature measurement, representing an absolute
/// point on the temperature scale.
///
/// Absolute temperatures include an offset when converted between Celsius and
/// Kelvin; temperature intervals do not. See
/// [`TemperatureInterval`](crate::measurement::TemperatureInterval) for
/// temperature differences.
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
pub type Temperature = Measurement<unit::Temperature>;
