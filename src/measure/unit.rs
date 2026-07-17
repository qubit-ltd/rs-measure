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
///
/// Implementations must satisfy this contract:
///
/// - [`Unit::QUANTITY`] is non-empty ASCII `snake_case`, starts with a
///   lowercase letter, and has no leading, trailing, or repeated underscores;
/// - [`Unit::all`] is non-empty, contains every family member exactly once, and
///   contains no duplicate value;
/// - canonical symbols are non-empty, unique, and contain no leading or
///   trailing Unicode whitespace;
/// - aliases are non-empty, unique among aliases, and contain no leading or
///   trailing Unicode whitespace;
/// - an alias may equal another member's canonical symbol, but canonical
///   symbols are searched first and therefore win;
/// - every member supplies a valid exact definition and obeys the documented
///   strict and lenient parsing behavior.
///
/// [`crate::define_unit_family!`] checks the statically expressible metadata
/// rules at compilation. Manual implementations should call
/// [`assert_unit_family_valid`] in tests. Stable Rust cannot prove that a
/// manual enum omitted no variant from [`Unit::all`].
pub trait Unit:
    Copy + Eq + fmt::Display + FromStr<Err = MeasurementError> + 'static
{
    /// Stable quantity identifier used in persistence and errors.
    ///
    /// This value must be non-empty ASCII `snake_case`, start with a lowercase
    /// letter, and contain no leading, trailing, or repeated underscores.
    const QUANTITY: &'static str;

    /// Returns all unit variants supported by this family.
    ///
    /// The slice must be non-empty and contain every family member exactly
    /// once. Stable Rust cannot verify completeness for a manual enum.
    ///
    /// # Returns
    ///
    /// A stable slice containing each supported unit exactly once.
    #[must_use]
    fn all() -> &'static [Self];

    /// Returns the canonical symbol used for display and serialization.
    ///
    /// Canonical symbols must be non-empty, unique within the family, and
    /// contain no leading or trailing Unicode whitespace.
    ///
    /// # Arguments
    ///
    /// * `self` - Unit whose canonical symbol is requested.
    ///
    /// # Returns
    ///
    /// The unit's non-empty canonical symbol.
    #[must_use]
    fn symbol(self) -> &'static str;

    /// Returns accepted non-canonical aliases for lenient parsing.
    ///
    /// Aliases must be non-empty, unique among all family aliases, and contain
    /// no leading or trailing Unicode whitespace. An alias may equal another
    /// member's canonical symbol; that canonical owner wins.
    ///
    /// # Arguments
    ///
    /// * `self` - Unit whose non-canonical aliases are requested.
    ///
    /// # Returns
    ///
    /// A stable slice of accepted lenient aliases.
    #[must_use]
    fn aliases(self) -> &'static [&'static str];

    /// Returns this unit's exact Decimal definition relative to its base unit.
    ///
    /// # Arguments
    ///
    /// * `self` - Unit whose exact definition is requested.
    ///
    /// # Returns
    ///
    /// The validated exact definition relative to the family base unit.
    ///
    /// # Errors
    ///
    /// Returns [`MeasurementError::InvalidUnitDefinition`] when a manually
    /// implemented external unit cannot provide a valid definition.
    fn definition(self) -> Result<UnitDefinition, MeasurementError>;

    /// Parses only canonical unit symbols.
    ///
    /// Canonical symbols are checked before aliases.
    ///
    /// # Arguments
    ///
    /// * `input` - Canonical symbol candidate; surrounding whitespace is
    ///   ignored.
    ///
    /// # Returns
    ///
    /// The unit that owns the matching canonical symbol.
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
    /// Canonical symbols are searched first, so they win over an earlier
    /// unit variant that declares the same text as an alias.
    ///
    /// # Arguments
    ///
    /// * `input` - Canonical symbol or alias candidate; surrounding whitespace
    ///   is ignored.
    ///
    /// # Returns
    ///
    /// The canonical owner when `input` is a canonical symbol, otherwise the
    /// unit that owns the matching alias.
    ///
    /// # Errors
    ///
    /// Returns [`MeasurementError::UnknownUnit`] if the trimmed input is not
    /// recognized by this unit family.
    fn parse_lenient(input: &str) -> Result<Self, MeasurementError> {
        let input = input.trim();
        if let Some(unit) = Self::all()
            .iter()
            .copied()
            .find(|unit| unit.symbol() == input)
        {
            return Ok(unit);
        }
        Self::all()
            .iter()
            .copied()
            .find(|unit| unit.aliases().contains(&input))
            .ok_or_else(|| MeasurementError::UnknownUnit {
                quantity: Self::QUANTITY.to_owned(),
                unit: input.to_owned(),
            })
    }
}

