/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Units for persisted volume measurements.

use super::define_measurement_unit;
use uom::si::f64::Volume as UomVolume;
use uom::si::volume::{
    cubic_centimeter,
    cubic_foot,
    cubic_inch,
    cubic_meter,
    cubic_millimeter,
    cubic_yard,
    cup,
    fluid_ounce,
    gallon,
    liter,
    microliter,
    milliliter,
    pint_liquid,
    quart_liquid,
};

define_measurement_unit! {
    /// Units for persisted `uom` volume quantities.
    pub enum Volume for UomVolume, "volume" {
        /// Cubic millimeter (`mm³`).
        CubicMillimeter => "mm³", cubic_millimeter;
        /// Cubic centimeter (`cm³`).
        CubicCentimeter => "cm³", cubic_centimeter;
        /// Cubic meter (`m³`).
        CubicMeter => "m³", cubic_meter;
        /// Microliter (`µL`).
        Microliter => "µL" | "uL" | "μL", microliter;
        /// Milliliter (`mL`).
        Milliliter => "mL", milliliter;
        /// Liter (`L`).
        Liter => "L", liter;
        /// Cubic inch (`in³`).
        CubicInch => "in³", cubic_inch;
        /// Cubic foot (`ft³`).
        CubicFoot => "ft³", cubic_foot;
        /// Cubic yard (`yd³`).
        CubicYard => "yd³", cubic_yard;
        /// US fluid ounce (`fl oz`).
        FluidOunce => "fl oz", fluid_ounce;
        /// US cup (`cup`).
        Cup => "cup", cup;
        /// US liquid pint (`liq pt`).
        PintLiquid => "liq pt", pint_liquid;
        /// US liquid quart (`liq qt`).
        QuartLiquid => "liq qt", quart_liquid;
        /// Gallon (`gal`).
        Gallon => "gal", gallon;
    }
}
