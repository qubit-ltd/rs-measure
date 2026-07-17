// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Configurable manual unit family used by metadata-validation tests.

use std::fmt;
use std::str::FromStr;

use qubit_measure::{
    MeasurementError,
    Unit,
    UnitDefinition,
};

/// Selects valid metadata.
pub(crate) const VALID: u8 = 0;

/// Selects a duplicated `all()` entry.
pub(crate) const DUPLICATE_ALL: u8 = 1;

/// Selects duplicated canonical symbols.
pub(crate) const DUPLICATE_SYMBOL: u8 = 2;

/// Selects duplicated aliases.
pub(crate) const DUPLICATE_ALIAS: u8 = 3;

/// Selects an invalid quantity identifier.
pub(crate) const INVALID_QUANTITY: u8 = 4;

/// Selects an invalid unit definition.
pub(crate) const INVALID_DEFINITION: u8 = 5;

/// Configurable manual unit family used by validation tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ManualValidationUnit<const CASE: u8> {
    /// Index of the represented test unit.
    index: u8,
}

impl<const CASE: u8> Unit for ManualValidationUnit<CASE> {
    const QUANTITY: &'static str = if CASE == INVALID_QUANTITY {
        "Invalid__Quantity"
    } else {
        "manual"
    };

    fn all() -> &'static [Self] {
        if CASE == DUPLICATE_ALL {
            &[Self { index: 0 }, Self { index: 0 }]
        } else {
            &[Self { index: 0 }, Self { index: 1 }]
        }
    }

    fn symbol(self) -> &'static str {
        match (CASE, self.index) {
            (DUPLICATE_SYMBOL, _) => "duplicate",
            (_, 0) => "base",
            _ => "derived",
        }
    }

    fn aliases(self) -> &'static [&'static str] {
        match (CASE, self.index) {
            (DUPLICATE_ALIAS, _) => &["duplicate-alias"],
            (_, 0) => &["b"],
            _ => &["d"],
        }
    }

    fn definition(self) -> Result<UnitDefinition, MeasurementError> {
        if CASE == INVALID_DEFINITION && self.index != 0 {
            Err(MeasurementError::InvalidUnitDefinition {
                reason: "test definition".to_owned(),
            })
        } else {
            Ok(UnitDefinition::base())
        }
    }
}

impl<const CASE: u8> fmt::Display for ManualValidationUnit<CASE> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.symbol())
    }
}

impl<const CASE: u8> FromStr for ManualValidationUnit<CASE> {
    type Err = MeasurementError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::parse_lenient(input)
    }
}
