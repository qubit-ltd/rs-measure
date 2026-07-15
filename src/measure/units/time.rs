// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted time measurements.

use crate::define_unit_family;
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

define_unit_family! {
    /// Units for persisted `uom` time quantities.
    pub enum Time for "time", uom = UomTime {
        /// Nanosecond (`ns`).
        Nanosecond => { symbol: "ns"; coefficient: 1 / 1000000000; uom: nanosecond; }
        /// Microsecond (`µs`).
        Microsecond => { symbol: "µs"; coefficient: 1 / 1000000; aliases: ["us", "μs"]; uom: microsecond; }
        /// Millisecond (`ms`).
        Millisecond => { symbol: "ms"; coefficient: 1 / 1000; uom: millisecond; }
        /// Second (`s`).
        Second => { symbol: "s"; coefficient: 1; uom: second; }
        /// Minute (`min`).
        Minute => { symbol: "min"; coefficient: 60; uom: minute; }
        /// Hour (`h`).
        Hour => { symbol: "h"; coefficient: 3600; uom: hour; }
        /// Day (`d`).
        Day => { symbol: "d"; coefficient: 86400; uom: day; }
        /// Year (`a`).
        CommonYear365 => { symbol: "a (365 d)"; coefficient: 31536000; aliases: ["a", "yr", "year"]; uom: year; }
    }
}
