// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use proptest::prop_assert_eq;
use proptest::proptest;
use qubit_measure::ConversionFactor;
use qubit_measure::ConversionOptions;
use qubit_measure::Measurement;
use qubit_measure::MeasurementError;
use qubit_measure::Unit;
use rust_decimal::Decimal;
use rust_decimal::RoundingStrategy;
use rust_decimal::dec;

use super::fixtures::DecimalConversionUnit;
use super::support::decimal_as_rational;
use super::support::expected_conversion;
use super::support::round_rational;

const ROUNDING_STRATEGIES: [RoundingStrategy; 7] = [
    RoundingStrategy::MidpointNearestEven,
    RoundingStrategy::MidpointAwayFromZero,
    RoundingStrategy::MidpointTowardZero,
    RoundingStrategy::ToZero,
    RoundingStrategy::AwayFromZero,
    RoundingStrategy::ToNegativeInfinity,
    RoundingStrategy::ToPositiveInfinity,
];

/// Converts a synthetic typed measurement through the public API.
fn convert(
    value: Decimal,
    source: DecimalConversionUnit,
    target: DecimalConversionUnit,
    options: ConversionOptions,
) -> Result<Decimal, MeasurementError> {
    Measurement::new(value, source)
        .convert_to_with_options(target, options)
        .map(|measurement| measurement.value)
}

#[test]
fn test_decimal_conversion_keeps_five_ninths_as_a_ratio() {
    let options = ConversionOptions::maximum_precision();

    assert_eq!(
        convert(
            dec!(32),
            DecimalConversionUnit::Fahrenheit,
            DecimalConversionUnit::Base,
            options,
        )
        .expect("Fahrenheit should convert to base units"),
        dec!(273.15),
    );
}

#[test]
fn test_maximum_precision_rounds_repeating_result_to_nearest_even() {
    let converted = convert(
        dec!(1),
        DecimalConversionUnit::TwoThirds,
        DecimalConversionUnit::Base,
        ConversionOptions::maximum_precision(),
    )
    .expect("repeating result should round to Decimal precision");

    assert_eq!(converted, dec!(0.6666666666666666666666666667));
    assert_eq!(converted.scale(), Decimal::MAX_SCALE);
}

#[test]
fn test_decimal_conversion_applies_requested_scale() {
    let options = ConversionOptions::fixed_scale(
        4,
        RoundingStrategy::MidpointNearestEven,
    )
    .expect("scale should be valid");

    let result = convert(
        dec!(1),
        DecimalConversionUnit::Base,
        DecimalConversionUnit::Foot,
        options,
    )
    .expect("meter should convert to foot");
    assert_eq!(result, dec!(3.2808));
    assert_eq!(result.scale(), 4);
}

/// Verifies that a representable result survives a larger intermediate product.
#[test]
fn test_decimal_conversion_avoids_intermediate_ratio_rounding() {
    let value =
        Decimal::try_from_i128_with_scale(Decimal::MAX.mantissa() - 3, 1)
            .expect("boundary input should fit Decimal");

    assert_eq!(
        convert(
            value,
            DecimalConversionUnit::TwoThirds,
            DecimalConversionUnit::Base,
            ConversionOptions::maximum_precision(),
        ),
        Ok(dec!(5281877500950955839569596688.8)),
    );
}

#[test]
fn test_fixed_scale_rounding_matches_independent_rational_oracle() {
    let source = DecimalConversionUnit::Base;
    let target = DecimalConversionUnit::Foot;
    let source_definition = source
        .definition()
        .expect("source definition should be valid");
    let target_definition = target
        .definition()
        .expect("target definition should be valid");

    for value in [
        dec!(1),
        dec!(-1),
        dec!(0.1524),
        dec!(-0.1524),
        dec!(0.4572),
        dec!(-0.4572),
    ] {
        let exact =
            expected_conversion(value, source_definition, target_definition);
        for strategy in ROUNDING_STRATEGIES {
            let options = ConversionOptions::fixed_scale(0, strategy)
                .expect("scale should be valid");
            let actual = convert(value, source, target, options)
                .expect("selected conversion should fit Decimal");
            let expected = round_rational(&exact, 0, strategy);

            assert_eq!(
                actual, expected,
                "value={value}, strategy={strategy:?}"
            );
            assert_eq!(actual.scale(), 0);
        }
    }
}

