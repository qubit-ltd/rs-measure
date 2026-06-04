/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_measure::{
    Unit,
    measurement,
    unit,
};
use rust_decimal::Decimal;

#[test]
fn test_unit_trait_exposes_typed_quantity_metadata() {
    let measurement = measurement::Length::new(Decimal::new(50, 0), unit::Length::Centimeter);

    assert_eq!(measurement.quantity_name(), "length");
    assert_eq!(unit::Length::QUANTITY, "length");
    assert_eq!(unit::Length::Meter.symbol(), "m");
}
