// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_measure::Measurement;
use qubit_measure::MeasurementError;
use qubit_measure::Unit;
use qubit_measure::UnitDefinition;
use qubit_measure::assert_unit_family_valid;
use rust_decimal::dec;
use serde_json::from_value;
use serde_json::json;
use serde_json::to_value;

use crate::measure::fixtures::CustomLength;
use crate::measure::fixtures::ManualUnit;

#[test]
fn test_external_family_supports_strict_and_lenient_parsing() {
    assert_eq!(
        CustomLength::parse_strict("hcu").expect("canonical unit should parse"),
        CustomLength::Half,
    );
    assert!(matches!(
        CustomLength::parse_strict("half-cu"),
        Err(MeasurementError::NonCanonicalUnit { .. }),
    ));
    assert_eq!(CustomLength::parse_lenient("half-cu"), Ok(CustomLength::Half));
    assert!(matches!(
        "half-cu".parse::<CustomLength>(),
        Err(MeasurementError::NonCanonicalUnit { .. }),
    ));
    assert_eq!(CustomLength::Half.to_string(), "hcu");
}

#[test]
fn test_spaced_measurement_round_trips_reserved_unit_prefixes() {
    for (unit, symbol) in [
        (CustomLength::Dot, ".cu"),
        (CustomLength::Signed, "+cu"),
        (CustomLength::Negative, "-cu"),
    ] {
        let measurement = Measurement::new(dec!(1.25), unit);
        let text = measurement.to_string();

        assert_eq!(text, format!("1.25 {symbol}"));
        assert_eq!(text.parse::<Measurement<CustomLength>>(), Ok(measurement),);
        assert_eq!(
            from_value::<Measurement<CustomLength>>(to_value(measurement).expect("measurement should serialize"),)
                .expect("measurement should deserialize"),
            measurement,
        );
    }
}

#[test]
fn test_compact_measurement_rejects_reserved_unit_prefixes() {
    for input in ["1.cu", "1+cu", "1-cu"] {
        assert!(
            matches!(
                input.parse::<Measurement<CustomLength>>(),
                Err(MeasurementError::InvalidMeasurementSyntax),
            ),
            "accepted ambiguous compact measurement {input:?}",
        );
    }
}

#[test]
fn test_unit_trait_supports_manual_external_implementations() {
    assert_eq!(ManualUnit::parse_lenient("mnl"), Ok(ManualUnit::Base));
    assert_eq!(
        ManualUnit::Base
            .definition()
            .expect("manual definition should be valid"),
        UnitDefinition::base(),
    );
}

#[test]
fn test_measurement_serde_uses_manual_unit_contract() {
    let measurement = Measurement::new(dec!(1.25), ManualUnit::Base);

    assert_eq!(
        to_value(measurement).expect("manual-unit measurement should serialize"),
        json!({"quantity": "manual", "value": "1.25", "unit": "manual"}),
    );

    let error = from_value::<Measurement<ManualUnit>>(json!({
        "quantity": "manual",
        "value": "1.25",
        "unit": "mnl",
    }))
    .expect_err("manual-unit measurement alias should be rejected");
    assert!(error.to_string().contains("non-canonical"));
}

#[test]
fn test_external_unit_families_satisfy_metadata_contract() {
    assert_unit_family_valid::<CustomLength>();
    assert_unit_family_valid::<ManualUnit>();
}