#[test]
fn test_affine_fixed_scale_rounding_matches_independent_rational_oracle() {
    let source = DecimalConversionUnit::Fahrenheit;
    let target = DecimalConversionUnit::Base;
    let source_definition = source
        .definition()
        .expect("source definition should be valid");
    let target_definition = target
        .definition()
        .expect("target definition should be valid");

    for value in [dec!(-459.67), dec!(-40), dec!(0), dec!(32), dec!(212)] {
        let exact =
            expected_conversion(value, source_definition, target_definition);
        for scale in [0, 2, 6] {
            for strategy in ROUNDING_STRATEGIES {
                let options = ConversionOptions::fixed_scale(scale, strategy)
                    .expect("scale should be valid");
                let actual = convert(value, source, target, options)
                    .expect("selected affine conversion should fit Decimal");
                let expected = round_rational(&exact, scale, strategy);

                assert_eq!(
                    actual, expected,
                    "value={value}, scale={scale}, strategy={strategy:?}",
                );
                assert_eq!(actual.scale(), scale);
            }
        }
    }
}

#[test]
fn test_conversion_factor_rejects_non_positive_terms() {
    for (numerator, denominator) in [
        (Decimal::ZERO, Decimal::ONE),
        (dec!(-1), Decimal::ONE),
        (Decimal::ONE, Decimal::ZERO),
        (Decimal::ONE, dec!(-1)),
    ] {
        assert!(matches!(
            ConversionFactor::new(numerator, denominator),
            Err(MeasurementError::InvalidUnitDefinition { .. }),
        ));
    }
}

#[test]
fn test_conversion_factor_from_decimal_uses_identity_denominator() {
    let factor = ConversionFactor::from_decimal(dec!(2.5))
        .expect("positive finite decimal factor should be valid");

    assert_eq!(factor.numerator(), dec!(2.5));
    assert_eq!(factor.denominator(), Decimal::ONE);
    assert!(matches!(
        ConversionFactor::from_decimal(Decimal::ZERO),
        Err(MeasurementError::InvalidUnitDefinition { .. }),
    ));
}

#[test]
fn test_identical_definition_preserves_or_applies_scale() {
    let value = dec!(12.3400);
    let maximum = ConversionOptions::maximum_precision();
    let fixed = ConversionOptions::fixed_scale(
        2,
        RoundingStrategy::MidpointNearestEven,
    )
    .expect("scale should be valid");

    let preserved = convert(
        value,
        DecimalConversionUnit::Base,
        DecimalConversionUnit::Base,
        maximum,
    )
    .expect("identical conversion should succeed");
    let rounded = convert(
        dec!(12.345),
        DecimalConversionUnit::Base,
        DecimalConversionUnit::Base,
        fixed,
    )
    .expect("identical conversion should apply scale");

    assert_eq!(preserved, value);
    assert_eq!(preserved.scale(), 4);
    assert_eq!(rounded, dec!(12.34));
    assert_eq!(rounded.scale(), 2);
}

#[test]
fn test_mathematically_equivalent_definitions_preserve_scale() {
    let value = dec!(12.3400);
    let options = ConversionOptions::maximum_precision();

    let converted = convert(
        value,
        DecimalConversionUnit::DivideByPointOne,
        DecimalConversionUnit::Ten,
        options,
    )
    .expect("equivalent definitions should convert");

    assert_eq!(converted, value);
    assert_eq!(converted.scale(), value.scale());
}

#[test]
fn test_identical_tiny_definition_applies_scale_without_ratio_underflow() {
    let options = ConversionOptions::fixed_scale(
        2,
        RoundingStrategy::MidpointNearestEven,
    )
    .expect("scale should be valid");

    let converted = convert(
        dec!(12.345),
        DecimalConversionUnit::TinyHalf,
        DecimalConversionUnit::TinyHalf,
        options,
    )
    .expect("identical tiny definition should only apply output scale");

    assert_eq!(converted, dec!(12.34));
    assert_eq!(converted.scale(), 2);
}

#[test]
fn test_equivalent_tiny_definitions_avoid_combined_ratio_underflow() {
    assert_eq!(
        convert(
            dec!(12.345),
            DecimalConversionUnit::TinyHalf,
            DecimalConversionUnit::EquivalentTinyHalf,
            ConversionOptions::default(),
        ),
        Ok(dec!(12.345)),
    );
}

