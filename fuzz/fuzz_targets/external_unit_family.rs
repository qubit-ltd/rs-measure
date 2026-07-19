// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Fuzzes parsing for an external unit family with ambiguous numeric prefixes.

#![no_main]

use libfuzzer_sys::fuzz_target;
use qubit_measure::{
    Measurement,
    Unit,
    define_unit_family,
};

define_unit_family! {
    /// External family covering ordinary and numeric-prefix-sensitive symbols.
    pub enum FuzzUnit for "fuzz_unit" {
        /// Base unit with ASCII and Unicode aliases.
        Base => {
            symbol: "fu";
            coefficient: 1;
            aliases: ["fuzz-unit", "µfu"];
        }
        /// Unit whose canonical symbol begins with a decimal point.
        Dot => {
            symbol: ".fu";
            coefficient: 2;
            aliases: ["dot-fu"];
        }
        /// Unit whose canonical symbol begins with a plus sign.
        Plus => {
            symbol: "+fu";
            coefficient: 3;
            aliases: ["plus-fu"];
        }
        /// Unit whose canonical symbol begins with a minus sign.
        Minus => {
            symbol: "-fu";
            coefficient: 4;
            aliases: ["minus-fu"];
        }
    }
}

fuzz_target!(|data: &[u8]| {
    let Ok(input) = str::from_utf8(data) else {
        return;
    };

    let _ = FuzzUnit::parse_strict(input);
    let _ = FuzzUnit::parse_lenient(input);
    let _ = Measurement::<FuzzUnit>::parse_strict(input);
    let _ = Measurement::<FuzzUnit>::parse_lenient(input);
});
