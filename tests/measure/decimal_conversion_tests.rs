// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_measure::{
    ConversionFactor,
    ConversionOptions,
    Decimal,
    MeasurementError,
    RoundingStrategy,
    UnitDefinition,
};
use rust_decimal::dec;

#[test]
fn test_decimal_conversion_keeps_five_ninths_as_a_ratio() {
    let fahrenheit = UnitDefinition::new(
        ConversionFactor::new(dec!(5), dec!(9))
            .expect("factor should be valid"),
        dec!(459.67),
    );
    let kelvin = UnitDefinition::base();
    let options = ConversionOptions::maximum_precision(
        RoundingStrategy::MidpointNearestEven,
    );

    assert_eq!(
        fahrenheit
            .convert_value_to(dec!(32), kelvin, options)
            .expect("Fahrenheit should convert to kelvin"),
        dec!(273.15),
    );
}

#[test]
fn test_decimal_conversion_applies_requested_scale() {
    let meter = UnitDefinition::base();
    let foot = UnitDefinition::new(
        ConversionFactor::new(dec!(381), dec!(1250))
            .expect("factor should be valid"),
        Decimal::ZERO,
    );
    let options = ConversionOptions::fixed_scale(
        4,
        RoundingStrategy::MidpointNearestEven,
    )
    .expect("scale should be valid");

    let result = meter
        .convert_value_to(dec!(1), foot, options)
        .expect("meter should convert to foot");
    assert_eq!(result, dec!(3.2808));
    assert_eq!(result.scale(), 4);
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
fn test_conversion_factor_from_integer_uses_identity_denominator() {
    let factor = ConversionFactor::from_integer(dec!(2.5))
        .expect("positive finite decimal factor should be valid");

    assert_eq!(factor.numerator(), dec!(2.5));
    assert_eq!(factor.denominator(), Decimal::ONE);
    assert!(matches!(
        ConversionFactor::from_integer(Decimal::ZERO),
        Err(MeasurementError::InvalidUnitDefinition { .. }),
    ));
}

#[test]
fn test_identical_definition_preserves_or_applies_scale() {
    let definition = UnitDefinition::base();
    let value = dec!(12.3400);
    let maximum = ConversionOptions::maximum_precision(
        RoundingStrategy::MidpointNearestEven,
    );
    let fixed = ConversionOptions::fixed_scale(
        2,
        RoundingStrategy::MidpointNearestEven,
    )
    .expect("scale should be valid");

    let preserved = definition
        .convert_value_to(value, definition, maximum)
        .expect("identical conversion should succeed");
    let rounded = definition
        .convert_value_to(dec!(12.345), definition, fixed)
        .expect("identical conversion should apply scale");

    assert_eq!(preserved, value);
    assert_eq!(preserved.scale(), 4);
    assert_eq!(rounded, dec!(12.34));
    assert_eq!(rounded.scale(), 2);
}

#[test]
fn test_identical_tiny_definition_applies_scale_without_ratio_underflow() {
    let tiny_factor =
        ConversionFactor::new(dec!(0.000000000000001), dec!(0.000000000000002))
            .expect("tiny factor should be valid");
    let definition = UnitDefinition::new(tiny_factor, Decimal::ZERO);
    let options = ConversionOptions::fixed_scale(
        2,
        RoundingStrategy::MidpointNearestEven,
    )
    .expect("scale should be valid");

    let converted = definition
        .convert_value_to(dec!(12.345), definition, options)
        .expect("identical tiny definition should only apply output scale");

    assert_eq!(converted, dec!(12.34));
    assert_eq!(converted.scale(), 2);
}

#[test]
fn test_equivalent_tiny_definitions_avoid_combined_ratio_underflow() {
    let source = UnitDefinition::new(
        ConversionFactor::new(dec!(0.000000000000001), dec!(0.000000000000002))
            .expect("source factor should be valid"),
        Decimal::ZERO,
    );
    let target = UnitDefinition::new(
        ConversionFactor::new(dec!(0.000000000000002), dec!(0.000000000000004))
            .expect("target factor should be valid"),
        Decimal::ZERO,
    );

    assert_eq!(
        source.convert_value_to(
            dec!(12.345),
            target,
            ConversionOptions::default(),
        ),
        Ok(dec!(12.345)),
    );
}

#[test]
fn test_decimal_conversion_falls_back_when_combined_numerator_underflows() {
    let tiny = Decimal::new(1, 28);
    let source = UnitDefinition::new(
        ConversionFactor::new(tiny, Decimal::ONE)
            .expect("source factor should be valid"),
        Decimal::ZERO,
    );
    let target = UnitDefinition::new(
        ConversionFactor::new(Decimal::ONE, tiny)
            .expect("target factor should be valid"),
        Decimal::ZERO,
    );
    let expected = Decimal::MAX
        .checked_mul(tiny)
        .and_then(|value| value.checked_mul(tiny))
        .expect("sequential products should be representable");
    assert_ne!(expected, Decimal::ZERO);

    assert_eq!(
        source.convert_value_to(
            Decimal::MAX,
            target,
            ConversionOptions::default(),
        ),
        Ok(expected),
    );
}

#[test]
fn test_decimal_conversion_falls_back_when_combined_denominator_underflows() {
    let tiny = Decimal::new(1, 28);
    let source = UnitDefinition::new(
        ConversionFactor::new(Decimal::ONE, tiny)
            .expect("source factor should be valid"),
        Decimal::ZERO,
    );
    let target = UnitDefinition::new(
        ConversionFactor::new(tiny, Decimal::ONE)
            .expect("target factor should be valid"),
        Decimal::ZERO,
    );
    let expected = Decimal::ONE
        .checked_div(tiny)
        .expect("inverse tiny factor should be representable");

    assert_eq!(
        source.convert_value_to(tiny, target, ConversionOptions::default()),
        Ok(expected),
    );
}

#[test]
fn test_decimal_conversion_falls_back_when_combined_factor_would_round() {
    let factor = dec!(0.000000000000012);
    let source = UnitDefinition::new(
        ConversionFactor::new(factor, Decimal::ONE)
            .expect("source factor should be valid"),
        Decimal::ZERO,
    );
    let target = UnitDefinition::new(
        ConversionFactor::new(Decimal::ONE, factor)
            .expect("target factor should be valid"),
        Decimal::ZERO,
    );
    let expected = Decimal::MAX
        .checked_mul(factor)
        .and_then(|value| value.checked_mul(factor))
        .expect("sequential products should be representable");

    assert_eq!(
        source.convert_value_to(
            Decimal::MAX,
            target,
            ConversionOptions::default(),
        ),
        Ok(expected),
    );
}

#[test]
fn test_decimal_conversion_reports_unrepresentable_requested_scale() {
    let definition = UnitDefinition::base();
    let options = ConversionOptions::fixed_scale(
        1,
        RoundingStrategy::MidpointNearestEven,
    )
    .expect("scale should be valid");

    assert_eq!(
        definition.convert_value_to(Decimal::MAX, definition, options),
        Err(MeasurementError::ArithmeticOverflow {
            operation: "set output scale",
        }),
    );
}

#[test]
fn test_decimal_conversion_reports_offset_overflow() {
    let identity = ConversionFactor::new(Decimal::ONE, Decimal::ONE)
        .expect("identity factor should be valid");
    let offset_unit = UnitDefinition::new(identity, Decimal::ONE);
    let options = ConversionOptions::default();

    assert_eq!(
        offset_unit.convert_value_to(
            Decimal::MAX,
            UnitDefinition::base(),
            options,
        ),
        Err(MeasurementError::ArithmeticOverflow {
            operation: "add source offset",
        }),
    );
    assert_eq!(
        UnitDefinition::base().convert_value_to(
            Decimal::MIN,
            offset_unit,
            options,
        ),
        Err(MeasurementError::ArithmeticOverflow {
            operation: "subtract target offset",
        }),
    );
}

#[test]
fn test_decimal_conversion_falls_back_when_combined_factor_overflows() {
    let source = UnitDefinition::new(
        ConversionFactor::new(Decimal::MAX, Decimal::ONE)
            .expect("source factor should be valid"),
        Decimal::ZERO,
    );
    let target = UnitDefinition::new(
        ConversionFactor::new(Decimal::ONE, Decimal::MAX)
            .expect("target factor should be valid"),
        Decimal::ZERO,
    );

    assert_eq!(
        source.convert_value_to(
            Decimal::ZERO,
            target,
            ConversionOptions::default(),
        ),
        Ok(Decimal::ZERO),
    );
}

#[test]
fn test_decimal_conversion_falls_back_when_combined_mantissa_exceeds_decimal() {
    let large = dec!(1000000000000000);
    let source = UnitDefinition::new(
        ConversionFactor::new(large, Decimal::ONE)
            .expect("source factor should be valid"),
        Decimal::ZERO,
    );
    let target = UnitDefinition::new(
        ConversionFactor::new(Decimal::ONE, large)
            .expect("target factor should be valid"),
        Decimal::ZERO,
    );

    assert_eq!(
        source.convert_value_to(
            dec!(0.01),
            target,
            ConversionOptions::default(),
        ),
        Ok(dec!(10000000000000000000000000000)),
    );
}

#[test]
fn test_decimal_conversion_divides_first_after_multiplication_overflow() {
    let reducible = UnitDefinition::new(
        ConversionFactor::new(dec!(10), dec!(10))
            .expect("reducible factor should be valid"),
        Decimal::ZERO,
    );

    assert_eq!(
        reducible.convert_value_to(
            Decimal::MAX,
            UnitDefinition::base(),
            ConversionOptions::default(),
        ),
        Ok(Decimal::MAX),
    );
}

#[test]
fn test_decimal_conversion_reports_ratio_overflow() {
    let division_overflow = UnitDefinition::new(
        ConversionFactor::new(Decimal::ONE, dec!(0.1))
            .expect("division factor should be valid"),
        Decimal::ZERO,
    );
    assert_eq!(
        division_overflow.convert_value_to(
            Decimal::MAX,
            UnitDefinition::base(),
            ConversionOptions::default(),
        ),
        Err(MeasurementError::ArithmeticOverflow {
            operation: "divide conversion ratio",
        }),
    );

    let multiplication_overflow = UnitDefinition::new(
        ConversionFactor::new(dec!(10), Decimal::ONE)
            .expect("multiplication factor should be valid"),
        Decimal::ZERO,
    );
    assert_eq!(
        multiplication_overflow.convert_value_to(
            Decimal::MAX,
            UnitDefinition::base(),
            ConversionOptions::default(),
        ),
        Err(MeasurementError::ArithmeticOverflow {
            operation: "multiply conversion ratio",
        }),
    );
}

#[test]
fn test_decimal_conversion_reduces_two_over_two_before_max_arithmetic() {
    let identity = UnitDefinition::new(
        ConversionFactor::new(dec!(2), dec!(2))
            .expect("factor should be valid"),
        Decimal::ZERO,
    );

    assert_eq!(
        identity.convert_value_to(
            Decimal::MAX,
            UnitDefinition::base(),
            ConversionOptions::default(),
        ),
        Ok(Decimal::MAX),
    );
}

#[test]
fn test_decimal_conversion_cross_cancels_equal_large_factors() {
    let factor = ConversionFactor::new(Decimal::MAX, dec!(2))
        .expect("factor should be valid");
    let source = UnitDefinition::new(factor, Decimal::ZERO);
    let target = UnitDefinition::new(factor, Decimal::ONE);
    let expected = Decimal::MAX
        .checked_sub(Decimal::ONE)
        .expect("MAX minus one should be representable");

    assert_eq!(
        source.convert_value_to(
            Decimal::MAX,
            target,
            ConversionOptions::default(),
        ),
        Ok(expected),
    );
}
