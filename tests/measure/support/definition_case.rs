// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact unit-definition oracle case used by family tests.

/// One independently written exact unit-definition oracle.
pub(crate) struct DefinitionCase<U> {
    /// Unit being checked.
    pub(crate) unit: U,

    /// Expected canonical factor numerator.
    pub(crate) numerator: &'static str,

    /// Expected canonical factor denominator.
    pub(crate) denominator: &'static str,

    /// Expected pre-factor offset.
    pub(crate) offset: &'static str,
}
