// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted time measurements.

use crate::define_unit_family;
#[cfg(feature = "uom")]
use uom::si::f64::Time as UomTime;
#[cfg(feature = "uom")]
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

define_unit_family! {
    /// Units for persisted time measurements.
    pub enum Time for "time", uom = UomTime {
        /// Nanosecond (`ns`).
        Nanosecond => { symbol: "ns"; definition: crate::consts::time::NANOSECOND; uom: nanosecond; }
        /// Microsecond (`µs`).
        Microsecond => { symbol: "µs"; definition: crate::consts::time::MICROSECOND; aliases: ["us", "μs"]; uom: microsecond; }
        /// Millisecond (`ms`).
        Millisecond => { symbol: "ms"; definition: crate::consts::time::MILLISECOND; uom: millisecond; }
        /// Second (`s`).
        Second => { symbol: "s"; definition: crate::consts::time::SECOND; uom: second; }
        /// Minute (`min`).
        Minute => { symbol: "min"; definition: crate::consts::time::MINUTE; uom: minute; }
        /// Hour (`h`).
        Hour => { symbol: "h"; definition: crate::consts::time::HOUR; uom: hour; }
        /// Day (`d`).
        Day => { symbol: "d"; definition: crate::consts::time::DAY; uom: day; }
        /// Common 365-day year with canonical symbol `a (365 d)`.
        CommonYear365 => { symbol: "a (365 d)"; definition: crate::consts::time::COMMON_YEAR365; aliases: ["a", "yr", "year"]; uom: year; }
    }
}
