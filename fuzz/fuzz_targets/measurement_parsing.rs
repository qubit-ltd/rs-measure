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

use std::str;

use libfuzzer_sys::fuzz_target;
use qubit_measure::Measurement;
use qubit_measure::MeasurementError;
use qubit_measure::MeasurementParseOptions;
use qubit_measure::Unit;
use qubit_measure::unit;

fn assert_successful_parse<U: Unit>(result: Result<Measurement<U>, MeasurementError>) {
    if let Ok(original) = result {
        let text = original.to_string();
        let restored =
            Measurement::<U>::parse_strict(&text).expect("canonical display of a parsed measurement should parse");
        assert_eq!(restored.value, original.value);
        assert_eq!(restored.value.scale(), original.value.scale());
        assert!(restored.unit == original.unit, "round trip should preserve the unit");
    }
}

fuzz_target!(|data: &[u8]| {
    let Ok(input) = str::from_utf8(data) else {
        return;
    };
    let constrained = MeasurementParseOptions::default().with_max_text_bytes(data.len().saturating_div(2));

    assert_successful_parse(Measurement::<unit::Length>::parse_strict(input));
    assert_successful_parse(Measurement::<unit::Length>::parse_lenient(input));
    assert_successful_parse(Measurement::<unit::Length>::parse_strict_with_options(
        input,
        &constrained,
    ));
    assert_successful_parse(Measurement::<unit::Length>::parse_lenient_with_options(
        input,
        &constrained,
    ));
    assert_successful_parse(Measurement::<unit::Time>::parse_strict(input));
    assert_successful_parse(Measurement::<unit::Time>::parse_lenient(input));
    assert_successful_parse(Measurement::<unit::Temperature>::parse_strict(input));
    assert_successful_parse(Measurement::<unit::Temperature>::parse_lenient(input));
    assert_successful_parse(Measurement::<unit::Area>::parse_lenient(input));
    assert_successful_parse(Measurement::<unit::Volume>::parse_lenient(input));
    assert_successful_parse(Measurement::<unit::Velocity>::parse_lenient(input));
});
