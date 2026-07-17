// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_measure::{
    Decimal,
    Measurement,
};
use qubit_measure_downstream_uom_fixture::{
    DownstreamLength,
    assert_uom_bridge_is_generated,
};
use uom::si::length::meter;

#[test]
fn test_downstream_uom_bridge_runs_in_dependency_owned_feature_build() {
    assert_uom_bridge_is_generated();
    let measurement =
        Measurement::new(Decimal::new(25, 1), DownstreamLength::Meter);

    let quantity = measurement.to_uom_approx();
    assert_eq!(quantity.get::<meter>(), 2.5);
    assert_eq!(
        Measurement::<DownstreamLength>::from_uom_approx(
            quantity,
            DownstreamLength::Meter,
        )
        .expect("downstream uom quantity should convert back"),
        measurement,
    );
}