#[test]
fn test_decimal_conversion_falls_back_when_combined_numerator_underflows() {
    let tiny = Decimal::new(1, 28);
    let expected = Decimal::MAX
        .checked_mul(tiny)
        .and_then(|value| value.checked_mul(tiny))
        .expect("sequential products should be representable");
    assert_ne!(expected, Decimal::ZERO);

    assert_eq!(
        convert(
            Decimal::MAX,
            DecimalConversionUnit::Tiny,
            DecimalConversionUnit::InverseTiny,
            ConversionOptions::default(),
        ),
        Ok(expected),
    );
}

#[test]
fn test_decimal_conversion_falls_back_when_combined_denominator_underflows() {
    let tiny = Decimal::new(1, 28);
    let expected = Decimal::ONE
        .checked_div(tiny)
        .expect("inverse tiny factor should be representable");

    assert_eq!(
        convert(
            tiny,
            DecimalConversionUnit::InverseTiny,
            DecimalConversionUnit::Tiny,
            ConversionOptions::default(),
        ),
        Ok(expected),
    );
}

#[test]
fn test_decimal_conversion_falls_back_when_combined_factor_would_round() {
    let factor = dec!(0.000000000000012);
    let expected = Decimal::MAX
        .checked_mul(factor)
        .and_then(|value| value.checked_mul(factor))
        .expect("sequential products should be representable");

    assert_eq!(
        convert(
            Decimal::MAX,
            DecimalConversionUnit::TwelveFemto,
            DecimalConversionUnit::InverseTwelveFemto,
            ConversionOptions::default(),
        ),
        Ok(expected),
    );
}

#[test]
fn test_decimal_conversion_reports_unrepresentable_requested_scale() {
    let options = ConversionOptions::fixed_scale(
        1,
        RoundingStrategy::MidpointNearestEven,
    )
    .expect("scale should be valid");

    assert_eq!(
        convert(
            Decimal::MAX,
            DecimalConversionUnit::Base,
            DecimalConversionUnit::Base,
            options,
        ),
        Err(MeasurementError::OutputScaleUnrepresentable { scale: 1 }),
    );
}

#[test]
fn test_decimal_conversion_reports_offset_overflow() {
    let options = ConversionOptions::default();

    assert_eq!(
        convert(
            Decimal::MAX,
            DecimalConversionUnit::OffsetOne,
            DecimalConversionUnit::Base,
            options,
        ),
        Err(MeasurementError::ValueOutOfRange),
    );
    assert_eq!(
        convert(
            Decimal::MIN,
            DecimalConversionUnit::Base,
            DecimalConversionUnit::OffsetOne,
            options,
        ),
        Err(MeasurementError::ValueOutOfRange),
    );
}

#[test]
fn test_decimal_conversion_rejects_exact_boundary_overflow_before_rounding() {
    let options = ConversionOptions::maximum_precision();

    assert_eq!(
        convert(
            Decimal::MAX,
            DecimalConversionUnit::OffsetPointFour,
            DecimalConversionUnit::Base,
            options,
        ),
        Err(MeasurementError::ValueOutOfRange),
    );
    assert_eq!(
        convert(
            Decimal::MIN,
            DecimalConversionUnit::Base,
            DecimalConversionUnit::OffsetPointFour,
            options,
        ),
        Err(MeasurementError::ValueOutOfRange),
    );
}

#[test]
fn test_decimal_conversion_falls_back_when_combined_factor_overflows() {
    assert_eq!(
        convert(
            Decimal::ZERO,
            DecimalConversionUnit::Maximum,
            DecimalConversionUnit::InverseMaximum,
            ConversionOptions::default(),
        ),
        Ok(Decimal::ZERO),
    );
}

#[test]
fn test_decimal_conversion_falls_back_when_combined_mantissa_exceeds_decimal() {
    assert_eq!(
        convert(
            dec!(0.01),
            DecimalConversionUnit::Large,
            DecimalConversionUnit::InverseLarge,
            ConversionOptions::default(),
        ),
        Ok(dec!(10000000000000000000000000000)),
    );
}

#[test]
fn test_decimal_conversion_divides_first_after_multiplication_overflow() {
    assert_eq!(
        convert(
            Decimal::MAX,
            DecimalConversionUnit::TwoOverTwo,
            DecimalConversionUnit::Base,
            ConversionOptions::default(),
        ),
        Ok(Decimal::MAX),
    );
}

