// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Mathematical and unit-conversion constants used throughout this crate.
//!
//! Constants are grouped by quantity. Exact rational definitions follow the
//! [BIPM SI Brochure], [NIST unit-conversion references], [NIST Handbook 44],
//! and the [2022 CODATA recommended values]. U.S. customary area, volume,
//! velocity, and derived definitions are calculated from the exact
//! international foot, pound, and gallon relationships in those references.
//!
//! Definitions involving the irrational constant pi are necessarily finite
//! approximations. Their digits follow [NIST DLMF section 3.12]; pi retains 23
//! decimal places so the derived arcsecond denominator remains within
//! Decimal's 96-bit mantissa, while square degree uses 28 decimal places.
//! The crate's versioned, quantity-family provenance matrix is maintained in
//! `doc/unit-definition-provenance.tsv`.
//!
//! [BIPM SI Brochure]: https://www.bipm.org/en/publications/si-brochure
//! [NIST unit-conversion references]: https://www.nist.gov/pml/owm/metric-si/unit-conversion
//! [NIST Handbook 44]: https://www.nist.gov/pml/owm/nist-handbook-44-current-edition
//! [2022 CODATA recommended values]: https://physics.nist.gov/cuu/Constants/
//! [NIST DLMF section 3.12]: https://dlmf.nist.gov/3.12

/// Decimal denominator used for the finite 23-place representation of pi.
const PI_DECIMAL_DENOMINATOR: i128 = 100_000_000_000_000_000_000_000;

/// Rounded numerator for pi from NIST DLMF section 3.12.
const PI_DECIMAL_NUMERATOR: i128 = 314_159_265_358_979_323_846_264;

/// Decimal denominator used for the finite representation of tau.
const TAU_DECIMAL_DENOMINATOR: i128 = PI_DECIMAL_DENOMINATOR;

/// Numerator that keeps finite tau exactly equal to twice finite pi.
const TAU_DECIMAL_NUMERATOR: i128 = 2 * PI_DECIMAL_NUMERATOR;

/// Ensures the finite standard-library representations remain coherent.
const _: () = assert!(
    TAU_DECIMAL_DENOMINATOR == PI_DECIMAL_DENOMINATOR
        && TAU_DECIMAL_NUMERATOR == 2 * PI_DECIMAL_NUMERATOR,
);

/// Builds an exact Decimal constant and validates its representation.
macro_rules! decimal {
    ($mantissa:expr, $scale:expr) => {{
        let mantissa: i128 = $mantissa;
        let scale: u32 = $scale;
        let negative = mantissa < 0;
        let magnitude = if negative {
            (-mantissa) as u128
        } else {
            mantissa as u128
        };
        assert!(magnitude >> 96 == 0);
        assert!(scale <= rust_decimal::Decimal::MAX_SCALE);
        rust_decimal::Decimal::from_parts(
            magnitude as u32,
            (magnitude >> 32) as u32,
            (magnitude >> 64) as u32,
            negative,
            scale,
        )
    }};
}

/// Builds a compile-time unit definition from validated numeric constants.
macro_rules! definition {
    ($numerator:expr, $denominator:expr, $offset_mantissa:expr, $offset_scale:expr $(,)?) => {{
        let numerator: i128 = $numerator;
        let denominator: i128 = $denominator;
        let factor = crate::measure::ConversionFactor::from_const_integers(numerator, denominator);
        crate::measure::UnitDefinition::new(factor, decimal!($offset_mantissa, $offset_scale))
    }};
}

/// Conversion constants for acceleration units.
pub(crate) mod acceleration {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `MillimeterPerSecondSquared` unit.
    pub(crate) const MILLIMETER_PER_SECOND_SQUARED: UnitDefinition = definition!(1, 1000, 0, 0);
    /// Exact conversion definition for the `MeterPerSecondSquared` unit.
    pub(crate) const METER_PER_SECOND_SQUARED: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `FootPerSecondSquared` unit.
    pub(crate) const FOOT_PER_SECOND_SQUARED: UnitDefinition = definition!(381, 1250, 0, 0);
    /// Exact conversion definition for the `StandardGravity` unit.
    pub(crate) const STANDARD_GRAVITY: UnitDefinition = definition!(196133, 20000, 0, 0);
}

/// Conversion constants for amount of substance units.
pub(crate) mod amount_of_substance {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `Micromole` unit.
    pub(crate) const MICROMOLE: UnitDefinition = definition!(1, 1000000, 0, 0);
    /// Exact conversion definition for the `Millimole` unit.
    pub(crate) const MILLIMOLE: UnitDefinition = definition!(1, 1000, 0, 0);
    /// Exact conversion definition for the `Mole` unit.
    pub(crate) const MOLE: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `Kilomole` unit.
    pub(crate) const KILOMOLE: UnitDefinition = definition!(1000, 1, 0, 0);
    /// Exact conversion definition for the `Particle` unit.
    pub(crate) const PARTICLE: UnitDefinition = definition!(1, 602214076000000000000000, 0, 0);
}

/// Conversion constants for angle units.
pub(crate) mod angle {
    use super::PI_DECIMAL_DENOMINATOR;
    use super::PI_DECIMAL_NUMERATOR;
    use super::TAU_DECIMAL_DENOMINATOR;
    use super::TAU_DECIMAL_NUMERATOR;
    use crate::measure::UnitDefinition;

    /// Number of degrees in one revolution.
    pub(crate) const DEGREES_PER_REVOLUTION: i128 = 360;
    /// Number of arcminutes in one degree.
    pub(crate) const ARC_MINUTES_PER_DEGREE: i128 = 60;
    /// Number of arcseconds in one arcminute.
    pub(crate) const ARC_SECONDS_PER_MINUTE: i128 = 60;

    /// Exact conversion definition for the `Radian` unit.
    pub(crate) const RADIAN: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `Degree` unit.
    pub(crate) const DEGREE: UnitDefinition = definition!(
        PI_DECIMAL_NUMERATOR,
        PI_DECIMAL_DENOMINATOR * DEGREES_PER_REVOLUTION / 2,
        0,
        0,
    );
    /// Exact conversion definition for the `Revolution` unit.
    pub(crate) const REVOLUTION: UnitDefinition =
        definition!(TAU_DECIMAL_NUMERATOR, TAU_DECIMAL_DENOMINATOR, 0, 0,);
    /// Exact conversion definition for the `Minute` unit.
    pub(crate) const MINUTE: UnitDefinition = definition!(
        PI_DECIMAL_NUMERATOR,
        PI_DECIMAL_DENOMINATOR * DEGREES_PER_REVOLUTION * ARC_MINUTES_PER_DEGREE / 2,
        0,
        0,
    );
    /// Exact conversion definition for the `Second` unit.
    pub(crate) const SECOND: UnitDefinition = definition!(
        PI_DECIMAL_NUMERATOR,
        PI_DECIMAL_DENOMINATOR
            * DEGREES_PER_REVOLUTION
            * ARC_MINUTES_PER_DEGREE
            * ARC_SECONDS_PER_MINUTE
            / 2,
        0,
        0,
    );
}

