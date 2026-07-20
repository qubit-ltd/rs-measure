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
/// [`ConversionFactor::from_decimal`](crate::ConversionFactor::from_decimal) or
/// [`ConversionFactor::new`](crate::ConversionFactor::new).
#[doc(hidden)]
#[macro_export]
macro_rules! __unit_factor {
    ($numerator:literal) => {
        $crate::ConversionFactor::from_decimal(
            const {
                $crate::__private::positive_decimal_from_literal(stringify!(
                    $numerator
                ))
            },
        )
    };
    ($numerator:literal / $denominator:literal) => {
        $crate::ConversionFactor::new(
            const {
                $crate::__private::positive_decimal_from_literal(stringify!(
                    $numerator
                ))
            },
            const {
                $crate::__private::positive_decimal_from_literal(stringify!(
                    $denominator
                ))
            },
        )
    };
}

/// Produces an optional Decimal offset for an exported unit macro.
///
/// An empty invocation expands to an exact Decimal zero; one Decimal literal
/// expands to that exact value.
#[doc(hidden)]
#[macro_export]
macro_rules! __unit_offset {
    () => {
        const { $crate::__private::decimal_from_literal("0") }
    };
    ($offset:literal) => {
        const { $crate::__private::decimal_from_literal(stringify!($offset)) }
    };
}

/// Implements the exact unit metadata shared by public macro variants.
///
/// The expansion validates quantity, canonical-symbol, and alias metadata at
/// compilation, then generates the enum, [`Unit`](crate::Unit), display,
/// strict `FromStr`, and canonical-only string Serde implementations.
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
        #[must_use]
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

            #[inline(always)]
            fn from_str(input: &str) -> Result<Self, Self::Err> {
                <Self as $crate::Unit>::parse_strict(input)
            }
        }

        impl ::serde::Serialize for $unit {
            #[inline]
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                serializer.serialize_str(<Self as $crate::Unit>::symbol(*self))
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $unit {
            #[inline]
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                let symbol = <::std::string::String as
                    ::serde::Deserialize>::deserialize(deserializer)?;
                <Self as $crate::Unit>::parse_strict(&symbol)
                    .map_err(::serde::de::Error::custom)
            }
        }
    };
}

/// Implements an explicit approximate `uom` bridge for a unit family.
///
/// Invoke this macro only when `qubit-measure`'s `uom` feature is enabled and
/// the supplied `uom` quantity and SI base-unit types are available. The base
/// unit must represent the abstract base used by every
/// [`UnitDefinition`](crate::UnitDefinition)
/// returned by the family. The bridge applies that exact definition before
/// crossing `f64`; reading the resulting quantity through another `uom` unit
/// then follows `uom`'s own coefficient for that unit. The generated
/// required `UomUnit::try_to_uom_approx` method returns a unit-definition
/// error, while the trait's default `to_uom_approx` convenience method panics
/// for the same invalid external definition. Keeping this bridge separate from
/// [`define_unit_family!`](crate::define_unit_family) prevents Cargo feature
/// unification from changing whether an exact-only unit declaration resolves
/// optional `uom` paths.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "uom")]
/// # {
/// use qubit_measure::{
///     UomUnit,
///     define_unit_family,
///     impl_uom_unit,
/// };
/// use uom::si::{
///     f64::Length as UomLength,
///     length::meter,
/// };
///
/// define_unit_family! {
///     /// Example external length unit.
///     enum ExampleLength for "example_length" {
///         /// Meter.
///         Meter => { symbol: "m"; coefficient: 1; }
///     }
/// }
///
/// impl_uom_unit! {
///     ExampleLength, UomLength {
///         base: meter;
///     }
/// }
///
/// fn assert_bridge<U: UomUnit>() {}
/// assert_bridge::<ExampleLength>();
/// # }
/// ```
#[macro_export]
macro_rules! impl_uom_unit {
    (
        $unit:ident,
        $quantity_ty:ty {
            base: $uom_base_unit:ty;
        }
    ) => {
        impl $crate::UomUnit for $unit {
            type Quantity = $quantity_ty;

            #[inline]
            fn try_to_uom_approx(
                self,
                value: ::rust_decimal::Decimal,
            ) -> Result<Self::Quantity, $crate::MeasurementError> {
                let definition = $crate::Unit::definition(self)?;
                let base_value = $crate::__private::unit_value_to_base_f64(
                    value, definition,
                );
                Ok(<$quantity_ty>::new::<$uom_base_unit>(base_value))
            }

            #[inline]
            fn value_from_uom_approx(
                self,
                quantity: Self::Quantity,
            ) -> Result<::rust_decimal::Decimal, $crate::MeasurementError> {
                let definition = $crate::Unit::definition(self)?;
                let base_value = quantity.get::<$uom_base_unit>();
                $crate::__private::base_f64_to_unit_value(
                    base_value, definition,
                )
            }
        }
    };
}

