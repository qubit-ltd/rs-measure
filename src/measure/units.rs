/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Unit families backed by `uom` quantities.

mod area;
mod energy;
mod frequency;
mod length;
mod mass;
mod mass_density;
mod power;
mod pressure;
mod temperature;
mod temperature_interval;
mod time;
mod velocity;
mod volume;

pub use area::Area;
pub use energy::Energy;
pub use frequency::Frequency;
pub use length::Length;
pub use mass::Mass;
pub use mass_density::MassDensity;
pub use power::Power;
pub use pressure::Pressure;
pub use temperature::Temperature;
pub use temperature_interval::TemperatureInterval;
pub use time::Time;
pub use velocity::Velocity;
pub use volume::Volume;

use crate::measure::MeasurementError;
use rust_decimal::Decimal;
use rust_decimal::prelude::{
    FromPrimitive,
    ToPrimitive,
};

macro_rules! define_measurement_unit {
    (
        $(#[$enum_attr:meta])*
        pub enum $unit:ident for $quantity_ty:ty, $quantity_name:literal {
            $(
                $(#[$variant_attr:meta])*
                $variant:ident => $symbol:literal $(| $alias:literal)*, $uom_unit:ty;
            )+
        }
    ) => {
        $(#[$enum_attr])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $unit {
            $(
                $(#[$variant_attr])*
                $variant,
            )+
        }

        impl crate::measure::Unit for $unit {
            type Quantity = $quantity_ty;

            const QUANTITY: &'static str = $quantity_name;

            fn all() -> &'static [Self] {
                &[
                    $(Self::$variant,)+
                ]
            }

            fn symbol(self) -> &'static str {
                match self {
                    $(Self::$variant => $symbol,)+
                }
            }

            fn to_uom(self, value: rust_decimal::Decimal) -> Result<Self::Quantity, crate::measure::MeasurementError> {
                let value = super::decimal_to_f64(value)?;
                Ok(match self {
                    $(Self::$variant => <$quantity_ty>::new::<$uom_unit>(value),)+
                })
            }

            fn value_from_uom(self, quantity: Self::Quantity) -> Result<rust_decimal::Decimal, crate::measure::MeasurementError> {
                let value = match self {
                    $(Self::$variant => quantity.get::<$uom_unit>(),)+
                };
                super::decimal_from_f64(value)
            }
        }

        impl std::fmt::Display for $unit {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(crate::measure::Unit::symbol(*self))
            }
        }

        impl std::str::FromStr for $unit {
            type Err = crate::measure::MeasurementError;

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                match input.trim() {
                    $($symbol $(| $alias)* => Ok(Self::$variant),)+
                    unit => Err(crate::measure::MeasurementError::UnknownUnit {
                        quantity: <Self as crate::measure::Unit>::QUANTITY.to_owned(),
                        unit: unit.to_owned(),
                    }),
                }
            }
        }

        impl serde::Serialize for $unit {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(crate::measure::Unit::symbol(*self))
            }
        }

        impl<'de> serde::Deserialize<'de> for $unit {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let symbol = String::deserialize(deserializer)?;
                <Self as std::str::FromStr>::from_str(&symbol).map_err(serde::de::Error::custom)
            }
        }
    };
}

pub(super) use define_measurement_unit;

/// Converts a decimal value into a finite `f64` for `uom`.
fn decimal_to_f64(value: Decimal) -> Result<f64, MeasurementError> {
    match value.to_f64().filter(|value| value.is_finite()) {
        Some(value) => Ok(value),
        None => Err(MeasurementError::DecimalConversion(value.to_string())),
    }
}

/// Converts a finite `f64` value from `uom` into `Decimal`.
fn decimal_from_f64(value: f64) -> Result<Decimal, MeasurementError> {
    match Decimal::from_f64(value) {
        Some(value) => Ok(value),
        None => Err(MeasurementError::DecimalConversion(value.to_string())),
    }
}
