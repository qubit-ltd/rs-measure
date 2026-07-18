// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact adapter tests between typed time measurements and `Duration`.

use std::time::Duration;

use qubit_measure::{
    Measurement,
    MeasurementError,
    unit::Time,
};
use rust_decimal::{
    Decimal,
    dec,
};

#[test]
fn test_duration_round_trip_is_exact() {
    let duration = Duration::new(12, 345_678_901);
    let measurement = Measurement::<Time>::from(duration);

    assert_eq!(measurement.unit, Time::Second);
    assert_eq!(measurement.value, dec!(12.345678901));
    assert_eq!(Duration::try_from(measurement), Ok(duration));
}

#[test]
fn test_duration_boundaries_round_trip_exactly() {
    for duration in [Duration::ZERO, Duration::MAX] {
        let measurement = Measurement::<Time>::from(duration);
        assert_eq!(Duration::try_from(measurement), Ok(duration));
    }
}

#[test]
fn test_time_units_convert_to_duration_exactly() {
    for (measurement, expected) in [
        (
            Measurement::new(Decimal::ONE, Time::Nanosecond),
            Duration::from_nanos(1),
        ),
        (
            Measurement::new(Decimal::ONE, Time::Microsecond),
            Duration::from_micros(1),
        ),
        (
            Measurement::new(Decimal::ONE, Time::Millisecond),
            Duration::from_millis(1),
        ),
        (
            Measurement::new(Decimal::ONE, Time::Minute),
            Duration::from_secs(60),
        ),
        (
            Measurement::new(Decimal::ONE, Time::Hour),
            Duration::from_secs(3_600),
        ),
        (
            Measurement::new(Decimal::ONE, Time::Day),
            Duration::from_secs(86_400),
        ),
    ] {
        assert_eq!(Duration::try_from(measurement), Ok(expected));
    }
}

#[test]
fn test_negative_time_cannot_convert_to_duration() {
    let measurement = Measurement::new(dec!(-1), Time::Second);

    assert_eq!(
        Duration::try_from(measurement),
        Err(MeasurementError::NegativeDuration {
            value: dec!(-1),
            unit: "s".to_owned(),
        }),
    );
}

#[test]
fn test_negative_zero_converts_to_zero_duration() {
    let mut negative_zero = Decimal::ZERO;
    negative_zero.set_sign_negative(true);
    assert!(negative_zero.is_sign_negative());
    let measurement = Measurement::new(negative_zero, Time::Second);

    assert_eq!(Duration::try_from(measurement), Ok(Duration::ZERO));
}

#[test]
fn test_subnanosecond_time_cannot_convert_to_duration() {
    let measurement = Measurement::new(dec!(0.1), Time::Nanosecond);

    assert_eq!(
        Duration::try_from(measurement),
        Err(MeasurementError::SubnanosecondDuration {
            value: dec!(0.1),
            unit: "ns".to_owned(),
        }),
    );
}

#[test]
fn test_time_above_duration_max_is_rejected() {
    let measurement = Measurement::new(Decimal::MAX, Time::Second);

    assert_eq!(
        Duration::try_from(measurement),
        Err(MeasurementError::DurationOutOfRange {
            value: Decimal::MAX,
            unit: "s".to_owned(),
        }),
    );
}

#[test]
fn test_nanoseconds_immediately_above_duration_max_are_rejected() {
    let value = dec!(18446744073709551616000000000);
    let measurement = Measurement::new(value, Time::Nanosecond);

    assert_eq!(
        Duration::try_from(measurement),
        Err(MeasurementError::DurationOutOfRange {
            value,
            unit: "ns".to_owned(),
        }),
    );
}