/// Defines an externally extensible unit family with exact Decimal factors.
///
/// The generated type implements [`Unit`](crate::Unit), display, strict
/// `FromStr`, and canonical-only string Serde. Use
/// [`impl_uom_unit!`](crate::impl_uom_unit) separately to add an optional
/// approximate `UomUnit` bridge.
///
/// # Syntax and generated API
///
/// Each variant supplies a canonical `symbol`. One macro invocation uses either
/// exact `definition` paths for every variant or positive Decimal `coefficient`
/// values for every variant; the two forms cannot be mixed within one family.
/// A coefficient may be written as a ratio and followed by an `offset`. An
/// optional `aliases` list enables lenient input.
/// The supported Decimal literal subset includes integer, fractional,
/// scientific, and digit-separated decimal forms plus binary, octal, and
/// hexadecimal integers. Representability is decided from the final value
/// after scientific-exponent and coefficient-zero cancellation. Every accepted
/// value must fit Decimal exactly, without rounding, and representable input
/// scale is preserved as far as Decimal's coefficient and scale allow.
/// The generated enum is non-exhaustive and implements [`Unit`](crate::Unit),
/// `Display`, strict `FromStr`, and canonical-only string Serde. Its `all()`
/// slice is generated from every declared variant, so it is complete by
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
/// - aliases do not match any canonical symbol in the family;
/// - canonical symbols and aliases beginning with `.`, `+`, or `-` require
///   whitespace after a measurement's Decimal value; their compact forms are
///   rejected as ambiguous numeric boundaries (for example, use `"1.25 +cu"`).
///
/// Violating a statically expressible metadata rule fails compilation.
/// Coefficient numerator and denominator literals must both be positive.
///
/// # Examples
///
/// A reusable non-identity definition can be built in an external const
/// context and supplied by path:
///
/// ```
/// use qubit_measure::{
///     ConversionFactor,
///     Unit,
///     UnitDefinition,
///     define_unit_family,
/// };
/// use rust_decimal::Decimal;
///
/// const TWO_THIRDS: UnitDefinition = UnitDefinition::new(
///     ConversionFactor::from_const_integers(2, 3),
///     Decimal::ZERO,
/// );
///
/// define_unit_family! {
///     /// Unit family backed by a reusable definition.
///     enum DefinitionUnit for "definition_unit" {
///         /// Unit with a two-thirds base-unit factor.
///         TwoThirds => {
///             symbol: "two-thirds";
///             definition: TWO_THIRDS;
///         }
///     }
/// }
///
/// let definition = DefinitionUnit::TwoThirds
///     .definition()
///     .expect("const definition should be valid");
/// assert_eq!(definition.factor().numerator(), Decimal::new(2, 0));
/// assert_eq!(definition.factor().denominator(), Decimal::new(3, 0));
/// ```
///
/// An alias-to-canonical collision is rejected at compilation:
///
/// ```compile_fail
/// use qubit_measure::define_unit_family;
///
/// define_unit_family! {
///     /// Invalid family with an alias-to-canonical collision.
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
/// An alias that repeats its own canonical symbol is rejected at compilation:
///
/// ```compile_fail
/// use qubit_measure::define_unit_family;
///
/// define_unit_family! {
///     /// Invalid family with a redundant self alias.
///     enum SelfAliasUnit for "self_alias_unit" {
///         /// Unit whose alias repeats its canonical symbol.
///         Invalid => {
///             symbol: "m";
///             coefficient: 1;
///             aliases: ["m"];
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
/// Zero coefficient numerators are rejected at compilation:
///
/// ```compile_fail
/// use qubit_measure::define_unit_family;
///
/// define_unit_family! {
///     /// Invalid family with a zero coefficient numerator.
///     enum ZeroNumeratorUnit for "zero_numerator_unit" {
///         /// Unit whose coefficient is zero.
///         Invalid => { symbol: "invalid"; coefficient: 0; }
///     }
/// }
/// ```
///
/// Negative coefficient numerators are rejected at compilation:
///
/// ```compile_fail
/// use qubit_measure::define_unit_family;
///
/// define_unit_family! {
///     /// Invalid family with a negative coefficient numerator.
///     enum NegativeNumeratorUnit for "negative_numerator_unit" {
///         /// Unit whose coefficient is negative.
///         Invalid => { symbol: "invalid"; coefficient: -1; }
///     }
/// }
/// ```
///
/// Zero coefficient denominators are rejected at compilation:
///
/// ```compile_fail
/// use qubit_measure::define_unit_family;
///
/// define_unit_family! {
///     /// Invalid family with a zero coefficient denominator.
///     enum ZeroDenominatorUnit for "zero_denominator_unit" {
///         /// Unit whose denominator is zero.
///         Invalid => { symbol: "invalid"; coefficient: 1 / 0; }
///     }
/// }
/// ```
///
/// Negative coefficient denominators are rejected at compilation:
///
/// ```compile_fail
/// use qubit_measure::define_unit_family;
///
/// define_unit_family! {
///     /// Invalid family with a negative coefficient denominator.
///     enum NegativeDenominatorUnit for "negative_denominator_unit" {
///         /// Unit whose denominator is negative.
///         Invalid => { symbol: "invalid"; coefficient: 1 / -1; }
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
