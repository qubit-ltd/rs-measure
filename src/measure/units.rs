// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Unit families backed by `uom` quantities.

mod acceleration;
mod amount_of_substance;
mod angle;
mod angular_velocity;
mod area;
mod capacitance;
mod catalytic_activity;
mod catalytic_activity_concentration;
mod dynamic_viscosity;
mod electric_charge;
mod electric_current;
mod electric_current_density;
mod electric_field;
mod electric_potential;
mod electrical_conductance;
mod electrical_conductivity;
mod electrical_resistance;
mod electrical_resistivity;
mod energy;
mod force;
mod frequency;
mod heat_capacity;
mod heat_flux_density;
mod illuminance;
mod inductance;
mod kinematic_viscosity;
mod length;
mod luminance;
mod luminous_intensity;
mod magnetic_field_strength;
mod magnetic_flux;
mod magnetic_flux_density;
mod mass;
mod mass_concentration;
mod mass_density;
mod mass_rate;
mod molality;
mod molar_concentration;
mod molar_mass;
mod molar_volume;
mod power;
mod pressure;
mod radioactivity;
mod solid_angle;
mod specific_heat_capacity;
mod specific_radioactivity;
mod surface_tension;
mod temperature;
mod temperature_interval;
mod thermal_conductivity;
mod thermal_resistance;
mod time;
mod torque;
mod velocity;
mod volume;
mod volume_rate;

pub use acceleration::Acceleration;
pub use amount_of_substance::AmountOfSubstance;
pub use angle::Angle;
pub use angular_velocity::AngularVelocity;
pub use area::Area;
pub use capacitance::Capacitance;
pub use catalytic_activity::CatalyticActivity;
pub use catalytic_activity_concentration::CatalyticActivityConcentration;
pub use dynamic_viscosity::DynamicViscosity;
pub use electric_charge::ElectricCharge;
pub use electric_current::ElectricCurrent;
pub use electric_current_density::ElectricCurrentDensity;
pub use electric_field::ElectricField;
pub use electric_potential::ElectricPotential;
pub use electrical_conductance::ElectricalConductance;
pub use electrical_conductivity::ElectricalConductivity;
pub use electrical_resistance::ElectricalResistance;
pub use electrical_resistivity::ElectricalResistivity;
pub use energy::Energy;
pub use force::Force;
pub use frequency::Frequency;
pub use heat_capacity::HeatCapacity;
pub use heat_flux_density::HeatFluxDensity;
pub use illuminance::Illuminance;
pub use inductance::Inductance;
pub use kinematic_viscosity::KinematicViscosity;
pub use length::Length;
pub use luminance::Luminance;
pub use luminous_intensity::LuminousIntensity;
pub use magnetic_field_strength::MagneticFieldStrength;
pub use magnetic_flux::MagneticFlux;
pub use magnetic_flux_density::MagneticFluxDensity;
pub use mass::Mass;
pub use mass_concentration::MassConcentration;
pub use mass_density::MassDensity;
pub use mass_rate::MassRate;
pub use molality::Molality;
pub use molar_concentration::MolarConcentration;
pub use molar_mass::MolarMass;
pub use molar_volume::MolarVolume;
pub use power::Power;
pub use pressure::Pressure;
pub use radioactivity::Radioactivity;
pub use solid_angle::SolidAngle;
pub use specific_heat_capacity::SpecificHeatCapacity;
pub use specific_radioactivity::SpecificRadioactivity;
pub use surface_tension::SurfaceTension;
pub use temperature::Temperature;
pub use temperature_interval::TemperatureInterval;
pub use thermal_conductivity::ThermalConductivity;
pub use thermal_resistance::ThermalResistance;
pub use time::Time;
pub use torque::Torque;
pub use velocity::Velocity;
pub use volume::Volume;
pub use volume_rate::VolumeRate;

/// Builds a validated Decimal conversion factor for an exported unit macro.
#[doc(hidden)]
#[macro_export]
macro_rules! __unit_factor {
    ($numerator:literal) => {
        $crate::ConversionFactor::from_integer(
            $crate::__private::rust_decimal::dec!($numerator),
        )
    };
    ($numerator:literal / $denominator:literal) => {
        $crate::ConversionFactor::new(
            $crate::__private::rust_decimal::dec!($numerator),
            $crate::__private::rust_decimal::dec!($denominator),
        )
    };
}

/// Produces an optional Decimal offset for an exported unit macro.
#[doc(hidden)]
#[macro_export]
macro_rules! __unit_offset {
    () => {
        $crate::Decimal::ZERO
    };
    ($offset:literal) => {
        $crate::__private::rust_decimal::dec!($offset)
    };
}

