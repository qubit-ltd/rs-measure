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
    rust_decimal,
    serde,
};

#[test]
fn test_private_module_preserves_macro_expansion_reexports() {
    assert_eq!(decimal_from_literal("1.25").to_string(), "1.25");
    assert!(is_ascii_snake_case("unit_family"));
    assert_unit_family_metadata("unit_family", &["u"], &[]);
    assert_eq!(
        std::any::type_name::<rust_decimal::Decimal>(),
        "rust_decimal::decimal::Decimal",
    );
    assert_eq!(
        std::any::type_name::<serde::de::value::Error>(),
        "serde_core::de::value::Error",
    );
}
