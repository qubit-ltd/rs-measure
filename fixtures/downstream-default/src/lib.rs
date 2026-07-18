// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Default-feature downstream macro fixture.

use qubit_measure::{
    ConversionFactor,
    Decimal,
    UnitDefinition,
    define_unit_family,
};

/// Non-identity definition constructed entirely in an external const context.
const RATIONAL_DEFINITION: UnitDefinition = UnitDefinition::new(
    ConversionFactor::from_const_integers(4, 6),
    Decimal::ZERO,
);

define_unit_family! {
    /// Exact-only unit family that remains independent of optional features.
    pub enum DefaultFixtureUnit for "default_fixture" {
        /// Base fixture unit.
        Base => {
            symbol: "dfu";
            coefficient: 1;
        }
        /// Integer coefficient fixture unit.
        Integer => {
            symbol: "integer";
            coefficient: 42;
        }
        /// Decimal coefficient fixture unit.
        Decimal => {
            symbol: "decimal";
            coefficient: 1.25;
        }
        /// Negative offset fixture unit.
        NegativeOffset => {
            symbol: "negative-offset";
            coefficient: 2;
            offset: -273.15;
        }
        /// Scientific-notation coefficient fixture unit.
        Scientific => {
            symbol: "scientific";
            coefficient: 1.234_5e-6;
        }
        /// Digit-separated coefficient fixture unit.
        DigitSeparated => {
            symbol: "digit-separated";
            coefficient: 1_234_567;
        }
        /// Radix integer coefficient fixture unit.
        RadixInteger => {
            symbol: "radix-integer";
            coefficient: 0x1_FF;
        }
        /// Radix integer ratio fixture unit.
        RadixRatio => {
            symbol: "radix-ratio";
            coefficient: 0o20 / 0b10;
        }
    }
}

define_unit_family! {
    /// Exact-definition family declared entirely by reusable paths.
    pub enum DefinitionFixtureUnit for "definition_fixture" {
        /// Reusable exact definition supplied through a path.
        ExactDefinition => {
            symbol: "exact-definition";
            definition: RATIONAL_DEFINITION;
        }
    }
}
