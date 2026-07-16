// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Shared assertions for generated unit-family contracts.

use std::fmt::Debug;

use qubit_measure::Unit;
use serde::{
    Deserialize,
    Serialize,
};
use serde_json::json;

/// Checks canonical parsing, display, and Serde for every family member.
///
/// # Type Parameters
///
/// * `U` - Unit family whose complete canonical contract is checked.
///
/// # Panics
///
/// Panics if any canonical symbol fails to parse, display, serialize, or
/// deserialize back to its owning unit.
pub(crate) fn assert_unit_contract<U>()
where
    U: Unit + Serialize + for<'de> Deserialize<'de> + Debug,
{
    for unit in U::all() {
        assert_eq!(
            U::from_str(unit.symbol()).expect("unit symbol should parse"),
            *unit,
        );
        assert_eq!(unit.to_string(), unit.symbol());

        let value = serde_json::to_value(unit).expect("unit should serialize");

        assert_eq!(value, json!(unit.symbol()));
        assert_eq!(
            serde_json::from_value::<U>(value)
                .expect("unit should deserialize"),
            *unit,
        );
    }
}
