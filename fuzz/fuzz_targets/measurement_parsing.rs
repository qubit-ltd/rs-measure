// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Fuzzes strict and lenient measurement parsing across representative
//! families.

#![no_main]

use libfuzzer_sys::fuzz_target;
use qubit_measure::{
    Measurement,
    MeasurementParseOptions,
    unit,
};

fuzz_target!(|data: &[u8]| {
    let Ok(input) = str::from_utf8(data) else {
        return;
    };
    let constrained = MeasurementParseOptions::default()
        .with_max_text_bytes(data.len().saturating_div(2));

    let _ = Measurement::<unit::Length>::parse_strict(input);
    let _ = Measurement::<unit::Length>::parse_lenient(input);
    let _ = Measurement::<unit::Length>::parse_strict_with_options(
        input,
        &constrained,
    );
    let _ = Measurement::<unit::Length>::parse_lenient_with_options(
        input,
        &constrained,
    );
    let _ = Measurement::<unit::Time>::parse_strict(input);
    let _ = Measurement::<unit::Time>::parse_lenient(input);
    let _ = Measurement::<unit::Temperature>::parse_strict(input);
    let _ = Measurement::<unit::Temperature>::parse_lenient(input);
    let _ = Measurement::<unit::Area>::parse_lenient(input);
    let _ = Measurement::<unit::Volume>::parse_lenient(input);
    let _ = Measurement::<unit::Velocity>::parse_lenient(input);
});
