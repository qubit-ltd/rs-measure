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
    MassMeasurement,
    MassUnit,
};
use rust_decimal::Decimal;
use uom::si::length::meter;

/// Demonstrates persisted measurements and `uom` conversion.
fn main() -> Result<(), qubit_measure::MeasurementError> {
    let persisted = LengthMeasurement::new(Decimal::new(50, 0), LengthUnit::Centimeter);
    let meters = persisted.to_uom()?.get::<meter>();
    let kilograms = MassMeasurement::new(Decimal::new(1, 1), MassUnit::Gram).convert_to(MassUnit::Kilogram)?;

    println!("{persisted} = {meters} m");
    println!("{kilograms}");
    Ok(())
}
