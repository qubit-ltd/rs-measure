// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact adapters between persisted time measurements and `Duration`.

use std::time::Duration;

use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

use crate::measure::{
    Measurement,
    MeasurementError,
    Time,
    Unit,
};

/// Number of nanoseconds in one second.
const NANOS_PER_SECOND: u128 = 1_000_000_000;

impl From<Duration> for Measurement<Time> {
    /// Converts a non-negative standard duration into exact Decimal seconds.
    ///
    /// # Parameters
    ///
    /// * `duration` - Standard duration to persist as typed seconds.
    ///
    /// # Returns
    ///
    /// An exact time measurement using [`Time::Second`].
    #[inline]
    fn from(duration: Duration) -> Self {
        let total_nanos = u128::from(duration.as_secs()) * NANOS_PER_SECOND
            + u128::from(duration.subsec_nanos());
        let mantissa = i128::try_from(total_nanos)
            .expect("Duration total nanoseconds always fit i128");
        let value = Decimal::try_from_i128_with_scale(mantissa, 9)
            .expect("Duration total nanoseconds always fit Decimal");
        Self::new(value, Time::Second)
    }
}

impl TryFrom<Measurement<Time>> for Duration {
    type Error = MeasurementError;

    /// Converts an exact time measurement into a standard duration.
    ///
    /// # Parameters
    ///
    /// * `measurement` - Time measurement to convert without rounding.
    ///
    /// # Returns
    ///
    /// A standard duration with the same exact nanosecond value.
    ///
    /// # Errors
    ///
    /// Returns [`MeasurementError::NegativeDuration`] for negative values,
    /// [`MeasurementError::SubnanosecondDuration`] when exact nanoseconds are
    /// impossible, [`MeasurementError::DurationOutOfRange`] above
    /// [`Duration::MAX`], or when exact unit conversion exceeds Decimal range.
    fn try_from(measurement: Measurement<Time>) -> Result<Self, Self::Error> {
        let original_value = measurement.value;
        let original_unit = measurement.unit.symbol().to_owned();
        if original_value.is_sign_negative() {
            return Err(MeasurementError::NegativeDuration {
                value: original_value,
                unit: original_unit,
            });
        }

        let nanoseconds = match measurement.convert_to(Time::Nanosecond) {
            Ok(converted) => converted.value.normalize(),
            Err(_) => {
                return Err(MeasurementError::DurationOutOfRange {
                    value: original_value,
                    unit: original_unit,
                });
            }
        };
        if !nanoseconds.fract().is_zero() {
            return Err(MeasurementError::SubnanosecondDuration {
                value: original_value,
                unit: original_unit,
            });
        }

        let total_nanos = nanoseconds
            .to_u128()
            .expect("non-negative integral Decimal always fits u128");
        let maximum_nanos = u128::from(u64::MAX) * NANOS_PER_SECOND
            + u128::from(Duration::MAX.subsec_nanos());
        if total_nanos > maximum_nanos {
            return Err(MeasurementError::DurationOutOfRange {
                value: original_value,
                unit: original_unit,
            });
        }

        let seconds = (total_nanos / NANOS_PER_SECOND) as u64;
        let subsec_nanos = (total_nanos % NANOS_PER_SECOND) as u32;
        Ok(Duration::new(seconds, subsec_nanos))
    }
}
