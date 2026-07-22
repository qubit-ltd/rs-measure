// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact integer adapter tests for information measurements.

use qubit_measure::{
    Measurement,
    MeasurementError,
    unit::Information,
};
use rust_decimal::{
    Decimal,
    dec,
};

#[test]
fn test_information_constructs_u64_as_bytes() {
    for value in [0, 2_048, u64::MAX] {
        let measurement: Measurement<Information> = value.into();

        assert_eq!(
            measurement,
            Measurement::new(Decimal::from(value), Information::Byte),
        );
        assert_eq!(u64::try_from(measurement), Ok(value));
    }
}

#[test]
fn test_information_converts_exactly_to_u64_bytes() {
    for (measurement, expected) in [
        (Measurement::new(dec!(8), Information::Bit), 1),
        (Measurement::new(dec!(2), Information::Kibibyte), 2_048),
        (Measurement::new(dec!(2), Information::Kilobyte), 2_000),
    ] {
        assert_eq!(u64::try_from(measurement), Ok(expected));
    }
}

#[test]
fn test_information_converts_exactly_to_usize_bytes() {
    let measurement = Measurement::new(dec!(4), Information::Mebibyte);

    assert_eq!(usize::try_from(measurement), Ok(4 * 1_048_576));
}

#[test]
fn test_negative_information_cannot_convert_to_integer_bytes() {
    let measurement = Measurement::new(dec!(-1), Information::Byte);

    assert_eq!(
        u64::try_from(measurement),
        Err(MeasurementError::NegativeInformation {
            value: dec!(-1),
            unit: "B".to_owned(),
        }),
    );
}

#[test]
fn test_fractional_byte_information_cannot_convert_to_integer_bytes() {
    let measurement = Measurement::new(Decimal::ONE, Information::Bit);

    assert_eq!(
        u64::try_from(measurement),
        Err(MeasurementError::FractionalByteInformation {
            value: Decimal::ONE,
            unit: "b".to_owned(),
        }),
    );
}

#[test]
fn test_information_outside_u64_range_is_rejected() {
    let value = dec!(18446744073709551616);
    let measurement = Measurement::new(value, Information::Byte);

    assert_eq!(
        u64::try_from(measurement),
        Err(MeasurementError::InformationOutOfRange {
            value,
            unit: "B".to_owned(),
            target: "u64",
        }),
    );
}

#[test]
fn test_information_outside_usize_range_is_rejected() {
    let measurement = Measurement::new(Decimal::MAX, Information::Tebibyte);

    assert_eq!(
        usize::try_from(measurement),
        Err(MeasurementError::InformationOutOfRange {
            value: Decimal::MAX,
            unit: "TiB".to_owned(),
            target: "usize",
        }),
    );
}