/// Conversion constants for angular velocity units.
pub(crate) mod angular_velocity {
    use super::PI_DECIMAL_DENOMINATOR;
    use super::PI_DECIMAL_NUMERATOR;
    use super::TAU_DECIMAL_DENOMINATOR;
    use super::TAU_DECIMAL_NUMERATOR;
    use super::angle;
    use super::time;
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `RadianPerSecond` unit.
    pub(crate) const RADIAN_PER_SECOND: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `DegreePerSecond` unit.
    pub(crate) const DEGREE_PER_SECOND: UnitDefinition = definition!(
        PI_DECIMAL_NUMERATOR,
        PI_DECIMAL_DENOMINATOR * angle::DEGREES_PER_REVOLUTION / 2,
        0,
        0,
    );
    /// Exact conversion definition for the `RevolutionPerSecond` unit.
    pub(crate) const REVOLUTION_PER_SECOND: UnitDefinition =
        definition!(TAU_DECIMAL_NUMERATOR, TAU_DECIMAL_DENOMINATOR, 0, 0,);
    /// Exact conversion definition for the `RevolutionPerMinute` unit.
    pub(crate) const REVOLUTION_PER_MINUTE: UnitDefinition = definition!(
        TAU_DECIMAL_NUMERATOR,
        TAU_DECIMAL_DENOMINATOR * time::SECONDS_PER_MINUTE,
        0,
        0,
    );
}

/// Conversion constants for area units.
pub(crate) mod area {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `SquareMillimeter` unit.
    pub(crate) const SQUARE_MILLIMETER: UnitDefinition = definition!(1, 1000000, 0, 0);
    /// Exact conversion definition for the `SquareCentimeter` unit.
    pub(crate) const SQUARE_CENTIMETER: UnitDefinition = definition!(1, 10000, 0, 0);
    /// Exact conversion definition for the `SquareMeter` unit.
    pub(crate) const SQUARE_METER: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `SquareKilometer` unit.
    pub(crate) const SQUARE_KILOMETER: UnitDefinition = definition!(1000000, 1, 0, 0);
    /// Exact conversion definition for the `Hectare` unit.
    pub(crate) const HECTARE: UnitDefinition = definition!(10000, 1, 0, 0);
    /// Exact conversion definition for the `Acre` unit.
    pub(crate) const ACRE: UnitDefinition = definition!(316160658, 78125, 0, 0);
    /// Exact conversion definition for the `SquareInch` unit.
    pub(crate) const SQUARE_INCH: UnitDefinition = definition!(16129, 25000000, 0, 0);
    /// Exact conversion definition for the `SquareFoot` unit.
    pub(crate) const SQUARE_FOOT: UnitDefinition = definition!(145161, 1562500, 0, 0);
    /// Exact conversion definition for the `SquareYard` unit.
    pub(crate) const SQUARE_YARD: UnitDefinition = definition!(1306449, 1562500, 0, 0);
    /// Exact conversion definition for the `SquareMile` unit.
    pub(crate) const SQUARE_MILE: UnitDefinition = definition!(40468564224, 15625, 0, 0);
}

/// Conversion constants for capacitance units.
pub(crate) mod capacitance {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `Picofarad` unit.
    pub(crate) const PICOFARAD: UnitDefinition = definition!(1, 1000000000000, 0, 0);
    /// Exact conversion definition for the `Nanofarad` unit.
    pub(crate) const NANOFARAD: UnitDefinition = definition!(1, 1000000000, 0, 0);
    /// Exact conversion definition for the `Microfarad` unit.
    pub(crate) const MICROFARAD: UnitDefinition = definition!(1, 1000000, 0, 0);
    /// Exact conversion definition for the `Millifarad` unit.
    pub(crate) const MILLIFARAD: UnitDefinition = definition!(1, 1000, 0, 0);
    /// Exact conversion definition for the `Farad` unit.
    pub(crate) const FARAD: UnitDefinition = definition!(1, 1, 0, 0);
}

/// Conversion constants for catalytic activity units.
pub(crate) mod catalytic_activity {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `Microkatal` unit.
    pub(crate) const MICROKATAL: UnitDefinition = definition!(1, 1000000, 0, 0);
    /// Exact conversion definition for the `Millikatal` unit.
    pub(crate) const MILLIKATAL: UnitDefinition = definition!(1, 1000, 0, 0);
    /// Exact conversion definition for the `Katal` unit.
    pub(crate) const KATAL: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `EnzymeUnit` unit.
    pub(crate) const ENZYME_UNIT: UnitDefinition = definition!(1, 60000000, 0, 0);
    /// Exact conversion definition for the `MilliEnzymeUnit` unit.
    pub(crate) const MILLI_ENZYME_UNIT: UnitDefinition = definition!(1, 60000000000, 0, 0);
}

/// Conversion constants for catalytic activity concentration units.
pub(crate) mod catalytic_activity_concentration {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `KatalPerCubicMeter` unit.
    pub(crate) const KATAL_PER_CUBIC_METER: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `EnzymeUnitPerLiter` unit.
    pub(crate) const ENZYME_UNIT_PER_LITER: UnitDefinition = definition!(1, 60000, 0, 0);
    /// Exact conversion definition for the `MilliEnzymeUnitPerMilliliter` unit.
    pub(crate) const MILLI_ENZYME_UNIT_PER_MILLILITER: UnitDefinition = definition!(1, 60000, 0, 0);
}

/// Conversion constants for dynamic viscosity units.
pub(crate) mod dynamic_viscosity {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `MicropascalSecond` unit.
    pub(crate) const MICROPASCAL_SECOND: UnitDefinition = definition!(1, 1000000, 0, 0);
    /// Exact conversion definition for the `MillipascalSecond` unit.
    pub(crate) const MILLIPASCAL_SECOND: UnitDefinition = definition!(1, 1000, 0, 0);
    /// Exact conversion definition for the `PascalSecond` unit.
    pub(crate) const PASCAL_SECOND: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `Poise` unit.
    pub(crate) const POISE: UnitDefinition = definition!(1, 10, 0, 0);
    /// Exact conversion definition for the `Centipoise` unit.
    pub(crate) const CENTIPOISE: UnitDefinition = definition!(1, 1000, 0, 0);
}

/// Conversion constants for electric charge units.
pub(crate) mod electric_charge {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `Microcoulomb` unit.
    pub(crate) const MICROCOULOMB: UnitDefinition = definition!(1, 1000000, 0, 0);
    /// Exact conversion definition for the `Millicoulomb` unit.
    pub(crate) const MILLICOULOMB: UnitDefinition = definition!(1, 1000, 0, 0);
    /// Exact conversion definition for the `Coulomb` unit.
    pub(crate) const COULOMB: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `Kilocoulomb` unit.
    pub(crate) const KILOCOULOMB: UnitDefinition = definition!(1000, 1, 0, 0);
    /// Exact conversion definition for the `AmpereHour` unit.
    pub(crate) const AMPERE_HOUR: UnitDefinition = definition!(3600, 1, 0, 0);
    /// Exact conversion definition for the `MilliampereHour` unit.
    pub(crate) const MILLIAMPERE_HOUR: UnitDefinition = definition!(18, 5, 0, 0);
}

/// Conversion constants for electric current units.
pub(crate) mod electric_current {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `Picoampere` unit.
    pub(crate) const PICOAMPERE: UnitDefinition = definition!(1, 1000000000000, 0, 0);
    /// Exact conversion definition for the `Nanoampere` unit.
    pub(crate) const NANOAMPERE: UnitDefinition = definition!(1, 1000000000, 0, 0);
    /// Exact conversion definition for the `Microampere` unit.
    pub(crate) const MICROAMPERE: UnitDefinition = definition!(1, 1000000, 0, 0);
    /// Exact conversion definition for the `Milliampere` unit.
    pub(crate) const MILLIAMPERE: UnitDefinition = definition!(1, 1000, 0, 0);
    /// Exact conversion definition for the `Ampere` unit.
    pub(crate) const AMPERE: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `Kiloampere` unit.
    pub(crate) const KILOAMPERE: UnitDefinition = definition!(1000, 1, 0, 0);
    /// Exact conversion definition for the `Megaampere` unit.
    pub(crate) const MEGAAMPERE: UnitDefinition = definition!(1000000, 1, 0, 0);
}

