// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Fuzzes persisted measurement decoding and successful round trips.

#![no_main]

use libfuzzer_sys::fuzz_target;
use qubit_measure::Measurement;
use qubit_measure::unit;

fuzz_target!(|data: &[u8]| {
    if let Ok(measurement) = serde_json::from_slice::<Measurement<unit::Length>>(data) {
        let encoded = serde_json::to_vec(&measurement).expect("a valid length measurement should serialize");
        let decoded = serde_json::from_slice(&encoded).expect("a serialized length measurement should deserialize");
        assert_eq!(measurement, decoded);
    }

    if let Ok(measurement) = serde_json::from_slice::<Measurement<unit::Time>>(data) {
        let encoded = serde_json::to_vec(&measurement).expect("a valid time measurement should serialize");
        let decoded = serde_json::from_slice(&encoded).expect("a serialized time measurement should deserialize");
        assert_eq!(measurement, decoded);
    }
});
