// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Hidden helper re-export contract tests.

use qubit_measure::__private::{
    assert_unit_family_metadata,
    decimal_from_literal,
    is_ascii_snake_case,
};

#[test]
fn test_private_module_exposes_only_crate_owned_macro_support() {
    assert_eq!(decimal_from_literal("1.25").to_string(), "1.25");
    assert!(is_ascii_snake_case("unit_family"));
    assert_unit_family_metadata("unit_family", &["u"], &[]);
}