/// Conversion constants for electric current density units.
pub(crate) mod electric_current_density {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `AmperePerSquareMeter` unit.
    pub(crate) const AMPERE_PER_SQUARE_METER: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `AmperePerSquareCentimeter` unit.
    pub(crate) const AMPERE_PER_SQUARE_CENTIMETER: UnitDefinition = definition!(10000, 1, 0, 0);
    /// Exact conversion definition for the `AmperePerSquareMillimeter` unit.
    pub(crate) const AMPERE_PER_SQUARE_MILLIMETER: UnitDefinition = definition!(1000000, 1, 0, 0);
}

/// Conversion constants for electric field units.
pub(crate) mod electric_field {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `VoltPerMeter` unit.
    pub(crate) const VOLT_PER_METER: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `VoltPerCentimeter` unit.
    pub(crate) const VOLT_PER_CENTIMETER: UnitDefinition = definition!(100, 1, 0, 0);
    /// Exact conversion definition for the `VoltPerMillimeter` unit.
    pub(crate) const VOLT_PER_MILLIMETER: UnitDefinition = definition!(1000, 1, 0, 0);
    /// Exact conversion definition for the `VoltPerMicrometer` unit.
    pub(crate) const VOLT_PER_MICROMETER: UnitDefinition = definition!(1000000, 1, 0, 0);
    /// Exact conversion definition for the `KilovoltPerMillimeter` unit.
    pub(crate) const KILOVOLT_PER_MILLIMETER: UnitDefinition = definition!(1000000, 1, 0, 0);
    /// Exact conversion definition for the `MegavoltPerMeter` unit.
    pub(crate) const MEGAVOLT_PER_METER: UnitDefinition = definition!(1000000, 1, 0, 0);
}

/// Conversion constants for electric potential units.
pub(crate) mod electric_potential {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `Nanovolt` unit.
    pub(crate) const NANOVOLT: UnitDefinition = definition!(1, 1000000000, 0, 0);
    /// Exact conversion definition for the `Microvolt` unit.
    pub(crate) const MICROVOLT: UnitDefinition = definition!(1, 1000000, 0, 0);
    /// Exact conversion definition for the `Millivolt` unit.
    pub(crate) const MILLIVOLT: UnitDefinition = definition!(1, 1000, 0, 0);
    /// Exact conversion definition for the `Volt` unit.
    pub(crate) const VOLT: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `Kilovolt` unit.
    pub(crate) const KILOVOLT: UnitDefinition = definition!(1000, 1, 0, 0);
    /// Exact conversion definition for the `Megavolt` unit.
    pub(crate) const MEGAVOLT: UnitDefinition = definition!(1000000, 1, 0, 0);
}

/// Conversion constants for electrical conductance units.
pub(crate) mod electrical_conductance {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `Microsiemens` unit.
    pub(crate) const MICROSIEMENS: UnitDefinition = definition!(1, 1000000, 0, 0);
    /// Exact conversion definition for the `Millisiemens` unit.
    pub(crate) const MILLISIEMENS: UnitDefinition = definition!(1, 1000, 0, 0);
    /// Exact conversion definition for the `Siemens` unit.
    pub(crate) const SIEMENS: UnitDefinition = definition!(1, 1, 0, 0);
}

/// Conversion constants for electrical conductivity units.
pub(crate) mod electrical_conductivity {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `SiemensPerMeter` unit.
    pub(crate) const SIEMENS_PER_METER: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `SiemensPerCentimeter` unit.
    pub(crate) const SIEMENS_PER_CENTIMETER: UnitDefinition = definition!(100, 1, 0, 0);
}

/// Conversion constants for electrical resistance units.
pub(crate) mod electrical_resistance {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `Microohm` unit.
    pub(crate) const MICROOHM: UnitDefinition = definition!(1, 1000000, 0, 0);
    /// Exact conversion definition for the `Milliohm` unit.
    pub(crate) const MILLIOHM: UnitDefinition = definition!(1, 1000, 0, 0);
    /// Exact conversion definition for the `Ohm` unit.
    pub(crate) const OHM: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `Kiloohm` unit.
    pub(crate) const KILOOHM: UnitDefinition = definition!(1000, 1, 0, 0);
    /// Exact conversion definition for the `Megaohm` unit.
    pub(crate) const MEGAOHM: UnitDefinition = definition!(1000000, 1, 0, 0);
    /// Exact conversion definition for the `Gigaohm` unit.
    pub(crate) const GIGAOHM: UnitDefinition = definition!(1000000000, 1, 0, 0);
}

/// Conversion constants for electrical resistivity units.
pub(crate) mod electrical_resistivity {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `MilliohmMeter` unit.
    pub(crate) const MILLIOHM_METER: UnitDefinition = definition!(1, 1000, 0, 0);
    /// Exact conversion definition for the `OhmMeter` unit.
    pub(crate) const OHM_METER: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `OhmCentimeter` unit.
    pub(crate) const OHM_CENTIMETER: UnitDefinition = definition!(1, 100, 0, 0);
    /// Exact conversion definition for the `OhmSquareMillimeterPerMeter` unit.
    pub(crate) const OHM_SQUARE_MILLIMETER_PER_METER: UnitDefinition =
        definition!(1, 1000000, 0, 0);
}

/// Conversion constants for energy units.
pub(crate) mod energy {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `Joule` unit.
    pub(crate) const JOULE: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `Kilojoule` unit.
    pub(crate) const KILOJOULE: UnitDefinition = definition!(1000, 1, 0, 0);
    /// Exact conversion definition for the `Megajoule` unit.
    pub(crate) const MEGAJOULE: UnitDefinition = definition!(1000000, 1, 0, 0);
    /// Exact conversion definition for the `WattHour` unit.
    pub(crate) const WATT_HOUR: UnitDefinition = definition!(3600, 1, 0, 0);
    /// Exact conversion definition for the `KilowattHour` unit.
    pub(crate) const KILOWATT_HOUR: UnitDefinition = definition!(3600000, 1, 0, 0);
    /// Exact conversion definition for the `Electronvolt` unit.
    pub(crate) const ELECTRONVOLT: UnitDefinition =
        definition!(801088317, 5000000000000000000000000000, 0, 0);
    /// Exact conversion definition for the `ThermochemicalCalorie` unit.
    pub(crate) const THERMOCHEMICAL_CALORIE: UnitDefinition = definition!(523, 125, 0, 0);
    /// Exact conversion definition for the `ThermochemicalKilocalorie` unit.
    pub(crate) const THERMOCHEMICAL_KILOCALORIE: UnitDefinition = definition!(4184, 1, 0, 0);
    /// Exact conversion definition for the
    /// `BritishThermalUnitInternationalTable` unit.
    pub(crate) const BRITISH_THERMAL_UNIT_INTERNATIONAL_TABLE: UnitDefinition =
        definition!(131882, 125, 0, 0);
}

