// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Shared support types and assertions for measurement integration tests.

mod definition_assertions;
mod definition_case;
mod manual_validation_unit;
mod unit_assertions;

pub(crate) use definition_assertions::assert_definition_cases;
pub(crate) use definition_case::DefinitionCase;
pub(crate) use manual_validation_unit::{
    DUPLICATE_ALIAS,
    DUPLICATE_ALL,
    DUPLICATE_SYMBOL,
    INVALID_DEFINITION,
    INVALID_QUANTITY,
    ManualValidationUnit,
    VALID,
};
pub(crate) use unit_assertions::assert_unit_contract;
