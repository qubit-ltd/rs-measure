// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Private implementation types for persisted measurements.

mod conversion_mode;
mod decimal_text;
mod measurement_wire;

pub(super) use conversion_mode::ConversionMode;
pub(super) use decimal_text::parse_decimal_text_exact;
pub(super) use measurement_wire::MeasurementWire;