/// Conversion constants for force units.
pub(crate) mod force {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `Millinewton` unit.
    pub(crate) const MILLINEWTON: UnitDefinition = definition!(1, 1000, 0, 0);
    /// Exact conversion definition for the `Newton` unit.
    pub(crate) const NEWTON: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `Kilonewton` unit.
    pub(crate) const KILONEWTON: UnitDefinition = definition!(1000, 1, 0, 0);
    /// Exact conversion definition for the `Meganewton` unit.
    pub(crate) const MEGANEWTON: UnitDefinition = definition!(1000000, 1, 0, 0);
    /// Exact conversion definition for the `GramForce` unit.
    pub(crate) const GRAM_FORCE: UnitDefinition = definition!(196133, 20000000, 0, 0);
    /// Exact conversion definition for the `KilogramForce` unit.
    pub(crate) const KILOGRAM_FORCE: UnitDefinition = definition!(196133, 20000, 0, 0);
    /// Exact conversion definition for the `PoundForce` unit.
    pub(crate) const POUND_FORCE: UnitDefinition = definition!(8896443230521, 2000000000000, 0, 0);
}

/// Conversion constants for frequency units.
pub(crate) mod frequency {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `Hertz` unit.
    pub(crate) const HERTZ: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `Kilohertz` unit.
    pub(crate) const KILOHERTZ: UnitDefinition = definition!(1000, 1, 0, 0);
    /// Exact conversion definition for the `Megahertz` unit.
    pub(crate) const MEGAHERTZ: UnitDefinition = definition!(1000000, 1, 0, 0);
    /// Exact conversion definition for the `Gigahertz` unit.
    pub(crate) const GIGAHERTZ: UnitDefinition = definition!(1000000000, 1, 0, 0);
}

/// Conversion constants for heat capacity units.
pub(crate) mod heat_capacity {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `JoulePerKelvin` unit.
    pub(crate) const JOULE_PER_KELVIN: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `KilojoulePerKelvin` unit.
    pub(crate) const KILOJOULE_PER_KELVIN: UnitDefinition = definition!(1000, 1, 0, 0);
    /// Exact conversion definition for the `JoulePerDegreeCelsius` unit.
    pub(crate) const JOULE_PER_DEGREE_CELSIUS: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `ThermochemicalCaloriePerKelvin`
    /// unit.
    pub(crate) const THERMOCHEMICAL_CALORIE_PER_KELVIN: UnitDefinition =
        definition!(523, 125, 0, 0);
    /// Exact conversion definition for the
    /// `BritishThermalUnitInternationalTablePerDegreeFahrenheit` unit.
    pub(crate) const BRITISH_THERMAL_UNIT_INTERNATIONAL_TABLE_PER_DEGREE_FAHRENHEIT:
        UnitDefinition = definition!(1186938, 625, 0, 0);
}

/// Conversion constants for heat flux density units.
pub(crate) mod heat_flux_density {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `MilliwattPerSquareMeter` unit.
    pub(crate) const MILLIWATT_PER_SQUARE_METER: UnitDefinition = definition!(1, 1000, 0, 0);
    /// Exact conversion definition for the `WattPerSquareMeter` unit.
    pub(crate) const WATT_PER_SQUARE_METER: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `KilowattPerSquareMeter` unit.
    pub(crate) const KILOWATT_PER_SQUARE_METER: UnitDefinition = definition!(1000, 1, 0, 0);
    /// Exact conversion definition for the `WattPerSquareCentimeter` unit.
    pub(crate) const WATT_PER_SQUARE_CENTIMETER: UnitDefinition = definition!(10000, 1, 0, 0);
}

/// Conversion constants for illuminance units.
pub(crate) mod illuminance {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `Lux` unit.
    pub(crate) const LUX: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `Kilolux` unit.
    pub(crate) const KILOLUX: UnitDefinition = definition!(1000, 1, 0, 0);
    /// Exact conversion definition for the `Footcandle` unit.
    pub(crate) const FOOTCANDLE: UnitDefinition = definition!(1562500, 145161, 0, 0);
}

/// Conversion constants for inductance units.
pub(crate) mod inductance {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `Nanohenry` unit.
    pub(crate) const NANOHENRY: UnitDefinition = definition!(1, 1000000000, 0, 0);
    /// Exact conversion definition for the `Microhenry` unit.
    pub(crate) const MICROHENRY: UnitDefinition = definition!(1, 1000000, 0, 0);
    /// Exact conversion definition for the `Millihenry` unit.
    pub(crate) const MILLIHENRY: UnitDefinition = definition!(1, 1000, 0, 0);
    /// Exact conversion definition for the `Henry` unit.
    pub(crate) const HENRY: UnitDefinition = definition!(1, 1, 0, 0);
}

/// Conversion constants for information units, expressed in bytes.
pub(crate) mod information {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `Bit` unit.
    pub(crate) const BIT: UnitDefinition = definition!(1, 8, 0, 0);
    /// Exact conversion definition for the `Byte` unit.
    pub(crate) const BYTE: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `Kilobyte` unit.
    pub(crate) const KILOBYTE: UnitDefinition = definition!(1000, 1, 0, 0);
    /// Exact conversion definition for the `Megabyte` unit.
    pub(crate) const MEGABYTE: UnitDefinition = definition!(1000000, 1, 0, 0);
    /// Exact conversion definition for the `Gigabyte` unit.
    pub(crate) const GIGABYTE: UnitDefinition = definition!(1000000000, 1, 0, 0);
    /// Exact conversion definition for the `Terabyte` unit.
    pub(crate) const TERABYTE: UnitDefinition = definition!(1000000000000, 1, 0, 0);
    /// Exact conversion definition for the `Kibibyte` unit.
    pub(crate) const KIBIBYTE: UnitDefinition = definition!(1024, 1, 0, 0);
    /// Exact conversion definition for the `Mebibyte` unit.
    pub(crate) const MEBIBYTE: UnitDefinition = definition!(1048576, 1, 0, 0);
    /// Exact conversion definition for the `Gibibyte` unit.
    pub(crate) const GIBIBYTE: UnitDefinition = definition!(1073741824, 1, 0, 0);
    /// Exact conversion definition for the `Tebibyte` unit.
    pub(crate) const TEBIBYTE: UnitDefinition = definition!(1099511627776, 1, 0, 0);
}

/// Conversion constants for kinematic viscosity units.
pub(crate) mod kinematic_viscosity {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `SquareMillimeterPerSecond` unit.
    pub(crate) const SQUARE_MILLIMETER_PER_SECOND: UnitDefinition = definition!(1, 1000000, 0, 0);
    /// Exact conversion definition for the `SquareMeterPerSecond` unit.
    pub(crate) const SQUARE_METER_PER_SECOND: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `Stokes` unit.
    pub(crate) const STOKES: UnitDefinition = definition!(1, 10000, 0, 0);
    /// Exact conversion definition for the `Centistokes` unit.
    pub(crate) const CENTISTOKES: UnitDefinition = definition!(1, 1000000, 0, 0);
}

/// Conversion constants for length units.
pub(crate) mod length {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `Nanometer` unit.
    pub(crate) const NANOMETER: UnitDefinition = definition!(1, 1000000000, 0, 0);
    /// Exact conversion definition for the `Micrometer` unit.
    pub(crate) const MICROMETER: UnitDefinition = definition!(1, 1000000, 0, 0);
    /// Exact conversion definition for the `Millimeter` unit.
    pub(crate) const MILLIMETER: UnitDefinition = definition!(1, 1000, 0, 0);
    /// Exact conversion definition for the `Centimeter` unit.
    pub(crate) const CENTIMETER: UnitDefinition = definition!(1, 100, 0, 0);
    /// Exact conversion definition for the `Meter` unit.
    pub(crate) const METER: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `Kilometer` unit.
    pub(crate) const KILOMETER: UnitDefinition = definition!(1000, 1, 0, 0);
    /// Exact conversion definition for the `Inch` unit.
    pub(crate) const INCH: UnitDefinition = definition!(127, 5000, 0, 0);
    /// Exact conversion definition for the `Foot` unit.
    pub(crate) const FOOT: UnitDefinition = definition!(381, 1250, 0, 0);
    /// Exact conversion definition for the `Yard` unit.
    pub(crate) const YARD: UnitDefinition = definition!(1143, 1250, 0, 0);
    /// Exact conversion definition for the `Mile` unit.
    pub(crate) const MILE: UnitDefinition = definition!(201168, 125, 0, 0);
}