#[test]
fn test_decimal_conversion_reports_value_out_of_range() {
    assert_eq!(
        convert(
            Decimal::MAX,
            DecimalConversionUnit::DivideByPointOne,
            DecimalConversionUnit::Base,
            ConversionOptions::default(),
        ),
        Err(MeasurementError::ValueOutOfRange),
    );

    assert_eq!(
        convert(
            Decimal::MAX,
            DecimalConversionUnit::Ten,
            DecimalConversionUnit::Base,
            ConversionOptions::default(),
        ),
        Err(MeasurementError::ValueOutOfRange),
    );
}

/// Verifies that fixed-scale output preserves value-range classification.
#[test]
fn test_fixed_scale_conversion_preserves_value_range_error() {
    let options = ConversionOptions::fixed_scale(
        0,
        RoundingStrategy::MidpointNearestEven,
    )
    .expect("scale should be valid");

    assert_eq!(
        convert(
            Decimal::MAX,
            DecimalConversionUnit::Ten,
            DecimalConversionUnit::Base,
            options,
        ),
        Err(MeasurementError::ValueOutOfRange),
    );
}

#[test]
fn test_decimal_conversion_reduces_two_over_two_before_max_arithmetic() {
    assert_eq!(
        convert(
            Decimal::MAX,
            DecimalConversionUnit::TwoOverTwo,
            DecimalConversionUnit::Base,
            ConversionOptions::default(),
        ),
        Ok(Decimal::MAX),
    );
}

#[test]
fn test_decimal_conversion_cross_cancels_equal_large_factors() {
    let expected = Decimal::MAX
        .checked_sub(Decimal::ONE)
        .expect("MAX minus one should be representable");

    assert_eq!(
        convert(
            Decimal::MAX,
            DecimalConversionUnit::MaximumOverTwo,
            DecimalConversionUnit::MaximumOverTwoOffsetOne,
            ConversionOptions::default(),
        ),
        Ok(expected),
    );
}

proptest! {
    /// Checks cross-cancelled values near Decimal's positive boundary.
    #[test]
    fn prop_boundary_ratio_conversion_matches_independent_rational_oracle(
        multiplier in 1_u32..=1_000,
    ) {
        let value = Decimal::try_from_i128_with_scale(
            Decimal::MAX.mantissa() - 3 * i128::from(multiplier),
            1,
        )
        .expect("generated boundary value should fit Decimal");
        let source = DecimalConversionUnit::TwoThirds;
        let target = DecimalConversionUnit::Base;
        let actual = convert(
            value,
            source,
            target,
            ConversionOptions::maximum_precision(),
        )
        .expect("cross-cancelled boundary result should fit Decimal");
        let expected = expected_conversion(
            value,
            source.definition().expect("source definition should be valid"),
            target.definition().expect("target definition should be valid"),
        );

        prop_assert_eq!(decimal_as_rational(actual), expected);
    }

    #[test]
    fn prop_public_conversion_matches_independent_rational_oracle(
        multiplier in -1_000_000_i64..=1_000_000_i64,
    ) {
        let value = Decimal::from(multiplier) * dec!(381);
        let source = DecimalConversionUnit::Base;
        let target = DecimalConversionUnit::Foot;
        let actual = convert(
            value,
            source,
            target,
            ConversionOptions::maximum_precision(),
        )
        .expect("selected rational cases should fit Decimal exactly");
        let expected = expected_conversion(
            value,
            source.definition().expect("source definition should be valid"),
            target.definition().expect("target definition should be valid"),
        );

        prop_assert_eq!(decimal_as_rational(actual), expected);
    }

    #[test]
    fn prop_fixed_scale_conversion_matches_independent_rational_oracle(
        value in -1_000_000_i64..=1_000_000_i64,
        scale in 0_u32..=6,
        strategy_index in 0_usize..ROUNDING_STRATEGIES.len(),
    ) {
        let value = Decimal::from(value);
        let source = DecimalConversionUnit::Base;
        let target = DecimalConversionUnit::Foot;
        let strategy = ROUNDING_STRATEGIES[strategy_index];
        let actual = convert(
            value,
            source,
            target,
            ConversionOptions::fixed_scale(scale, strategy)
                .expect("generated scale should be valid"),
        )
        .expect("generated conversion should fit Decimal");
        let exact = expected_conversion(
            value,
            source.definition().expect("source definition should be valid"),
            target.definition().expect("target definition should be valid"),
        );
        let expected = round_rational(&exact, scale, strategy);

        prop_assert_eq!(actual, expected);
        prop_assert_eq!(actual.scale(), scale);
    }
}
