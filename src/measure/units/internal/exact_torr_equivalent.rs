// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact Torr-equivalent pressure marker for the approximate `uom` bridge.

unit! {
    system: uom::si;
    quantity: uom::si::pressure;

    /// Pressure coefficient equal to 101325/760 pascals.
    @exact_torr_equivalent: 101_325.0 / 760.0;
        "exact Torr equivalent", "exact Torr equivalent", "exact Torr equivalents";
}
