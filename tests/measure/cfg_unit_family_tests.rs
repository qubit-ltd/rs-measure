// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_measure::Unit;
use qubit_measure::UnitDefinition;
use qubit_measure::define_unit_family;

define_unit_family! {
    #[cfg(any())]
    enum DisabledFamily for "disabled_family" {
        One => { symbol: "one"; coefficient: 1; }
    }
}

define_unit_family! {
    enum ConditionalFamily for "conditional_family" {
        #[cfg(all(any(), not(unix)))]
        InactiveCombined => { symbol: "unavailable"; coefficient: 3; }
        #[cfg(all())]
        /// The enabled member retains its documentation and lint attributes.
        #[allow(dead_code)]
        Active => { symbol: "active"; coefficient: 1; aliases: ["enabled", "alternate"]; }
        /// This disabled member duplicates active metadata without contributing it.
        #[cfg(any())]
        #[cfg(all())]
        Inactive => { symbol: "active"; coefficient: 2; aliases: ["enabled", "alternate"]; }
    }
}

const BASE_DEFINITION: UnitDefinition = UnitDefinition::base();
define_unit_family! {
    enum ConditionalDefinition for "conditional_definition" {
        Active => { symbol: "active"; definition: BASE_DEFINITION; }
        #[cfg(any())]
        Inactive => { symbol: "inactive"; definition: path_that_does_not_exist::VALUE; }
    }
}

#[test]
fn test_cfg_unit_family_only_exposes_enabled_members() {
    assert_eq!(ConditionalFamily::all(), &[ConditionalFamily::Active]);
    assert_eq!(ConditionalFamily::parse_strict("active"), Ok(ConditionalFamily::Active));
    assert_eq!(
        ConditionalFamily::parse_lenient("enabled"),
        Ok(ConditionalFamily::Active)
    );
    assert_eq!(ConditionalFamily::Active.to_string(), "active");
    assert_eq!(ConditionalFamily::Active.definition(), Ok(UnitDefinition::base()));
    assert_eq!(ConditionalDefinition::all(), &[ConditionalDefinition::Active]);
}
