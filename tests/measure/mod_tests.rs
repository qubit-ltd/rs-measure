// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Public core-type aggregation tests for the measure module.

use qubit_measure::{
    ConversionFactor,
    ConversionOptions,
    Measurement,
    MeasurementError,
    MeasurementParseOptions,
    Unit,
    UnitDefinition,
    unit,
};
use rust_decimal::Decimal;

/// Verifies that all public core measurement types remain available together.
#[test]
fn test_measure_module_core_types_are_reexported() {
    let definition: UnitDefinition = unit::Length::Meter
        .definition()
        .expect("meter definition should be valid");
    let factor: ConversionFactor = definition.factor();
    let options = ConversionOptions::default();
    let parse_options = MeasurementParseOptions::default();
    let result: Result<Measurement<unit::Length>, MeasurementError> =
        Measurement::new(Decimal::ONE, unit::Length::Meter)
            .convert_to(unit::Length::Meter);

    assert_eq!(factor.numerator(), Decimal::ONE);
    assert_eq!(options, ConversionOptions::DEFAULT);
    assert_eq!(
        parse_options.max_text_bytes(),
        MeasurementParseOptions::DEFAULT_MAX_TEXT_BYTES,
    );
    assert_eq!(
        result.expect("measurement should be available").value,
        Decimal::ONE,
    );
}
