// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted temperature interval measurements.

#[cfg(feature = "uom")]
use uom::si::f64::TemperatureInterval as UomTemperatureInterval;
#[cfg(feature = "uom")]
use uom::si::temperature_interval::kelvin;

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;

define_unit_family! {
    /// Units for persisted temperature interval measurements.
    ///
    /// These units measure temperature differences, so `1 °C` converts to
    /// `1 K` without an absolute zero offset. Absolute temperatures use
    /// [`crate::unit::Temperature`] instead.
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
    pub enum TemperatureInterval for "temperature_interval" {
        /// Kelvin (`K`).
        Kelvin => { symbol: "K"; definition: crate::consts::temperature_interval::KELVIN; }
        /// Degree Celsius (`°C`).
        Celsius => { symbol: "°C"; definition: crate::consts::temperature_interval::CELSIUS; aliases: ["degC"]; }
        /// Degree Fahrenheit (`°F`).
        Fahrenheit => { symbol: "°F"; definition: crate::consts::temperature_interval::FAHRENHEIT; aliases: ["degF"]; }
        /// Degree Rankine (`°R`).
        Rankine => { symbol: "°R"; definition: crate::consts::temperature_interval::RANKINE; aliases: ["degR"]; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    TemperatureInterval, UomTemperatureInterval {
        base: kelvin;
    }
}