/// Conversion constants for luminance units.
pub(crate) mod luminance {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `CandelaPerSquareMeter` unit.
    pub(crate) const CANDELA_PER_SQUARE_METER: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `CandelaPerSquareCentimeter` unit.
    pub(crate) const CANDELA_PER_SQUARE_CENTIMETER: UnitDefinition = definition!(10000, 1, 0, 0);
    /// Exact conversion definition for the `CandelaPerSquareFoot` unit.
    pub(crate) const CANDELA_PER_SQUARE_FOOT: UnitDefinition = definition!(1562500, 145161, 0, 0);
    /// Exact conversion definition for the `Footlambert` unit.
    pub(crate) const FOOTLAMBERT: UnitDefinition =
        definition!(6852518199270781, 2000000000000000, 0, 0);
    /// Exact conversion definition for the `Stilb` unit.
    pub(crate) const STILB: UnitDefinition = definition!(10000, 1, 0, 0);
}

/// Conversion constants for luminous intensity units.
pub(crate) mod luminous_intensity {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `Millicandela` unit.
    pub(crate) const MILLICANDELA: UnitDefinition = definition!(1, 1000, 0, 0);
    /// Exact conversion definition for the `Candela` unit.
    pub(crate) const CANDELA: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `Kilocandela` unit.
    pub(crate) const KILOCANDELA: UnitDefinition = definition!(1000, 1, 0, 0);
}

/// Conversion constants for magnetic field strength units.
pub(crate) mod magnetic_field_strength {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `AmperePerMeter` unit.
    pub(crate) const AMPERE_PER_METER: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `AmperePerCentimeter` unit.
    pub(crate) const AMPERE_PER_CENTIMETER: UnitDefinition = definition!(100, 1, 0, 0);
    /// Exact conversion definition for the `Oersted` unit.
    pub(crate) const OERSTED: UnitDefinition = definition!(7957747154594767, 100000000000000, 0, 0);
}

/// Conversion constants for magnetic flux units.
pub(crate) mod magnetic_flux {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `Microweber` unit.
    pub(crate) const MICROWEBER: UnitDefinition = definition!(1, 1000000, 0, 0);
    /// Exact conversion definition for the `Milliweber` unit.
    pub(crate) const MILLIWEBER: UnitDefinition = definition!(1, 1000, 0, 0);
    /// Exact conversion definition for the `Weber` unit.
    pub(crate) const WEBER: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `Maxwell` unit.
    pub(crate) const MAXWELL: UnitDefinition = definition!(1, 100000000, 0, 0);
}

/// Conversion constants for magnetic flux density units.
pub(crate) mod magnetic_flux_density {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `Nanotesla` unit.
    pub(crate) const NANOTESLA: UnitDefinition = definition!(1, 1000000000, 0, 0);
    /// Exact conversion definition for the `Microtesla` unit.
    pub(crate) const MICROTESLA: UnitDefinition = definition!(1, 1000000, 0, 0);
    /// Exact conversion definition for the `Millitesla` unit.
    pub(crate) const MILLITESLA: UnitDefinition = definition!(1, 1000, 0, 0);
    /// Exact conversion definition for the `Tesla` unit.
    pub(crate) const TESLA: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `Gauss` unit.
    pub(crate) const GAUSS: UnitDefinition = definition!(1, 10000, 0, 0);
}

/// Conversion constants for mass units.
pub(crate) mod mass {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `Microgram` unit.
    pub(crate) const MICROGRAM: UnitDefinition = definition!(1, 1000000000, 0, 0);
    /// Exact conversion definition for the `Milligram` unit.
    pub(crate) const MILLIGRAM: UnitDefinition = definition!(1, 1000000, 0, 0);
    /// Exact conversion definition for the `Gram` unit.
    pub(crate) const GRAM: UnitDefinition = definition!(1, 1000, 0, 0);
    /// Exact conversion definition for the `Kilogram` unit.
    pub(crate) const KILOGRAM: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `Tonne` unit.
    pub(crate) const TONNE: UnitDefinition = definition!(1000, 1, 0, 0);
    /// Exact conversion definition for the `Carat` unit.
    pub(crate) const CARAT: UnitDefinition = definition!(1, 5000, 0, 0);
    /// Exact conversion definition for the `Ounce` unit.
    pub(crate) const OUNCE: UnitDefinition = definition!(45359237, 1600000000, 0, 0);
    /// Exact conversion definition for the `Pound` unit.
    pub(crate) const POUND: UnitDefinition = definition!(45359237, 100000000, 0, 0);
    /// Exact conversion definition for the `TonShort` unit.
    pub(crate) const TON_SHORT: UnitDefinition = definition!(45359237, 50000, 0, 0);
    /// Exact conversion definition for the `TonLong` unit.
    pub(crate) const TON_LONG: UnitDefinition = definition!(317514659, 312500, 0, 0);
}

/// Conversion constants for mass concentration units.
pub(crate) mod mass_concentration {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `MicrogramPerLiter` unit.
    pub(crate) const MICROGRAM_PER_LITER: UnitDefinition = definition!(1, 1000000, 0, 0);
    /// Exact conversion definition for the `MilligramPerLiter` unit.
    pub(crate) const MILLIGRAM_PER_LITER: UnitDefinition = definition!(1, 1000, 0, 0);
    /// Exact conversion definition for the `GramPerLiter` unit.
    pub(crate) const GRAM_PER_LITER: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `KilogramPerCubicMeter` unit.
    pub(crate) const KILOGRAM_PER_CUBIC_METER: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `MilligramPerDeciliter` unit.
    pub(crate) const MILLIGRAM_PER_DECILITER: UnitDefinition = definition!(1, 100, 0, 0);
    /// Exact conversion definition for the `GramPerDeciliter` unit.
    pub(crate) const GRAM_PER_DECILITER: UnitDefinition = definition!(10, 1, 0, 0);
}

/// Conversion constants for mass density units.
pub(crate) mod mass_density {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `KilogramPerCubicMeter` unit.
    pub(crate) const KILOGRAM_PER_CUBIC_METER: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `GramPerCubicMeter` unit.
    pub(crate) const GRAM_PER_CUBIC_METER: UnitDefinition = definition!(1, 1000, 0, 0);
    /// Exact conversion definition for the `GramPerCubicCentimeter` unit.
    pub(crate) const GRAM_PER_CUBIC_CENTIMETER: UnitDefinition = definition!(1000, 1, 0, 0);
    /// Exact conversion definition for the `PoundPerCubicFoot` unit.
    pub(crate) const POUND_PER_CUBIC_FOOT: UnitDefinition =
        definition!(28349523125, 1769802912, 0, 0);
    /// Exact conversion definition for the `PoundPerUsGallon` unit.
    pub(crate) const POUND_PER_US_GALLON: UnitDefinition = definition!(736351250, 6145149, 0, 0);
}

