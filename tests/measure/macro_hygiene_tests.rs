// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

#[cfg(feature = "uom")]
use qubit_measure::UomUnit;
#[cfg(feature = "uom")]
use rust_decimal::Decimal;

mod conflicting_names {
    use qubit_measure::Unit;

    #[cfg(feature = "uom")]
    use super::Decimal;
    #[cfg(feature = "uom")]
    use super::UomUnit;

    #[allow(dead_code)]
    type Result<T> = ::core::result::Result<T, ()>;

    #[allow(dead_code, non_snake_case)]
    fn Ok(_: ()) {}

    const DEFINITION: qubit_measure::UnitDefinition = qubit_measure::UnitDefinition::base();

    qubit_measure::define_unit_family! {
        /// Coefficient declaration under conflicting caller names.
        pub enum CoefficientUnit for "hygiene_coefficient" {
            /// Exact base unit.
            Base => { symbol: "hc"; coefficient: 1; }
        }
    }

    qubit_measure::define_unit_family! {
        /// Definition-path declaration under conflicting caller names.
        pub enum DefinitionUnit for "hygiene_definition" {
            /// Exact base unit.
            Base => { symbol: "hd"; definition: DEFINITION; }
        }
    }

    #[test]
    fn test_macro_hygiene_coefficient() {
        let unit: CoefficientUnit = "hc".parse().expect("canonical symbol should parse");
        assert_eq!(unit, CoefficientUnit::Base);
        assert_eq!(unit.definition().expect("definition should be valid"), DEFINITION);
        let json = serde_json::to_string(&unit).expect("unit should serialize");
        assert_eq!(json, "\"hc\"");
        assert_eq!(
            serde_json::from_str::<CoefficientUnit>(&json).expect("unit should deserialize"),
            unit,
        );
    }

    #[test]
    fn test_macro_hygiene_definition() {
        let unit: DefinitionUnit = "hd".parse().expect("canonical symbol should parse");
        assert_eq!(unit, DefinitionUnit::Base);
        assert_eq!(unit.definition().expect("definition should be valid"), DEFINITION);
        let json = serde_json::to_string(&unit).expect("unit should serialize");
        assert_eq!(json, "\"hd\"");
        assert_eq!(
            serde_json::from_str::<DefinitionUnit>(&json).expect("unit should deserialize"),
            unit,
        );
    }

    #[cfg(feature = "uom")]
    qubit_measure::impl_uom_unit! {
        CoefficientUnit, uom::si::f64::Length {
            base: uom::si::length::meter;
        }
    }

    #[cfg(feature = "uom")]
    #[test]
    fn test_macro_hygiene_uom() {
        let quantity = CoefficientUnit::Base
            .try_to_uom_approx(Decimal::ONE)
            .expect("base unit should convert to uom");
        assert_eq!(quantity.get::<uom::si::length::meter>(), 1.0);
        let value = CoefficientUnit::Base
            .value_from_uom_approx(quantity)
            .expect("uom quantity should convert to base value");
        assert_eq!(value, Decimal::ONE);
    }
}
