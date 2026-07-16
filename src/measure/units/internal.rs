// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Private `uom` units that preserve persisted Decimal semantics.

mod exact_torr_equivalent;

pub(super) use exact_torr_equivalent::exact_torr_equivalent as ExactTorrEquivalent;
