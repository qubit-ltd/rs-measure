// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Path mirror test for the `CatalyticActivity` measurement alias.

use crate::measurement::support::assert_measurement_alias;

assert_measurement_alias!(CatalyticActivity, CatalyticActivity);
