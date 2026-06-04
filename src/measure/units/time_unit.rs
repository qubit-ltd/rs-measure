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
use uom::si::f64::Time;
use uom::si::time::{
    day,
    hour,
    millisecond,
    minute,
    second,
};

define_measurement_unit! {
    /// Units for persisted `uom` time quantities.
    pub enum TimeUnit for Time, "time" {
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
    }
}
