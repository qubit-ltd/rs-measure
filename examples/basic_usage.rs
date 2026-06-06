// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_measure::{
    measurement,
    unit,
};
use rust_decimal::Decimal;
use uom::si::length::meter;

/// Demonstrates persisted measurements and `uom` conversion.
fn main() -> Result<(), qubit_measure::MeasurementError> {
    let persisted =
        measurement::Length::new(Decimal::new(50, 0), unit::Length::Centimeter);
    let meters = persisted.to_uom().get::<meter>();
    let kilograms =
        measurement::Mass::new(Decimal::new(1, 1), unit::Mass::Gram)
            .convert_to(unit::Mass::Kilogram)?;

    println!("{persisted} = {meters} m");
    println!("{kilograms}");
    Ok(())
}
