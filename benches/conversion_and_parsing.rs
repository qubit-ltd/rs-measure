// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Microbenchmarks for Decimal conversion and measurement parsing.

mod support;

use std::hint::black_box;

use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use qubit_measure::ConversionOptions;
use qubit_measure::Measurement;
use qubit_measure::measurement;
use qubit_measure::unit;
use rust_decimal::Decimal;
use rust_decimal::RoundingStrategy;
use rust_decimal::dec;

use crate::support::equivalent_unit::EquivalentUnit;

/// Benchmarks exact, repeating, and affine Decimal conversions.
///
/// # Parameters
///
/// * `criterion` - Criterion registry that receives conversion benchmarks.
fn benchmark_conversions(criterion: &mut Criterion) {
    let meter = measurement::Length::new(dec!(1), unit::Length::Meter);
    let exact_meter = measurement::Length::new(dec!(381), unit::Length::Meter);
    let celsius =
        measurement::Temperature::new(dec!(37), unit::Temperature::Celsius);
    let equivalent =
        Measurement::new(dec!(12.3400), EquivalentUnit::DivideByPointOne);
    let fixed_scale = ConversionOptions::fixed_scale(
        6,
        RoundingStrategy::MidpointNearestEven,
    )
    .expect("benchmark output scale should be valid");
    let boundary =
        measurement::Length::new(Decimal::MAX, unit::Length::Nanometer);

    criterion.bench_function("conversion/same_unit", |bencher| {
        bencher.iter(|| {
            black_box(meter)
                .convert_to(unit::Length::Meter)
                .expect("same-unit benchmark conversion should succeed")
        });
    });
    criterion.bench_function("conversion/equivalent_definitions", |bencher| {
        bencher.iter(|| {
            black_box(equivalent)
                .convert_to(EquivalentUnit::Ten)
                .expect("equivalent definitions should preserve the value")
        });
    });
    criterion.bench_function("conversion/repeating_meter_to_foot", |bencher| {
        bencher.iter(|| {
            black_box(meter)
                .convert_to(unit::Length::Foot)
                .expect("benchmark conversion should fit Decimal")
        });
    });
    criterion.bench_function("conversion/exact_meter_to_foot", |bencher| {
        bencher.iter(|| {
            black_box(exact_meter)
                .convert_to(unit::Length::Foot)
                .expect("benchmark conversion should be exact")
        });
    });
    criterion.bench_function(
        "conversion/fixed_scale_meter_to_foot",
        |bencher| {
            bencher.iter(|| {
                black_box(meter)
                    .convert_to_with_options(unit::Length::Foot, fixed_scale)
                    .expect("fixed-scale benchmark conversion should succeed")
            });
        },
    );
    criterion.bench_function(
        "conversion/boundary_nanometer_to_meter",
        |bencher| {
            bencher.iter(|| {
                black_box(boundary)
                    .convert_to(unit::Length::Meter)
                    .expect("boundary benchmark conversion should fit Decimal")
            });
        },
    );
    criterion.bench_function(
        "conversion/affine_celsius_to_fahrenheit",
        |bencher| {
            bencher.iter(|| {
                black_box(celsius)
                    .convert_to(unit::Temperature::Fahrenheit)
                    .expect("benchmark conversion should fit Decimal")
            });
        },
    );
}

/// Benchmarks strict and lenient parsing in spaced and compact forms.
///
/// # Parameters
///
/// * `criterion` - Criterion registry that receives parsing benchmarks.
fn benchmark_parsing(criterion: &mut Criterion) {
    let long_spaced_input =
        format!("{}12.345 cm{}", " ".repeat(4_096), " ".repeat(4_096));

    criterion.bench_function("parsing/strict_compact", |bencher| {
        bencher.iter(|| {
            measurement::Length::parse_strict(black_box("12.345cm"))
                .expect("strict compact benchmark input should parse")
        });
    });
    criterion.bench_function("parsing/strict_spaced", |bencher| {
        bencher.iter(|| {
            measurement::Length::parse_strict(black_box("12.345 cm"))
                .expect("strict spaced benchmark input should parse")
        });
    });
    criterion.bench_function("parsing/lenient_compact_alias", |bencher| {
        bencher.iter(|| {
            measurement::Length::parse_lenient(black_box("12.345um"))
                .expect("lenient compact benchmark input should parse")
        });
    });
    criterion.bench_function("parsing/strict_spaced_long_padding", |bencher| {
        bencher.iter(|| {
            measurement::Length::parse_strict(black_box(
                long_spaced_input.as_str(),
            ))
            .expect("long padded benchmark input should parse")
        });
    });
}

criterion_group!(benches, benchmark_conversions, benchmark_parsing);
criterion_main!(benches);
