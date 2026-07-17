// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! External unit family exposing Decimal conversion boundary cases.

use qubit_measure::define_unit_family;

define_unit_family! {
    /// Synthetic units used to exercise public typed Decimal conversion.
    pub enum DecimalConversionUnit for "decimal_conversion" {
        /// Identity definition.
        Base => { symbol: "base"; coefficient: 1; }
        /// Fahrenheit-like affine definition.
        Fahrenheit => { symbol: "fahrenheit"; coefficient: 5 / 9; offset: 459.67; }
        /// Foot-like rational definition.
        Foot => { symbol: "foot"; coefficient: 381 / 1250; }
        /// Tiny one-half ratio.
        TinyHalf => { symbol: "tiny-half"; coefficient: 0.000000000000001 / 0.000000000000002; }
        /// Equivalent tiny one-half ratio.
        EquivalentTinyHalf => { symbol: "equivalent-tiny-half"; coefficient: 0.000000000000002 / 0.000000000000004; }
        /// Smallest positive scale-28 factor.
        Tiny => { symbol: "tiny"; coefficient: 0.0000000000000000000000000001; }
        /// Reciprocal representation of the smallest scale-28 factor.
        InverseTiny => { symbol: "inverse-tiny"; coefficient: 1 / 0.0000000000000000000000000001; }
        /// Scale-15 factor used to exercise rounded combined products.
        TwelveFemto => { symbol: "twelve-femto"; coefficient: 0.000000000000012; }
        /// Reciprocal scale-15 factor.
        InverseTwelveFemto => { symbol: "inverse-twelve-femto"; coefficient: 1 / 0.000000000000012; }
        /// Identity factor with a positive offset.
        OffsetOne => { symbol: "offset-one"; coefficient: 1; offset: 1; }
        /// Maximum Decimal factor.
        Maximum => { symbol: "maximum"; coefficient: 79228162514264337593543950335; }
        /// Reciprocal maximum Decimal factor.
        InverseMaximum => { symbol: "inverse-maximum"; coefficient: 1 / 79228162514264337593543950335; }
        /// Large integral factor.
        Large => { symbol: "large"; coefficient: 1000000000000000; }
        /// Reciprocal large integral factor.
        InverseLarge => { symbol: "inverse-large"; coefficient: 1 / 1000000000000000; }
        /// Reducible identity ratio.
        TwoOverTwo => { symbol: "two-over-two"; coefficient: 2 / 2; }
        /// Factor ten expressed through division.
        DivideByPointOne => { symbol: "divide-by-point-one"; coefficient: 1 / 0.1; }
        /// Integral factor ten.
        Ten => { symbol: "ten"; coefficient: 10; }
        /// Large equal factor with zero offset.
        MaximumOverTwo => { symbol: "maximum-over-two"; coefficient: 79228162514264337593543950335 / 2; }
        /// Large equal factor with a target offset.
        MaximumOverTwoOffsetOne => { symbol: "maximum-over-two-offset-one"; coefficient: 79228162514264337593543950335 / 2; offset: 1; }
    }
}
