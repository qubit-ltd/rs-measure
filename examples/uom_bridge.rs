// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_measure::measurement;
use qubit_measure::unit;
use rust_decimal::Decimal;
use uom::si::length::meter;

/// Demonstrates the explicitly enabled approximate `uom` bridge.
fn main() {
    let value = measurement::Length::new(Decimal::new(50, 0), unit::Length::Centimeter);
    assert_eq!(value.to_uom_approx().get::<meter>(), 0.5);
}
