// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Shared assertions for persisted measurement aliases.

macro_rules! assert_measurement_alias {
    ($alias:ident, $unit:ident) => {
        /// Verifies that the measurement alias uses its documented unit family.
        #[test]
        fn test_measurement_alias_uses_expected_unit_family() {
            use qubit_measure::Measurement;
            use qubit_measure::Unit;
            use qubit_measure::measurement;
            use qubit_measure::unit;
            use rust_decimal::Decimal;

            let selected_unit = *unit::$unit::all()
                .first()
                .expect("unit family should not be empty");
            let value: measurement::$alias = Measurement::new(Decimal::ONE, selected_unit);

            assert_eq!(value.quantity_name(), unit::$unit::QUANTITY);
        }
    };
}

pub(crate) use assert_measurement_alias;
