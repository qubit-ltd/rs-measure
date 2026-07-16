// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! External unit-family fixtures used by integration tests.

mod canonical_priority_unit;
mod custom_length;
mod manual_unit;

pub(crate) use canonical_priority_unit::CanonicalPriorityUnit;
pub(crate) use custom_length::CustomLength;
pub(crate) use manual_unit::ManualUnit;
