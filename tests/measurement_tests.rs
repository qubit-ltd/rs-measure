// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted measurement module aggregation tests.

use qubit_measure::Measurement;
use qubit_measure::measurement;
use qubit_measure::unit;
use rust_decimal::Decimal;

/// Verifies that the public measurement module aggregates the Length alias.
#[test]
fn test_measurement_module_aggregates_length_alias() {
    let value: measurement::Length = Measurement::new(Decimal::ONE, unit::Length::Meter);

    assert_eq!(value.value, Decimal::ONE);
}