/// Conversion constants for mass rate units.
pub(crate) mod mass_rate {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `MilligramPerSecond` unit.
    pub(crate) const MILLIGRAM_PER_SECOND: UnitDefinition = definition!(1, 1000000, 0, 0);
    /// Exact conversion definition for the `GramPerSecond` unit.
    pub(crate) const GRAM_PER_SECOND: UnitDefinition = definition!(1, 1000, 0, 0);
    /// Exact conversion definition for the `KilogramPerSecond` unit.
    pub(crate) const KILOGRAM_PER_SECOND: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `KilogramPerHour` unit.
    pub(crate) const KILOGRAM_PER_HOUR: UnitDefinition = definition!(1, 3600, 0, 0);
    /// Exact conversion definition for the `TonnePerHour` unit.
    pub(crate) const TONNE_PER_HOUR: UnitDefinition = definition!(5, 18, 0, 0);
    /// Exact conversion definition for the `PoundPerHour` unit.
    pub(crate) const POUND_PER_HOUR: UnitDefinition = definition!(45359237, 360000000000, 0, 0);
}

/// Conversion constants for molality units.
pub(crate) mod molality {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `MolePerKilogram` unit.
    pub(crate) const MOLE_PER_KILOGRAM: UnitDefinition = definition!(1, 1, 0, 0);
}

/// Conversion constants for molar concentration units.
pub(crate) mod molar_concentration {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `NanomolePerLiter` unit.
    pub(crate) const NANOMOLE_PER_LITER: UnitDefinition = definition!(1, 1000000, 0, 0);
    /// Exact conversion definition for the `MicromolePerLiter` unit.
    pub(crate) const MICROMOLE_PER_LITER: UnitDefinition = definition!(1, 1000, 0, 0);
    /// Exact conversion definition for the `MillimolePerLiter` unit.
    pub(crate) const MILLIMOLE_PER_LITER: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `MolePerLiter` unit.
    pub(crate) const MOLE_PER_LITER: UnitDefinition = definition!(1000, 1, 0, 0);
    /// Exact conversion definition for the `MolePerCubicMeter` unit.
    pub(crate) const MOLE_PER_CUBIC_METER: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `ParticlePerMilliliter` unit.
    pub(crate) const PARTICLE_PER_MILLILITER: UnitDefinition =
        definition!(1, 602214076000000000, 0, 0);
}

/// Conversion constants for molar mass units.
pub(crate) mod molar_mass {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `MilligramPerMole` unit.
    pub(crate) const MILLIGRAM_PER_MOLE: UnitDefinition = definition!(1, 1000000, 0, 0);
    /// Exact conversion definition for the `GramPerMole` unit.
    pub(crate) const GRAM_PER_MOLE: UnitDefinition = definition!(1, 1000, 0, 0);
    /// Exact conversion definition for the `KilogramPerMole` unit.
    pub(crate) const KILOGRAM_PER_MOLE: UnitDefinition = definition!(1, 1, 0, 0);
}

/// Conversion constants for molar volume units.
pub(crate) mod molar_volume {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `CubicCentimeterPerMole` unit.
    pub(crate) const CUBIC_CENTIMETER_PER_MOLE: UnitDefinition = definition!(1, 1000000, 0, 0);
    /// Exact conversion definition for the `CubicDecimeterPerMole` unit.
    pub(crate) const CUBIC_DECIMETER_PER_MOLE: UnitDefinition = definition!(1, 1000, 0, 0);
    /// Exact conversion definition for the `CubicMeterPerMole` unit.
    pub(crate) const CUBIC_METER_PER_MOLE: UnitDefinition = definition!(1, 1, 0, 0);
}

/// Conversion constants for power units.
pub(crate) mod power {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `Nanowatt` unit.
    pub(crate) const NANOWATT: UnitDefinition = definition!(1, 1000000000, 0, 0);
    /// Exact conversion definition for the `Microwatt` unit.
    pub(crate) const MICROWATT: UnitDefinition = definition!(1, 1000000, 0, 0);
    /// Exact conversion definition for the `Milliwatt` unit.
    pub(crate) const MILLIWATT: UnitDefinition = definition!(1, 1000, 0, 0);
    /// Exact conversion definition for the `Watt` unit.
    pub(crate) const WATT: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `Kilowatt` unit.
    pub(crate) const KILOWATT: UnitDefinition = definition!(1000, 1, 0, 0);
    /// Exact conversion definition for the `Megawatt` unit.
    pub(crate) const MEGAWATT: UnitDefinition = definition!(1000000, 1, 0, 0);
    /// Exact conversion definition for the `MechanicalHorsepower` unit.
    pub(crate) const MECHANICAL_HORSEPOWER: UnitDefinition =
        definition!(37284993579113511, 50000000000000, 0, 0);
}

/// Conversion constants for pressure units.
pub(crate) mod pressure {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `Nanopascal` unit.
    pub(crate) const NANOPASCAL: UnitDefinition = definition!(1, 1000000000, 0, 0);
    /// Exact conversion definition for the `Micropascal` unit.
    pub(crate) const MICROPASCAL: UnitDefinition = definition!(1, 1000000, 0, 0);
    /// Exact conversion definition for the `Millipascal` unit.
    pub(crate) const MILLIPASCAL: UnitDefinition = definition!(1, 1000, 0, 0);
    /// Exact conversion definition for the `Pascal` unit.
    pub(crate) const PASCAL: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `Hectopascal` unit.
    pub(crate) const HECTOPASCAL: UnitDefinition = definition!(100, 1, 0, 0);
    /// Exact conversion definition for the `Kilopascal` unit.
    pub(crate) const KILOPASCAL: UnitDefinition = definition!(1000, 1, 0, 0);
    /// Exact conversion definition for the `Megapascal` unit.
    pub(crate) const MEGAPASCAL: UnitDefinition = definition!(1000000, 1, 0, 0);
    /// Exact conversion definition for the `Bar` unit.
    pub(crate) const BAR: UnitDefinition = definition!(100000, 1, 0, 0);
    /// Exact conversion definition for the `Millibar` unit.
    pub(crate) const MILLIBAR: UnitDefinition = definition!(100, 1, 0, 0);
    /// Exact conversion definition for the `Atmosphere` unit.
    pub(crate) const ATMOSPHERE: UnitDefinition = definition!(101325, 1, 0, 0);
    /// Exact conversion definition for the `MillimeterOfMercury` unit.
    pub(crate) const MILLIMETER_OF_MERCURY: UnitDefinition = definition!(20265, 152, 0, 0);
    /// Exact conversion definition for the `Psi` unit.
    pub(crate) const PSI: UnitDefinition = definition!(8896443230521, 1290320000, 0, 0);
}

/// Conversion constants for radioactivity units.
pub(crate) mod radioactivity {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `Becquerel` unit.
    pub(crate) const BECQUEREL: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `Kilobecquerel` unit.
    pub(crate) const KILOBECQUEREL: UnitDefinition = definition!(1000, 1, 0, 0);
    /// Exact conversion definition for the `Megabecquerel` unit.
    pub(crate) const MEGABECQUEREL: UnitDefinition = definition!(1000000, 1, 0, 0);
    /// Exact conversion definition for the `Curie` unit.
    pub(crate) const CURIE: UnitDefinition = definition!(37000000000, 1, 0, 0);
    /// Exact conversion definition for the `Millicurie` unit.
    pub(crate) const MILLICURIE: UnitDefinition = definition!(37000000, 1, 0, 0);
    /// Exact conversion definition for the `Microcurie` unit.
    pub(crate) const MICROCURIE: UnitDefinition = definition!(37000, 1, 0, 0);
    /// Exact conversion definition for the `DisintegrationsPerMinute` unit.
    pub(crate) const DISINTEGRATIONS_PER_MINUTE: UnitDefinition = definition!(1, 60, 0, 0);
}

