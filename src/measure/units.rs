// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact unit families with an optional `uom` bridge.

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
#[cfg(feature = "uom")]
mod internal;
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
///
/// Accepts either one positive Decimal literal or a positive
/// `numerator / denominator` literal pair. The expansion returns the same
/// validation result as
/// [`ConversionFactor::from_integer`](crate::ConversionFactor::from_integer) or
/// [`ConversionFactor::new`](crate::ConversionFactor::new).
#[doc(hidden)]
#[macro_export]
macro_rules! __unit_factor {
    ($numerator:literal) => {
        $crate::ConversionFactor::from_integer(
            const {
                $crate::__private::decimal_from_literal(stringify!($numerator))
            },
        )
    };
    ($numerator:literal / $denominator:literal) => {
        $crate::ConversionFactor::new(
            const {
                $crate::__private::decimal_from_literal(stringify!($numerator))
            },
            const {
                $crate::__private::decimal_from_literal(stringify!($denominator))
            },
        )
    };
}

/// Produces an optional Decimal offset for an exported unit macro.
///
/// An empty invocation expands to [`Decimal::ZERO`](crate::Decimal::ZERO); one
/// Decimal literal expands to that exact value.
#[doc(hidden)]
#[macro_export]
macro_rules! __unit_offset {
    () => {
        $crate::Decimal::ZERO
    };
    ($offset:literal) => {
        const { $crate::__private::decimal_from_literal(stringify!($offset)) }
    };
}

