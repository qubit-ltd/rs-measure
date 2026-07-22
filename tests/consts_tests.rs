// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact built-in conversion-factor tests.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use qubit_measure::{
    ConversionFactor,
    Unit,
    unit,
};
use rust_decimal::dec;
use serde_json::from_str;

#[test]
fn test_builtin_revolution_factor_uses_reduced_terms() {
    let builtin = unit::Angle::Revolution
        .definition()
        .expect("revolution definition should be valid")
        .factor();
    let normalized = ConversionFactor::new(
        dec!(39269908169872415480783),
        dec!(6250000000000000000000),
    )
    .expect("normalized revolution factor should be valid");

    assert_eq!(builtin, normalized);
}

#[test]
fn test_unit_definition_provenance_covers_every_builtin_unit() {
    let manifest_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("docs/unit-definition-provenance.tsv");
    let manifest = fs::read_to_string(&manifest_path)
        .expect("unit-definition provenance manifest should exist");
    let mut expected = BTreeSet::new();
    let mut current_quantity = None;
    for line in
        include_str!("measure/fixtures/unit_persistence_contract.txt").lines()
    {
        if let Some(quantity) = line
            .strip_prefix("quantity \"")
            .and_then(|quantity| quantity.strip_suffix('"'))
        {
            current_quantity = Some(quantity);
            continue;
        }
        let Some(unit) = line.strip_prefix("unit ") else {
            continue;
        };
        let (symbol, _) = unit
            .split_once(" aliases ")
            .expect("unit persistence contract should contain aliases");
        let symbol = from_str::<String>(symbol)
            .expect("unit symbol should be valid JSON text");
        let quantity = current_quantity
            .expect("unit persistence contract should declare its quantity");
        assert!(
            expected.insert((quantity.to_owned(), symbol)),
            "duplicate unit persistence contract entry for {quantity}",
        );
    }
    let mut lines = manifest
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'));

    assert_eq!(
        lines.next(),
        Some("quantity\tunit\tsource_ids\tnumeric_policy\tscope"),
    );

    let known_source_ids = BTreeSet::from([
        "BIPM-SI-9.3.01",
        "CODATA-2022-V9.0",
        "IEC-80000-13-2025",
        "NIST-DLMF-3.12",
        "NIST-HB-44-2026",
        "NIST-SP-811-2008",
    ]);
    let mut actual = BTreeSet::new();
    for (index, line) in lines.enumerate() {
        let fields = line.split('\t').collect::<Vec<_>>();
        assert_eq!(
            fields.len(),
            5,
            "manifest row {} must contain five tab-separated fields",
            index + 2,
        );
        let quantity = fields[0];
        let unit = fields[1];
        let source_ids = fields[2];
        let numeric_policy = fields[3];
        let scope = fields[4];
        assert!(
            actual.insert((quantity.to_owned(), unit.to_owned())),
            "duplicate provenance unit: {quantity} {unit}",
        );
        assert!(!unit.is_empty(), "missing unit for {quantity}");
        assert!(!source_ids.is_empty(), "missing sources for {quantity}");
        for source_id in source_ids.split(';') {
            assert!(
                known_source_ids.contains(source_id),
                "unknown source ID {source_id} for {quantity}",
            );
        }
        assert!(
            matches!(numeric_policy, "exact" | "mixed_exact_and_finite_pi"),
            "invalid numeric policy {numeric_policy} for {quantity}",
        );
        assert!(!scope.is_empty(), "missing provenance scope for {quantity}");
    }

    assert_eq!(actual, expected);
}