/// Conversion constants for solid angle units.
pub(crate) mod solid_angle {
    use super::TAU_DECIMAL_DENOMINATOR;
    use super::TAU_DECIMAL_NUMERATOR;
    use crate::measure::UnitDefinition;

    /// Denominator used for the finite 28-place square-degree factor.
    const SQUARE_DEGREE_DENOMINATOR: i128 = 10_000_000_000_000_000_000_000_000_000;

    /// Rounded numerator for `(pi / 180)^2` from the NIST DLMF digits of pi.
    const SQUARE_DEGREE_NUMERATOR: i128 = 3_046_174_197_867_085_993_467_435;

    /// Exact conversion definition for the `Steradian` unit.
    pub(crate) const STERADIAN: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `Spat` unit.
    pub(crate) const SPAT: UnitDefinition =
        definition!(2 * TAU_DECIMAL_NUMERATOR, TAU_DECIMAL_DENOMINATOR, 0, 0,);
    /// Exact conversion definition for the `SquareDegree` unit.
    pub(crate) const SQUARE_DEGREE: UnitDefinition =
        definition!(SQUARE_DEGREE_NUMERATOR, SQUARE_DEGREE_DENOMINATOR, 0, 0,);
}

/// Conversion constants for specific heat capacity units.
pub(crate) mod specific_heat_capacity {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `JoulePerKilogramKelvin` unit.
    pub(crate) const JOULE_PER_KILOGRAM_KELVIN: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `KilojoulePerKilogramKelvin` unit.
    pub(crate) const KILOJOULE_PER_KILOGRAM_KELVIN: UnitDefinition = definition!(1000, 1, 0, 0);
    /// Exact conversion definition for the `JoulePerGramDegreeCelsius` unit.
    pub(crate) const JOULE_PER_GRAM_DEGREE_CELSIUS: UnitDefinition = definition!(1000, 1, 0, 0);
    /// Exact conversion definition for the `ThermochemicalCaloriePerGramKelvin`
    /// unit.
    pub(crate) const THERMOCHEMICAL_CALORIE_PER_GRAM_KELVIN: UnitDefinition =
        definition!(4184, 1, 0, 0);
    /// Exact conversion definition for the
    /// `BritishThermalUnitInternationalTablePerPoundDegreeFahrenheit` unit.
    pub(crate) const BRITISH_THERMAL_UNIT_INTERNATIONAL_TABLE_PER_POUND_DEGREE_FAHRENHEIT:
        UnitDefinition = definition!(189910080000, 45359237, 0, 0);
}

/// Conversion constants for specific radioactivity units.
pub(crate) mod specific_radioactivity {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `BecquerelPerKilogram` unit.
    pub(crate) const BECQUEREL_PER_KILOGRAM: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `CuriePerKilogram` unit.
    pub(crate) const CURIE_PER_KILOGRAM: UnitDefinition = definition!(37000000000, 1, 0, 0);
    /// Exact conversion definition for the
    /// `DisintegrationsPerMinutePerKilogram` unit.
    pub(crate) const DISINTEGRATIONS_PER_MINUTE_PER_KILOGRAM: UnitDefinition =
        definition!(1, 60, 0, 0);
}

/// Conversion constants for surface tension units.
pub(crate) mod surface_tension {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `MillinewtonPerMeter` unit.
    pub(crate) const MILLINEWTON_PER_METER: UnitDefinition = definition!(1, 1000, 0, 0);
    /// Exact conversion definition for the `NewtonPerMeter` unit.
    pub(crate) const NEWTON_PER_METER: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `DynePerCentimeter` unit.
    pub(crate) const DYNE_PER_CENTIMETER: UnitDefinition = definition!(1, 1000, 0, 0);
    /// Exact conversion definition for the `JoulePerSquareMeter` unit.
    pub(crate) const JOULE_PER_SQUARE_METER: UnitDefinition = definition!(1, 1, 0, 0);
}

/// Conversion constants for temperature units.
pub(crate) mod temperature {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `Kelvin` unit.
    pub(crate) const KELVIN: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `Celsius` unit.
    pub(crate) const CELSIUS: UnitDefinition = definition!(1, 1, 27315, 2);
    /// Exact conversion definition for the `Fahrenheit` unit.
    pub(crate) const FAHRENHEIT: UnitDefinition = definition!(5, 9, 45967, 2);
    /// Exact conversion definition for the `Rankine` unit.
    pub(crate) const RANKINE: UnitDefinition = definition!(5, 9, 0, 0);
}

/// Conversion constants for temperature interval units.
pub(crate) mod temperature_interval {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `Kelvin` unit.
    pub(crate) const KELVIN: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `Celsius` unit.
    pub(crate) const CELSIUS: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `Fahrenheit` unit.
    pub(crate) const FAHRENHEIT: UnitDefinition = definition!(5, 9, 0, 0);
    /// Exact conversion definition for the `Rankine` unit.
    pub(crate) const RANKINE: UnitDefinition = definition!(5, 9, 0, 0);
}

/// Conversion constants for thermal conductivity units.
pub(crate) mod thermal_conductivity {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `MilliwattPerMeterKelvin` unit.
    pub(crate) const MILLIWATT_PER_METER_KELVIN: UnitDefinition = definition!(1, 1000, 0, 0);
    /// Exact conversion definition for the `WattPerMeterKelvin` unit.
    pub(crate) const WATT_PER_METER_KELVIN: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `KilowattPerMeterKelvin` unit.
    pub(crate) const KILOWATT_PER_METER_KELVIN: UnitDefinition = definition!(1000, 1, 0, 0);
    /// Exact conversion definition for the `WattPerMeterDegreeCelsius` unit.
    pub(crate) const WATT_PER_METER_DEGREE_CELSIUS: UnitDefinition = definition!(1, 1, 0, 0);
}

/// Conversion constants for thermal resistance units.
pub(crate) mod thermal_resistance {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `KelvinPerMilliwatt` unit.
    pub(crate) const KELVIN_PER_MILLIWATT: UnitDefinition = definition!(1000, 1, 0, 0);
    /// Exact conversion definition for the `KelvinPerWatt` unit.
    pub(crate) const KELVIN_PER_WATT: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `KelvinPerKilowatt` unit.
    pub(crate) const KELVIN_PER_KILOWATT: UnitDefinition = definition!(1, 1000, 0, 0);
}

/// Conversion constants for time units.
pub(crate) mod time {
    use crate::measure::UnitDefinition;

    /// Number of nanoseconds in one second.
    pub(crate) const NANOSECONDS_PER_SECOND: i128 = 1_000_000_000;
    /// Number of microseconds in one second.
    pub(crate) const MICROSECONDS_PER_SECOND: i128 = 1_000_000;
    /// Number of milliseconds in one second.
    pub(crate) const MILLISECONDS_PER_SECOND: i128 = 1_000;
    /// Number of seconds in one minute.
    pub(crate) const SECONDS_PER_MINUTE: i128 = 60;
    /// Number of minutes in one hour.
    pub(crate) const MINUTES_PER_HOUR: i128 = 60;
    /// Number of hours in one day.
    pub(crate) const HOURS_PER_DAY: i128 = 24;
    /// Number of days in one common year.
    pub(crate) const DAYS_PER_COMMON_YEAR: i128 = 365;

