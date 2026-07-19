// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! External `uom` implementation with a fallible forward conversion.

use std::fmt;
use std::str::FromStr;

use qubit_measure::{
    MeasurementError,
    Unit,
    UnitDefinition,
    UomUnit,
};
use rust_decimal::{
    Decimal,
    prelude::{
        FromPrimitive,
        ToPrimitive,
    },
};
use uom::si::f64::Length as UomLength;
use uom::si::length::meter;

/// External unit implemented through only the fallible `UomUnit` contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TryOnlyUomUnit {
    /// Unit with a valid base definition.
    Valid,

    /// Unit whose definition reports a validation error.
    Invalid,
}

impl Unit for TryOnlyUomUnit {
    const QUANTITY: &'static str = "try_only_uom";

    fn all() -> &'static [Self] {
        &[Self::Valid, Self::Invalid]
    }

    fn symbol(self) -> &'static str {
        match self {
            Self::Valid => "try-only-uom",
            Self::Invalid => "invalid-try-only-uom",
        }
    }

    fn aliases(self) -> &'static [&'static str] {
        &[]
    }

    fn definition(self) -> Result<UnitDefinition, MeasurementError> {
        match self {
            Self::Valid => Ok(UnitDefinition::base()),
            Self::Invalid => Err(MeasurementError::InvalidUnitDefinition {
                reason: "try-only uom test definition".to_owned(),
            }),
        }
    }
}

impl UomUnit for TryOnlyUomUnit {
    type Quantity = UomLength;

    fn try_to_uom_approx(
        self,
        value: Decimal,
    ) -> Result<Self::Quantity, MeasurementError> {
        let _ = self.definition()?;
        Ok(UomLength::new::<meter>(
            value
                .to_f64()
                .expect("Decimal is representable as finite f64"),
        ))
    }

    fn value_from_uom_approx(
        self,
        quantity: Self::Quantity,
    ) -> Result<Decimal, MeasurementError> {
        let _ = self.definition()?;
        let value = quantity.get::<meter>();
        Decimal::from_f64(value).ok_or_else(|| {
            MeasurementError::DecimalConversion(value.to_string())
        })
    }
}

impl fmt::Display for TryOnlyUomUnit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.symbol())
    }
}

impl FromStr for TryOnlyUomUnit {
    type Err = MeasurementError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::parse_strict(input)
    }
}
