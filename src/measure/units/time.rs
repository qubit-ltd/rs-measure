/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Units for persisted time measurements.

use super::define_measurement_unit;
use uom::si::f64::Time as UomTime;
use uom::si::time::{
    day,
    hour,
    microsecond,
    millisecond,
    minute,
    nanosecond,
    second,
    year,
};

define_measurement_unit! {
    /// Units for persisted `uom` time quantities.
    pub enum Time for UomTime, "time" {
        /// Nanosecond (`ns`).
        Nanosecond => "ns", nanosecond;
        /// Microsecond (`µs`).
        Microsecond => "µs" | "us" | "μs", microsecond;
        /// Millisecond (`ms`).
        Millisecond => "ms", millisecond;
        /// Second (`s`).
        Second => "s", second;
        /// Minute (`min`).
        Minute => "min", minute;
        /// Hour (`h`).
        Hour => "h", hour;
        /// Day (`d`).
        Day => "d", day;
        /// Year (`a`).
        Year => "a" | "yr" | "year", year;
    }
}
