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
mod failing_writer;
mod manual_validation_unit;
mod rational_oracle;
mod unit_assertions;

pub(crate) use definition_assertions::assert_definition_cases;
pub(crate) use definition_case::DefinitionCase;
pub(crate) use failing_writer::FailingWriter;
pub(crate) use manual_validation_unit::CANONICAL_ALIAS;
pub(crate) use manual_validation_unit::DISPLAY_MISMATCH;
pub(crate) use manual_validation_unit::DUPLICATE_ALIAS;
pub(crate) use manual_validation_unit::DUPLICATE_ALL;
pub(crate) use manual_validation_unit::DUPLICATE_SYMBOL;
pub(crate) use manual_validation_unit::INVALID_DEFINITION;
pub(crate) use manual_validation_unit::INVALID_QUANTITY;
pub(crate) use manual_validation_unit::LENIENT_FROM_STR;
pub(crate) use manual_validation_unit::ManualValidationUnit;
pub(crate) use manual_validation_unit::SELF_ALIAS;
pub(crate) use manual_validation_unit::VALID;
pub(crate) use rational_oracle::decimal_as_rational;
pub(crate) use rational_oracle::expected_conversion;
pub(crate) use rational_oracle::round_rational;
pub(crate) use unit_assertions::assert_unit_contract;
