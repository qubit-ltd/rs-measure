// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_measure::{
    Measurement,
    MeasurementError,
    Unit,
    UnitDefinition,
    assert_unit_family_valid,
};
use rust_decimal::dec;
use serde_json::json;

use crate::measure::fixtures::{
    CanonicalPriorityUnit,
    CustomLength,
    ManualUnit,
};

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
    assert_eq!("half-cu".parse(), Ok(CustomLength::Half));
    assert_eq!(CustomLength::Half.to_string(), "hcu");
}

#[test]
fn test_spaced_measurement_round_trips_reserved_unit_prefix() {
    let measurement = Measurement::new(dec!(1.25), CustomLength::Signed);
    let text = measurement.to_string();

    assert_eq!(text, "1.25 +cu");
    assert_eq!(text.parse::<Measurement<CustomLength>>(), Ok(measurement));
    assert_eq!(
        serde_json::from_value::<Measurement<CustomLength>>(
            serde_json::to_value(measurement)
                .expect("measurement should serialize"),
        )
        .expect("measurement should deserialize"),
        measurement,
    );
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
        serde_json::to_value(measurement)
            .expect("manual-unit measurement should serialize"),
        json!({"quantity": "manual", "value": "1.25", "unit": "manual"}),
    );

    let deserialized: Measurement<ManualUnit> = serde_json::from_value(json!({
        "quantity": "manual",
        "value": "1.25",
        "unit": "mnl",
    }))
    .expect("manual-unit measurement alias should deserialize");
    assert_eq!(deserialized, measurement);
}

#[test]
fn test_lenient_parsing_prefers_canonical_symbol_over_earlier_alias() {
    assert_eq!(
        CanonicalPriorityUnit::parse_lenient("canonical")
            .expect("canonical symbol should parse"),
        CanonicalPriorityUnit::CanonicalOwner,
    );
}

#[test]
fn test_external_unit_families_satisfy_metadata_contract() {
    assert_unit_family_valid::<CustomLength>();
    assert_unit_family_valid::<CanonicalPriorityUnit>();
    assert_unit_family_valid::<ManualUnit>();
}
