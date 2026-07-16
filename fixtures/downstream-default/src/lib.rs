// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Default-feature downstream macro fixture.

use qubit_measure::define_unit_family;

define_unit_family! {
    /// Unit family whose unresolved uom tokens must be discarded.
    pub enum DefaultFixtureUnit for "default_fixture", uom = missing::Quantity {
        /// Base fixture unit.
        Base => {
            symbol: "dfu";
            coefficient: 1;
            uom: missing::BaseUnit;
        }
        /// Integer coefficient fixture unit.
        Integer => {
            symbol: "integer";
            coefficient: 42;
            uom: missing::IntegerUnit;
        }
        /// Decimal coefficient fixture unit.
        Decimal => {
            symbol: "decimal";
            coefficient: 1.25;
            uom: missing::DecimalUnit;
        }
        /// Negative offset fixture unit.
        NegativeOffset => {
            symbol: "negative-offset";
            coefficient: 2;
            offset: -273.15;
            uom: missing::NegativeOffsetUnit;
        }
        /// Scientific-notation coefficient fixture unit.
        Scientific => {
            symbol: "scientific";
            coefficient: 1.234_5e-6;
            uom: missing::ScientificUnit;
        }
        /// Digit-separated coefficient fixture unit.
        DigitSeparated => {
            symbol: "digit-separated";
            coefficient: 1_234_567;
            uom: missing::DigitSeparatedUnit;
        }
        /// Radix integer coefficient fixture unit.
        RadixInteger => {
            symbol: "radix-integer";
            coefficient: 0x1_FF;
            uom: missing::RadixIntegerUnit;
        }
        /// Radix integer ratio fixture unit.
        RadixRatio => {
            symbol: "radix-ratio";
            coefficient: 0o20 / 0b10;
            uom: missing::RadixRatioUnit;
        }
    }
}
