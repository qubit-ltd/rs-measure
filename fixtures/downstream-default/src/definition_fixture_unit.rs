// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Unit family backed by an external const definition path.

use qubit_measure::{
    ConversionFactor,
    UnitDefinition,
    define_unit_family,
};
use rust_decimal::Decimal;

/// Non-identity definition constructed entirely in an external const context.
const RATIONAL_DEFINITION: UnitDefinition = UnitDefinition::new(
    ConversionFactor::from_const_integers(4, 6),
    Decimal::ZERO,
);

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
