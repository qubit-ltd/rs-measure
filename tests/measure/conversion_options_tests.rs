// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::sync::{
    Arc,
    Barrier,
    Mutex,
};
use std::thread;

use qubit_measure::{
    ConversionOptions,
    MeasurementError,
    RoundingStrategy,
    default_conversion_options,
    measurement,
    set_default_conversion_options,
    unit,
};
use rust_decimal::dec;

static GLOBAL_OPTIONS_TEST_LOCK: Mutex<()> = Mutex::new(());

struct RestoreDefault(ConversionOptions);

impl Drop for RestoreDefault {
    fn drop(&mut self) {
        set_default_conversion_options(self.0);
    }
}

#[test]
fn test_conversion_options_reject_scale_above_decimal_limit() {
    assert_eq!(
        ConversionOptions::fixed_scale(
            29,
            RoundingStrategy::MidpointNearestEven
        ),
        Err(MeasurementError::InvalidScale { scale: 29, max: 28 }),
    );
}

#[test]
fn test_global_conversion_options_replace_and_restore() {
    let _test_lock = GLOBAL_OPTIONS_TEST_LOCK
        .lock()
        .expect("global options test lock should be available");
    let original = default_conversion_options();
    let _restore = RestoreDefault(original);
    let replacement = ConversionOptions::fixed_scale(
        6,
        RoundingStrategy::MidpointAwayFromZero,
    )
    .expect("scale should be valid");

    assert_eq!(set_default_conversion_options(replacement), original);
    assert_eq!(default_conversion_options(), replacement);
    assert_eq!(set_default_conversion_options(original), replacement);
}

#[test]
fn test_global_conversion_options_are_atomic_and_drive_default_conversion() {
    let _test_lock = GLOBAL_OPTIONS_TEST_LOCK
        .lock()
        .expect("global options test lock should be available");
    let original = default_conversion_options();
    let _restore = RestoreDefault(original);
    let first = ConversionOptions::fixed_scale(
        3,
        RoundingStrategy::MidpointAwayFromZero,
    )
    .expect("scale should be valid");
    let second = ConversionOptions::fixed_scale(7, RoundingStrategy::ToZero)
        .expect("scale should be valid");
    let barrier = Arc::new(Barrier::new(7));
    let mut handles = Vec::new();

    for options in [first, second] {
        let barrier = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            barrier.wait();
            for _ in 0..2_000 {
                set_default_conversion_options(options);
            }
        }));
    }
    for _ in 0..4 {
        let barrier = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            barrier.wait();
            for _ in 0..2_000 {
                let observed = default_conversion_options();
                assert!(
                    observed == original
                        || observed == first
                        || observed == second
                );
            }
        }));
    }
    barrier.wait();
    for handle in handles {
        handle.join().expect("options worker should not panic");
    }

    let process_default = ConversionOptions::fixed_scale(
        2,
        RoundingStrategy::MidpointNearestEven,
    )
    .expect("scale should be valid");
    set_default_conversion_options(process_default);
    let source = measurement::Length::new(dec!(1), unit::Length::Meter);
    let default_result = source
        .convert_to(unit::Length::Foot)
        .expect("default conversion should succeed");
    let explicit_result = source
        .convert_to_with_options(
            unit::Length::Foot,
            ConversionOptions::fixed_scale(
                4,
                RoundingStrategy::MidpointNearestEven,
            )
            .expect("scale should be valid"),
        )
        .expect("explicit conversion should succeed");

    assert_eq!(default_result.value, dec!(3.28));
    assert_eq!(default_result.value.scale(), 2);
    assert_eq!(explicit_result.value, dec!(3.2808));
    assert_eq!(explicit_result.value.scale(), 4);
}
