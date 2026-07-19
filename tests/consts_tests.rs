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
fn test_unit_definition_provenance_covers_every_builtin_quantity() {
    let manifest_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("docs/unit-definition-provenance.tsv");
    let manifest = fs::read_to_string(&manifest_path)
        .expect("unit-definition provenance manifest should exist");
    let expected =
        include_str!("measure/fixtures/unit_persistence_contract.txt")
            .lines()
            .filter_map(|line| {
                line.strip_prefix("quantity \"")
                    .and_then(|quantity| quantity.strip_suffix('"'))
            })
            .collect::<BTreeSet<_>>();
    let mut lines = manifest
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'));

    assert_eq!(
        lines.next(),
        Some("quantity\tsource_ids\tnumeric_policy\tscope"),
    );

    let known_source_ids = BTreeSet::from([
        "BIPM-SI-9.3.01",
        "CODATA-2022-V9.0",
        "NIST-DLMF-3.12",
        "NIST-HB-44-2026",
        "NIST-SP-811-2008",
    ]);
    let mut actual = BTreeSet::new();
    for (index, line) in lines.enumerate() {
        let fields = line.split('\t').collect::<Vec<_>>();
        assert_eq!(
            fields.len(),
            4,
            "manifest row {} must contain four tab-separated fields",
            index + 2,
        );
        let quantity = fields[0];
        let source_ids = fields[1];
        let numeric_policy = fields[2];
        let scope = fields[3];
        assert!(
            actual.insert(quantity),
            "duplicate provenance quantity: {quantity}",
        );
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
