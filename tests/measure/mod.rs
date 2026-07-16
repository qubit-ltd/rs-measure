// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Measurement Module Tests
//!
//! Tests for persisted measurements and unit conversions.

mod fixtures;
mod support;

mod conversion_factor_tests;
mod conversion_options_tests;
mod decimal_conversion_tests;
mod external_unit_tests;
mod internal;
mod internal_tests;
mod measurement_error_tests;
mod measurement_tests;
mod mod_tests;
mod unit_definition_tests;
mod unit_tests;
mod units;
mod units_tests;
#[cfg(feature = "uom")]
mod uom_unit_tests;
