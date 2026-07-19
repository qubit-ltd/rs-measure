// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! External unit-family fixtures used by integration tests.

mod compact_ambiguity_unit;
mod custom_length;
mod decimal_conversion_unit;
#[cfg(feature = "uom")]
mod fallible_uom_unit;
mod manual_unit;
#[cfg(feature = "uom")]
mod try_only_uom_unit;

pub(crate) use compact_ambiguity_unit::CompactAmbiguityUnit;
pub(crate) use custom_length::CustomLength;
pub(crate) use decimal_conversion_unit::DecimalConversionUnit;
#[cfg(feature = "uom")]
pub(crate) use fallible_uom_unit::FallibleUomUnit;
pub(crate) use manual_unit::ManualUnit;
#[cfg(feature = "uom")]
pub(crate) use try_only_uom_unit::TryOnlyUomUnit;
