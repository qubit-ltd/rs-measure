// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Invalid external unit family used to verify fallible `uom` conversion.

use std::fmt;
use std::str::FromStr;

use qubit_measure::MeasurementError;
use qubit_measure::Unit;
use qubit_measure::UnitDefinition;
use qubit_measure::impl_uom_unit;
use uom::si::f64::Length as UomLength;
use uom::si::length::meter;

/// External unit whose definition deliberately reports a validation error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FallibleUomUnit {
    /// Unit with an invalid definition.
    Invalid,
}

impl Unit for FallibleUomUnit {
    const QUANTITY: &'static str = "fallible_uom";

    fn all() -> &'static [Self] {
        &[Self::Invalid]
    }

    fn symbol(self) -> &'static str {
        "invalid-uom"
    }

    fn aliases(self) -> &'static [&'static str] {
        &[]
    }

    fn definition(self) -> Result<UnitDefinition, MeasurementError> {
        Err(MeasurementError::InvalidUnitDefinition {
            reason: "fallible uom test definition".to_owned(),
        })
    }
}

impl fmt::Display for FallibleUomUnit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.symbol())
    }
}

impl FromStr for FallibleUomUnit {
    type Err = MeasurementError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::parse_strict(input)
    }
}

impl_uom_unit! {
    FallibleUomUnit, UomLength {
        base: meter;
    }
}
