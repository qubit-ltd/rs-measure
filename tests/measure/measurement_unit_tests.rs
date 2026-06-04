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
    LengthMeasurement,
    LengthUnit,
    MeasurementUnit,
};
use rust_decimal::Decimal;

#[test]
fn test_measurement_unit_trait_exposes_typed_quantity_metadata() {
    let measurement = LengthMeasurement::new(Decimal::new(50, 0), LengthUnit::Centimeter);

    assert_eq!(measurement.quantity_name(), "length");
    assert_eq!(LengthUnit::QUANTITY, "length");
    assert_eq!(LengthUnit::Meter.symbol(), "m");
}
