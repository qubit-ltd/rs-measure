// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact parsing for numeric literals captured by exported macros.

mod coefficient;
mod parser;
mod scanner;

pub use parser::decimal_from_literal;