    /// Exact conversion definition for the `Nanosecond` unit.
    pub(crate) const NANOSECOND: UnitDefinition = definition!(1, NANOSECONDS_PER_SECOND, 0, 0);
    /// Exact conversion definition for the `Microsecond` unit.
    pub(crate) const MICROSECOND: UnitDefinition = definition!(1, MICROSECONDS_PER_SECOND, 0, 0);
    /// Exact conversion definition for the `Millisecond` unit.
    pub(crate) const MILLISECOND: UnitDefinition = definition!(1, MILLISECONDS_PER_SECOND, 0, 0);
    /// Exact conversion definition for the `Second` unit.
    pub(crate) const SECOND: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `Minute` unit.
    pub(crate) const MINUTE: UnitDefinition = definition!(SECONDS_PER_MINUTE, 1, 0, 0);
    /// Exact conversion definition for the `Hour` unit.
    pub(crate) const HOUR: UnitDefinition =
        definition!(SECONDS_PER_MINUTE * MINUTES_PER_HOUR, 1, 0, 0);
    /// Exact conversion definition for the `Day` unit.
    pub(crate) const DAY: UnitDefinition = definition!(
        SECONDS_PER_MINUTE * MINUTES_PER_HOUR * HOURS_PER_DAY,
        1,
        0,
        0,
    );
    /// Exact conversion definition for the `CommonYear365` unit.
    pub(crate) const COMMON_YEAR365: UnitDefinition = definition!(
        SECONDS_PER_MINUTE * MINUTES_PER_HOUR * HOURS_PER_DAY * DAYS_PER_COMMON_YEAR,
        1,
        0,
        0,
    );
}

/// Conversion constants for torque units.
pub(crate) mod torque {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `MillinewtonMeter` unit.
    pub(crate) const MILLINEWTON_METER: UnitDefinition = definition!(1, 1000, 0, 0);
    /// Exact conversion definition for the `NewtonMeter` unit.
    pub(crate) const NEWTON_METER: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `KilonewtonMeter` unit.
    pub(crate) const KILONEWTON_METER: UnitDefinition = definition!(1000, 1, 0, 0);
    /// Exact conversion definition for the `PoundForceFoot` unit.
    pub(crate) const POUND_FORCE_FOOT: UnitDefinition =
        definition!(3389544870828501, 2500000000000000, 0, 0);
    /// Exact conversion definition for the `PoundForceInch` unit.
    pub(crate) const POUND_FORCE_INCH: UnitDefinition =
        definition!(1129848290276167, 10000000000000000, 0, 0);
}

/// Conversion constants for velocity units.
pub(crate) mod velocity {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `MicrometerPerSecond` unit.
    pub(crate) const MICROMETER_PER_SECOND: UnitDefinition = definition!(1, 1000000, 0, 0);
    /// Exact conversion definition for the `MillimeterPerSecond` unit.
    pub(crate) const MILLIMETER_PER_SECOND: UnitDefinition = definition!(1, 1000, 0, 0);
    /// Exact conversion definition for the `CentimeterPerSecond` unit.
    pub(crate) const CENTIMETER_PER_SECOND: UnitDefinition = definition!(1, 100, 0, 0);
    /// Exact conversion definition for the `MeterPerSecond` unit.
    pub(crate) const METER_PER_SECOND: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `KilometerPerHour` unit.
    pub(crate) const KILOMETER_PER_HOUR: UnitDefinition = definition!(5, 18, 0, 0);
    /// Exact conversion definition for the `FootPerSecond` unit.
    pub(crate) const FOOT_PER_SECOND: UnitDefinition = definition!(381, 1250, 0, 0);
    /// Exact conversion definition for the `MilePerHour` unit.
    pub(crate) const MILE_PER_HOUR: UnitDefinition = definition!(1397, 3125, 0, 0);
    /// Exact conversion definition for the `Knot` unit.
    pub(crate) const KNOT: UnitDefinition = definition!(463, 900, 0, 0);
}

/// Conversion constants for volume units.
pub(crate) mod volume {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `CubicMillimeter` unit.
    pub(crate) const CUBIC_MILLIMETER: UnitDefinition = definition!(1, 1000000000, 0, 0);
    /// Exact conversion definition for the `CubicCentimeter` unit.
    pub(crate) const CUBIC_CENTIMETER: UnitDefinition = definition!(1, 1000000, 0, 0);
    /// Exact conversion definition for the `CubicMeter` unit.
    pub(crate) const CUBIC_METER: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `Microliter` unit.
    pub(crate) const MICROLITER: UnitDefinition = definition!(1, 1000000000, 0, 0);
    /// Exact conversion definition for the `Milliliter` unit.
    pub(crate) const MILLILITER: UnitDefinition = definition!(1, 1000000, 0, 0);
    /// Exact conversion definition for the `Liter` unit.
    pub(crate) const LITER: UnitDefinition = definition!(1, 1000, 0, 0);
    /// Exact conversion definition for the `CubicInch` unit.
    pub(crate) const CUBIC_INCH: UnitDefinition = definition!(2048383, 125000000000, 0, 0);
    /// Exact conversion definition for the `CubicFoot` unit.
    pub(crate) const CUBIC_FOOT: UnitDefinition = definition!(55306341, 1953125000, 0, 0);
    /// Exact conversion definition for the `CubicYard` unit.
    pub(crate) const CUBIC_YARD: UnitDefinition = definition!(1493271207, 1953125000, 0, 0);
    /// Exact conversion definition for the `UsFluidOunce` unit.
    pub(crate) const US_FLUID_OUNCE: UnitDefinition = definition!(473176473, 16000000000000, 0, 0);
    /// Exact conversion definition for the `UsCustomaryCup` unit.
    pub(crate) const US_CUSTOMARY_CUP: UnitDefinition = definition!(473176473, 2000000000000, 0, 0);
    /// Exact conversion definition for the `UsLiquidPint` unit.
    pub(crate) const US_LIQUID_PINT: UnitDefinition = definition!(473176473, 1000000000000, 0, 0);
    /// Exact conversion definition for the `UsLiquidQuart` unit.
    pub(crate) const US_LIQUID_QUART: UnitDefinition = definition!(473176473, 500000000000, 0, 0);
    /// Exact conversion definition for the `UsLiquidGallon` unit.
    pub(crate) const US_LIQUID_GALLON: UnitDefinition = definition!(473176473, 125000000000, 0, 0);
}

/// Conversion constants for volume rate units.
pub(crate) mod volume_rate {
    use crate::measure::UnitDefinition;

    /// Exact conversion definition for the `CubicMeterPerSecond` unit.
    pub(crate) const CUBIC_METER_PER_SECOND: UnitDefinition = definition!(1, 1, 0, 0);
    /// Exact conversion definition for the `CubicMeterPerHour` unit.
    pub(crate) const CUBIC_METER_PER_HOUR: UnitDefinition = definition!(1, 3600, 0, 0);
    /// Exact conversion definition for the `MilliliterPerSecond` unit.
    pub(crate) const MILLILITER_PER_SECOND: UnitDefinition = definition!(1, 1000000, 0, 0);
    /// Exact conversion definition for the `LiterPerSecond` unit.
    pub(crate) const LITER_PER_SECOND: UnitDefinition = definition!(1, 1000, 0, 0);
    /// Exact conversion definition for the `LiterPerMinute` unit.
    pub(crate) const LITER_PER_MINUTE: UnitDefinition = definition!(1, 60000, 0, 0);
    /// Exact conversion definition for the `UsGallonPerMinute` unit.
    pub(crate) const US_GALLON_PER_MINUTE: UnitDefinition =
        definition!(157725491, 2500000000000, 0, 0);
}
