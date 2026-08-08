// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Units for persisted information measurements.

#[cfg(feature = "uom")]
use uom::si::f64::Information as UomInformation;
#[cfg(feature = "uom")]
use uom::si::information::byte;

use crate::define_unit_family;
#[cfg(feature = "uom")]
use crate::impl_uom_unit;

define_unit_family! {
    /// Units for persisted information measurements.
    pub enum Information for "information" {
        /// Bit (`b`).
        Bit => { symbol: "b"; definition: crate::consts::information::BIT; }
        /// Byte (`B`).
        Byte => { symbol: "B"; definition: crate::consts::information::BYTE; }
        /// Kilobyte (`kB`).
        Kilobyte => { symbol: "kB"; definition: crate::consts::information::KILOBYTE; }
        /// Megabyte (`MB`).
        Megabyte => { symbol: "MB"; definition: crate::consts::information::MEGABYTE; }
        /// Gigabyte (`GB`).
        Gigabyte => { symbol: "GB"; definition: crate::consts::information::GIGABYTE; }
        /// Terabyte (`TB`).
        Terabyte => { symbol: "TB"; definition: crate::consts::information::TERABYTE; }
        /// Kibibyte (`KiB`).
        Kibibyte => { symbol: "KiB"; definition: crate::consts::information::KIBIBYTE; }
        /// Mebibyte (`MiB`).
        Mebibyte => { symbol: "MiB"; definition: crate::consts::information::MEBIBYTE; }
        /// Gibibyte (`GiB`).
        Gibibyte => { symbol: "GiB"; definition: crate::consts::information::GIBIBYTE; }
        /// Tebibyte (`TiB`).
        Tebibyte => { symbol: "TiB"; definition: crate::consts::information::TEBIBYTE; }
    }
}

#[cfg(feature = "uom")]
impl_uom_unit! {
    Information, UomInformation {
        base: byte;
    }
}
