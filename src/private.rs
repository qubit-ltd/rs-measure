// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Implementation dependencies used by exported declarative macros.

#[path = "private/decimal_literal/mod.rs"]
mod decimal_literal;
#[path = "private/unit_metadata.rs"]
mod unit_metadata;
#[cfg(feature = "uom")]
#[path = "private/uom_bridge.rs"]
mod uom_bridge;

pub use decimal_literal::decimal_from_literal;
pub use rust_decimal;
pub use serde;
pub use unit_metadata::{
    assert_unit_family_metadata,
    is_ascii_snake_case,
};
#[cfg(feature = "uom")]
pub use uom;
#[cfg(feature = "uom")]
pub use uom_bridge::{
    decimal_from_f64_approx,
    decimal_to_f64_approx,
};
