// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::fmt;
use std::str::FromStr;

use qubit_measure::{
    MeasurementError,
    Unit,
    UnitDefinition,
    define_unit_family,
};

define_unit_family! {
    pub enum CustomLength for "custom_length" {
        Base => {
            symbol: "cu";
            coefficient: 1;
        }
        Half => {
            symbol: "hcu";
            coefficient: 1 / 2;
            aliases: ["half-cu"];
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ManualUnit {
    Base,
}

impl Unit for ManualUnit {
    const QUANTITY: &'static str = "manual";

    fn all() -> &'static [Self] {
        &[Self::Base]
    }

    fn symbol(self) -> &'static str {
        "manual"
    }

    fn aliases(self) -> &'static [&'static str] {
        &["mnl"]
    }

    fn definition(self) -> Result<UnitDefinition, MeasurementError> {
        Ok(UnitDefinition::base())
    }
}

impl fmt::Display for ManualUnit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.symbol())
    }
}

impl FromStr for ManualUnit {
    type Err = MeasurementError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::parse_lenient(input)
    }
}

#[test]
fn test_external_family_supports_strict_and_lenient_parsing() {
    assert_eq!(
        CustomLength::parse_strict("hcu").expect("canonical unit should parse"),
        CustomLength::Half,
    );
    assert!(matches!(
        CustomLength::parse_strict("half-cu"),
        Err(MeasurementError::NonCanonicalUnit { .. }),
    ));
    assert_eq!("half-cu".parse(), Ok(CustomLength::Half));
    assert_eq!(CustomLength::Half.to_string(), "hcu");
}

#[test]
fn test_unit_trait_supports_manual_external_implementations() {
    assert_eq!(ManualUnit::parse_lenient("mnl"), Ok(ManualUnit::Base));
    assert_eq!(
        ManualUnit::Base
            .definition()
            .expect("manual definition should be valid"),
        UnitDefinition::base(),
    );
}