/// Implements the exact unit metadata shared by public macro variants.
#[doc(hidden)]
#[macro_export]
macro_rules! __define_unit_family_core {
    (
        $(#[$enum_attr:meta])*
        $visibility:vis enum $unit:ident for $quantity_name:literal {
            $(
                $(#[$variant_attr:meta])*
                $variant:ident => {
                    symbol: $symbol:literal;
                    coefficient: $numerator:literal $(/ $denominator:literal)?;
                    $(offset: $offset:literal;)?
                    $(aliases: [$($alias:literal),* $(,)?];)?
                }
            )+
        }
    ) => {
        $(#[$enum_attr])*
        #[non_exhaustive]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        $visibility enum $unit {
            $(
                $(#[$variant_attr])*
                $variant,
            )+
        }

        impl $crate::Unit for $unit {
            const QUANTITY: &'static str = $quantity_name;

            #[inline(always)]
            fn all() -> &'static [Self] {
                &[$(Self::$variant,)+]
            }

            #[inline(always)]
            fn symbol(self) -> &'static str {
                match self {
                    $(Self::$variant => $symbol,)+
                }
            }

            #[inline(always)]
            fn aliases(self) -> &'static [&'static str] {
                match self {
                    $(Self::$variant => &[$($($alias,)*)?],)+
                }
            }

            #[inline]
            fn definition(self) -> Result<$crate::UnitDefinition, $crate::MeasurementError> {
                match self {
                    $(
                        Self::$variant => {
                            let factor = $crate::__unit_factor!(
                                $numerator $(/ $denominator)?
                            )?;
                            Ok($crate::UnitDefinition::new(
                                factor,
                                $crate::__unit_offset!($($offset)?),
                            ))
                        }
                    )+
                }
            }
        }

        impl ::std::fmt::Display for $unit {
            #[inline]
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str(<Self as $crate::Unit>::symbol(*self))
            }
        }

        impl ::std::str::FromStr for $unit {
            type Err = $crate::MeasurementError;

            #[inline]
            fn from_str(input: &str) -> Result<Self, Self::Err> {
                <Self as $crate::Unit>::parse_lenient(input)
            }
        }

        impl $crate::__private::serde::Serialize for $unit {
            #[inline]
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: $crate::__private::serde::Serializer,
            {
                serializer.serialize_str(<Self as $crate::Unit>::symbol(*self))
            }
        }

        impl<'de> $crate::__private::serde::Deserialize<'de> for $unit {
            #[inline]
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: $crate::__private::serde::Deserializer<'de>,
            {
                let symbol = <::std::string::String as
                    $crate::__private::serde::Deserialize>::deserialize(deserializer)?;
                <Self as ::std::str::FromStr>::from_str(&symbol)
                    .map_err($crate::__private::serde::de::Error::custom)
            }
        }
    };
}

/// Defines an externally extensible unit family with exact Decimal factors.
///
/// The generated type implements [`Unit`](crate::Unit), display, lenient
/// `FromStr`, and canonical string Serde. Supplying `uom = Quantity` plus a
/// `uom` type for every variant also implements the optional approximate
/// [`UomUnit`](crate::UomUnit) bridge.
#[macro_export]
macro_rules! define_unit_family {
    (
        $(#[$enum_attr:meta])*
        $visibility:vis enum $unit:ident for $quantity_name:literal, uom = $quantity_ty:ty {
            $(
                $(#[$variant_attr:meta])*
                $variant:ident => {
                    symbol: $symbol:literal;
                    coefficient: $numerator:literal $(/ $denominator:literal)?;
                    $(offset: $offset:literal;)?
                    $(aliases: [$($alias:literal),* $(,)?];)?
                    uom: $uom_unit:ty;
                }
            )+
        }
    ) => {
        $crate::__define_unit_family_core! {
            $(#[$enum_attr])*
            $visibility enum $unit for $quantity_name {
                $(
                    $(#[$variant_attr])*
                    $variant => {
                        symbol: $symbol;
                        coefficient: $numerator $(/ $denominator)?;
                        $(offset: $offset;)?
                        $(aliases: [$($alias),*];)?
                    }
                )+
            }
        }

        impl $crate::UomUnit for $unit {
            type Quantity = $quantity_ty;

            #[inline(always)]
            fn to_uom_approx(self, value: $crate::Decimal) -> Self::Quantity {
                let value = $crate::__private::decimal_to_f64_approx(value);
                match self {
                    $(Self::$variant => <$quantity_ty>::new::<$uom_unit>(value),)+
                }
            }

            #[inline(always)]
            fn value_from_uom_approx(
                self,
                quantity: Self::Quantity,
            ) -> Result<$crate::Decimal, $crate::MeasurementError> {
                let value = match self {
                    $(Self::$variant => quantity.get::<$uom_unit>(),)+
                };
                $crate::__private::decimal_from_f64_approx(value)
            }
        }
    };
    (
        $(#[$enum_attr:meta])*
        $visibility:vis enum $unit:ident for $quantity_name:literal {
            $(
                $(#[$variant_attr:meta])*
                $variant:ident => {
                    symbol: $symbol:literal;
                    coefficient: $numerator:literal $(/ $denominator:literal)?;
                    $(offset: $offset:literal;)?
                    $(aliases: [$($alias:literal),* $(,)?];)?
                }
            )+
        }
    ) => {
        $crate::__define_unit_family_core! {
            $(#[$enum_attr])*
            $visibility enum $unit for $quantity_name {
                $(
                    $(#[$variant_attr])*
                    $variant => {
                        symbol: $symbol;
                        coefficient: $numerator $(/ $denominator)?;
                        $(offset: $offset;)?
                        $(aliases: [$($alias),*];)?
                    }
                )+
            }
        }
    };
}
