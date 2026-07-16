// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Manually implemented external unit family used by integration tests.

use std::fmt;
use std::str::FromStr;

use qubit_measure::{
    MeasurementError,
    Unit,
    UnitDefinition,
};

/// Minimal manually implemented external unit family.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ManualUnit {
    /// Base unit for the manual family.
    Base,
}

impl Unit for ManualUnit {
    const QUANTITY: &'static str = "manual";

    fn all() -> &'static [Self] {
        &[Self::Base]
    }

    fn symbol(self) -> &'static str {
        "manual"
    }

    fn aliases(self) -> &'static [&'static str] {
        &["mnl"]
    }

    fn definition(self) -> Result<UnitDefinition, MeasurementError> {
        Ok(UnitDefinition::base())
    }
}

impl fmt::Display for ManualUnit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.symbol())
    }
}

impl FromStr for ManualUnit {
    type Err = MeasurementError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::parse_lenient(input)
    }
}
