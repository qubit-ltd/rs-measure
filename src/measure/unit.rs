// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Traits shared by persisted measurement units.

use std::fmt;
use std::str::FromStr;

use crate::measure::{
    MeasurementError,
    UnitDefinition,
};

/// Exact metadata and Decimal conversion definition for one unit family.
pub trait Unit:
    Copy + Eq + fmt::Display + FromStr<Err = MeasurementError> + 'static
{
    /// Stable lower-case quantity identifier used in persistence and errors.
    const QUANTITY: &'static str;

    /// Returns all unit variants supported by this family.
    #[must_use]
    fn all() -> &'static [Self];

    /// Returns the canonical symbol used for display and serialization.
    #[must_use]
    fn symbol(self) -> &'static str;

    /// Returns accepted non-canonical aliases for lenient parsing.
    #[must_use]
    fn aliases(self) -> &'static [&'static str];

    /// Returns this unit's exact Decimal definition relative to its base unit.
    ///
    /// # Errors
    ///
    /// Returns [`MeasurementError::InvalidUnitDefinition`] when a manually
    /// implemented external unit cannot provide a valid definition.
    fn definition(self) -> Result<UnitDefinition, MeasurementError>;

    /// Parses only canonical unit symbols.
    ///
    /// # Errors
    ///
    /// Returns [`MeasurementError::NonCanonicalUnit`] for a recognized alias,
    /// or [`MeasurementError::UnknownUnit`] for an unrecognized symbol.
    fn parse_strict(input: &str) -> Result<Self, MeasurementError> {
        let input = input.trim();
        if let Some(unit) = Self::all()
            .iter()
            .copied()
            .find(|unit| unit.symbol() == input)
        {
            return Ok(unit);
        }
        if let Some(unit) = Self::all()
            .iter()
            .copied()
            .find(|unit| unit.aliases().contains(&input))
        {
            return Err(MeasurementError::NonCanonicalUnit {
                quantity: Self::QUANTITY.to_owned(),
                unit: input.to_owned(),
                canonical: unit.symbol().to_owned(),
            });
        }
        Err(MeasurementError::UnknownUnit {
            quantity: Self::QUANTITY.to_owned(),
            unit: input.to_owned(),
        })
    }

    /// Parses canonical symbols and documented aliases.
    ///
    /// # Errors
    ///
    /// Returns [`MeasurementError::UnknownUnit`] if the trimmed input is not
    /// recognized by this unit family.
    fn parse_lenient(input: &str) -> Result<Self, MeasurementError> {
        let input = input.trim();
        Self::all()
            .iter()
            .copied()
            .find(|unit| {
                unit.symbol() == input || unit.aliases().contains(&input)
            })
            .ok_or_else(|| MeasurementError::UnknownUnit {
                quantity: Self::QUANTITY.to_owned(),
                unit: input.to_owned(),
            })
    }
}