/// Asserts the observable metadata contract of a [`Unit`] implementation.
///
/// Canonical symbols and aliases are unique within their own sets. An alias may
/// equal a canonical symbol; canonical symbols are parsed first and therefore
/// win. Macro-generated families enforce the same metadata rules at compile
/// time. Manual implementations should call this assertion from their tests.
/// Stable Rust cannot prove that a manual enum omitted no variant from
/// [`Unit::all`].
///
/// # Type Parameters
///
/// * `U` - Unit family whose observable metadata and parsing are validated.
///
/// # Panics
///
/// Panics if the family is empty, its quantity is not non-empty ASCII
/// `snake_case`, `all()` repeats an entry, a canonical symbol or alias is
/// empty, contains surrounding Unicode whitespace, or is duplicated within
/// its own set, a definition is invalid, or strict or lenient parsing violates
/// the documented canonical-priority contract.
///
/// # Examples
///
/// ```
/// use qubit_measure::{
///     assert_unit_family_valid,
///     unit,
/// };
///
/// assert_unit_family_valid::<unit::Length>();
/// ```
#[track_caller]
pub fn assert_unit_family_valid<U>()
where
    U: Unit,
{
    let units = U::all();
    assert!(!units.is_empty(), "unit family must not be empty");
    assert!(
        crate::__private::is_ascii_snake_case(U::QUANTITY),
        "unit quantity must be non-empty ASCII snake_case",
    );

    for (index, unit) in units.iter().copied().enumerate() {
        assert!(
            !units[..index].contains(&unit),
            "duplicate all() entry at index {index}",
        );
        let symbol = unit.symbol();
        assert!(!symbol.is_empty(), "canonical symbol must not be empty");
        assert!(
            symbol.trim() == symbol,
            "canonical symbol must not contain surrounding whitespace: {symbol:?}",
        );
        assert!(
            !units[..index].iter().any(|other| other.symbol() == symbol),
            "duplicate canonical symbol: {symbol}",
        );
        let _ = unit.definition().unwrap_or_else(|error| {
            panic!("invalid definition for {symbol}: {error}")
        });
        assert!(
            U::parse_strict(symbol) == Ok(unit),
            "strict canonical parse failed for {symbol}",
        );
        assert!(
            U::parse_lenient(symbol) == Ok(unit),
            "lenient canonical parse failed for {symbol}",
        );
    }

    let mut seen_aliases: Vec<&str> = Vec::new();
    for unit in units.iter().copied() {
        for &alias in unit.aliases() {
            assert!(!alias.is_empty(), "unit alias must not be empty");
            assert!(
                alias.trim() == alias,
                "unit alias must not contain surrounding whitespace: {alias:?}",
            );
            assert!(!seen_aliases.contains(&alias), "duplicate alias: {alias}");
            seen_aliases.push(alias);

            if let Some(owner) = units
                .iter()
                .copied()
                .find(|candidate| candidate.symbol() == alias)
            {
                assert!(
                    U::parse_strict(alias) == Ok(owner),
                    "canonical owner must win strict parsing for {alias}",
                );
                assert!(
                    U::parse_lenient(alias) == Ok(owner),
                    "canonical owner must win lenient parsing for {alias}",
                );
            } else {
                assert!(
                    matches!(
                        U::parse_strict(alias),
                        Err(MeasurementError::NonCanonicalUnit {
                            canonical,
                            ..
                        }) if canonical == unit.symbol()
                    ),
                    "strict parsing must reject alias {alias}",
                );
                assert!(
                    U::parse_lenient(alias) == Ok(unit),
                    "lenient alias parse failed for {alias}",
                );
            }
        }
    }
}