/// Implements the exact unit metadata shared by public macro variants.
///
/// The expansion validates quantity, canonical-symbol, and alias metadata at
/// compilation, then generates the enum, [`Unit`](crate::Unit), display,
/// lenient `FromStr`, and canonical string Serde implementations.
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
                    definition: $definition:expr;
                    $(aliases: [$($alias:literal),* $(,)?];)?
                }
            )+
        }
    ) => {
        const _: () = {
            const SYMBOLS: &[&str] = &[
                $($symbol,)+
            ];
            const ALIASES: &[&str] = &[
                $(
                    $($($alias,)*)?
                )+
            ];
            $crate::__private::assert_unit_family_metadata(
                $quantity_name,
                SYMBOLS,
                ALIASES,
            );
        };

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

            #[inline(always)]
            fn definition(self) -> Result<$crate::UnitDefinition, $crate::MeasurementError> {
                match self {
                    $(Self::$variant => $definition,)+
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

/// Implements the approximate `uom` bridge when its Cargo feature is enabled.
///
/// The expansion maps each exact unit variant to its corresponding
/// `uom/f64` unit and converts at the explicit approximation boundary.
#[cfg(feature = "uom")]
#[doc(hidden)]
#[macro_export]
macro_rules! __define_uom_unit {
    (
        $unit:ident,
        $quantity_ty:ty,
        {
            $($variant:ident => $uom_unit:ty;)+
        }
    ) => {
        impl $crate::UomUnit for $unit {
            type Quantity = $quantity_ty;

            #[inline(always)]
            fn to_uom_approx(
                self,
                value: $crate::Decimal,
            ) -> Self::Quantity {
                let value = $crate::__private::decimal_to_f64_approx(value);
                match self {
                    $(
                        Self::$variant =>
                            <$quantity_ty>::new::<$uom_unit>(value),
                    )+
                }
            }

            #[inline(always)]
            fn value_from_uom_approx(
                self,
                quantity: Self::Quantity,
            ) -> Result<$crate::Decimal, $crate::MeasurementError> {
                let value = match self {
                    $(
                        Self::$variant => quantity.get::<$uom_unit>(),
                    )+
                };
                $crate::__private::decimal_from_f64_approx(value)
            }
        }
    };
}

/// Discards approximate bridge metadata when the `uom` feature is disabled.
///
/// This keeps exact unit-family macro invocations valid without compiling or
/// exposing any `uom` type.
#[cfg(not(feature = "uom"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __define_uom_unit {
    ($($tokens:tt)*) => {};
}

/// Defines an externally extensible unit family with exact Decimal factors.
///
/// The generated type implements [`Unit`](crate::Unit), display, lenient
/// `FromStr`, and canonical string Serde. Supplying `uom = Quantity` plus a
/// `uom` type for every variant also implements the optional approximate
/// `UomUnit` bridge when the `uom` Cargo feature is enabled. Exact `Unit`
/// generation is unconditional.
///
/// # Syntax and generated API
///
/// Each variant supplies a canonical `symbol` and either an exact `definition`
/// path or a positive Decimal `coefficient`, optionally written as a ratio and
/// followed by an `offset`. An optional `aliases` list enables lenient input.
/// Decimal literals support integer, fractional, scientific, digit-separated,
/// binary, octal, and hexadecimal forms accepted by
/// `rust_decimal_macros::dec!` through this macro's `$literal` grammar.
/// The `uom = Quantity` forms additionally require one `uom` unit type per
/// variant, but those tokens are used only when the `uom` feature is enabled.
///
/// The generated enum is non-exhaustive and implements [`Unit`](crate::Unit),
/// `Display`, lenient `FromStr`, and canonical string Serde. Its `all()` slice
/// is generated from every declared variant, so it is complete by
/// construction.
///
/// # Metadata contract
///
/// - the quantity is non-empty ASCII `snake_case`, starts with a lowercase
///   letter, and has no leading, trailing, or repeated underscores;
/// - the family is non-empty;
/// - canonical symbols are non-empty, unique, and contain no leading or
///   trailing Unicode whitespace;
/// - aliases are non-empty, unique among aliases, and contain no leading or
///   trailing Unicode whitespace;
/// - an alias may equal another variant's canonical symbol;
/// - canonical symbols are searched first and therefore win during parsing.
///
/// Violating a statically expressible metadata rule fails compilation. A
/// coefficient still returns
/// [`MeasurementError::InvalidUnitDefinition`](crate::MeasurementError::InvalidUnitDefinition)
/// if its Decimal factor is not positive when the generated definition is
/// requested.
///
/// # Examples
///
/// An alias-to-canonical collision is valid and the canonical owner wins:
///
/// ```
/// use qubit_measure::{
///     Unit,
///     define_unit_family,
/// };
///
/// define_unit_family! {
///     /// Unit family demonstrating canonical-symbol priority.
///     enum CollisionUnit for "collision_unit" {
///         /// Variant that owns the colliding alias.
///         AliasOwner => {
///             symbol: "alias-owner";
///             coefficient: 1;
///             aliases: ["canonical"];
///         }
///         /// Variant that owns the canonical symbol.
///         CanonicalOwner => {
///             symbol: "canonical";
///             coefficient: 1;
///         }
///     }
/// }
///
/// assert_eq!(
///     CollisionUnit::parse_lenient("canonical"),
///     Ok(CollisionUnit::CanonicalOwner),
/// );
/// ```
///
/// Duplicate canonical symbols are rejected at compilation:
///
/// ```compile_fail
/// use qubit_measure::define_unit_family;
///
/// define_unit_family! {
///     /// Invalid family with duplicate canonical symbols.
///     enum DuplicateSymbolUnit for "duplicate_symbol_unit" {
///         /// First duplicate owner.
///         First => {
///             symbol: "x";
///             coefficient: 1;
///         }
///         /// Second duplicate owner.
///         Second => {
///             symbol: "x";
///             coefficient: 1;
///         }
///     }
/// }
/// ```
///
/// Duplicate aliases are also rejected at compilation:
///
/// ```compile_fail
/// use qubit_measure::define_unit_family;
///
/// define_unit_family! {
///     /// Invalid family with duplicate aliases.
///     enum DuplicateAliasUnit for "duplicate_alias_unit" {
///         /// First duplicate alias owner.
///         First => {
///             symbol: "first";
///             coefficient: 1;
///             aliases: ["duplicate"];
///         }
///         /// Second duplicate alias owner.
///         Second => {
///             symbol: "second";
///             coefficient: 1;
///             aliases: ["duplicate"];
///         }
///     }
/// }
/// ```
///
/// Canonical symbols with surrounding whitespace are rejected at compilation:
///
/// ```compile_fail
/// use qubit_measure::define_unit_family;
///
/// define_unit_family! {
///     /// Invalid family with surrounding canonical whitespace.
///     enum WhitespaceSymbolUnit for "whitespace_symbol_unit" {
///         /// Invalid canonical owner.
///         Invalid => {
///             symbol: " m";
///             coefficient: 1;
///         }
///     }
/// }
/// ```
///
/// Aliases with surrounding whitespace are rejected at compilation:
///
/// ```compile_fail
/// use qubit_measure::define_unit_family;
///
/// define_unit_family! {
///     /// Invalid family with surrounding alias whitespace.
///     enum WhitespaceAliasUnit for "whitespace_alias_unit" {
///         /// Invalid alias owner.
///         Invalid => {
///             symbol: "m";
///             coefficient: 1;
///             aliases: ["meter "];
///         }
///     }
/// }
/// ```
///
/// Decimal coefficients that exceed the representable range are rejected at
/// compilation:
///
/// ```compile_fail
/// use qubit_measure::define_unit_family;
///
/// define_unit_family! {
///     /// Invalid family with an unrepresentable coefficient.
///     enum UnrepresentableCoefficientUnit for "unrepresentable_coefficient_unit" {
///         /// Unit whose coefficient exceeds Decimal's 96-bit mantissa.
///         Invalid => {
///             symbol: "invalid";
///             coefficient: 79_228_162_514_264_337_593_543_950_336;
///         }
///     }
/// }
/// ```
///
/// Non-numeric literals are rejected at compilation:
///
/// ```compile_fail
/// use qubit_measure::define_unit_family;
///
/// define_unit_family! {
///     /// Invalid family with a non-numeric coefficient.
///     enum NonNumericCoefficientUnit for "non_numeric_coefficient_unit" {
///         /// Unit whose coefficient is not numeric.
///         Invalid => {
///             symbol: "invalid";
///             coefficient: "not-a-number";
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! define_unit_family {
    (
        $(#[$enum_attr:meta])*
        $visibility:vis enum $unit:ident for $quantity_name:literal, uom = $quantity_ty:ty {
            $(
                $(#[$variant_attr:meta])*
                $variant:ident => {
                    symbol: $symbol:literal;
                    definition: $definition:path;
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
                        definition: Ok($definition);
                        $(aliases: [$($alias),*];)?
                    }
                )+
            }
        }

        $crate::__define_uom_unit! {
            $unit,
            $quantity_ty,
            {
                $($variant => $uom_unit;)+
            }
        }
    };
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
                        definition: {
                            let factor = $crate::__unit_factor!(
                                $numerator $(/ $denominator)?
                            )?;
                            Ok($crate::UnitDefinition::new(
                                factor,
                                $crate::__unit_offset!($($offset)?),
                            ))
                        };
                        $(aliases: [$($alias),*];)?
                    }
                )+
            }
        }

        $crate::__define_uom_unit! {
            $unit,
            $quantity_ty,
            {
                $($variant => $uom_unit;)+
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
                    definition: $definition:path;
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
                        definition: Ok($definition);
                        $(aliases: [$($alias),*];)?
                    }
                )+
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
                        definition: {
                            let factor = $crate::__unit_factor!(
                                $numerator $(/ $denominator)?
                            )?;
                            Ok($crate::UnitDefinition::new(
                                factor,
                                $crate::__unit_offset!($($offset)?),
                            ))
                        };
                        $(aliases: [$($alias),*];)?
                    }
                )+
            }
        }
    };
}
