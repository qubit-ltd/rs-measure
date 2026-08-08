// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted time measurements.

#[cfg(feature = "uom")]
use uom::si::f64::Time as UomTime;
#[cfg(feature = "uom")]
use uom::si::time::second;

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;

define_unit_family! {
    /// Units for persisted time measurements.
    pub enum Time for "time" {
        /// Nanosecond (`ns`).
        Nanosecond => { symbol: "ns"; definition: crate::consts::time::NANOSECOND; }
        /// Microsecond (`µs`).
        Microsecond => { symbol: "µs"; definition: crate::consts::time::MICROSECOND; aliases: ["us", "μs"]; }
        /// Millisecond (`ms`).
        Millisecond => { symbol: "ms"; definition: crate::consts::time::MILLISECOND; }
        /// Second (`s`).
        Second => { symbol: "s"; definition: crate::consts::time::SECOND; }
        /// Minute (`min`).
        Minute => { symbol: "min"; definition: crate::consts::time::MINUTE; aliases: ["m"]; }
        /// Hour (`h`).
        Hour => { symbol: "h"; definition: crate::consts::time::HOUR; }
        /// Day (`d`).
        Day => { symbol: "d"; definition: crate::consts::time::DAY; }
        /// Common 365-day year with canonical symbol `a (365 d)`.
        CommonYear365 => { symbol: "a (365 d)"; definition: crate::consts::time::COMMON_YEAR365; aliases: ["a", "yr", "year"]; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    Time, UomTime {
        base: second;
    }
}
