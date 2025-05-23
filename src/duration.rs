// SPDX-License-Identifier: CC0-1.0
//
// This file is part of systemd-duration.
//
// To the extent possible under law, the author(s) have dedicated all copyright
// and related and neighboring rights to this software to the public domain
// worldwide. This software is distributed without any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication along
// with this software. If not, see <https://creativecommons.org/publicdomain/zero/1.0/>.

use std::convert::TryFrom;

use crate::error;

/// A measurement of a given span of time.
#[derive(Copy, Clone, Debug)]
pub enum Duration {
    Year(f64),
    Month(f64),
    Week(f64),
    Day(f64),
    Hour(f64),
    Minute(f64),
    Second(f64),
    Millisecond(f64),
    Microsecond(f64),
    Nanosecond(i64),
}

/// A container of durations, which when summed give the total duration.
#[derive(Clone, Debug)]
pub struct Container(Vec<Duration>);

impl Container {
    /// Create a new container object from the given durations.
    #[must_use]
    pub const fn new(durations: Vec<Duration>) -> Self {
        Self(durations)
    }
}

/// Just a place to shove conversion factors.
#[allow(clippy::module_name_repetitions)]
struct Convert;

// Systemd uses 365.25 (Julian average) which has an error of 0.0075 days per year relative to the
// Gregorian calendar, or about one in every 133⅓ years.
//
// For the durations systemd deals with, this is not a practical issue in reality. However,
// because the deviation is small, there's no harm in being more accurate vs. being "incompatible."
impl Convert {
    const SECS_PER_MIN: f64 = 60.0;
    const SECS_PER_HOUR: f64 = 60.0 * Self::SECS_PER_MIN;
    const SECS_PER_DAY: f64 = 24.0 * Self::SECS_PER_HOUR;
    const SECS_PER_WEEK: f64 = 7.0 * Self::SECS_PER_DAY;
    const SECS_PER_MONTH: f64 = 30.436_875_f64 * Self::SECS_PER_DAY;
    const SECS_PER_YEAR: f64 = 365.242_5_f64 * Self::SECS_PER_DAY;
    const NANOS_PER_SEC: f64 = 1_000_000_000.0;
    const NANOS_PER_MILLI: f64 = Self::NANOS_PER_SEC / 1_000.0;
    const NANOS_PER_MICRO: f64 = Self::NANOS_PER_MILLI / 1_000.0;
}

/// Conversions from [`Duration`] to [`std::time::Duration`]
pub mod stdtime {
    use super::{error, Container, Convert, Duration, TryFrom};

    macro_rules! duration_ge_second {
        ($secs_per_interval:expr, $count:expr) => {{
            let sign = ($count).signum();
            if sign <= -1.0 || sign.is_nan() {
                return Err(error::Error::DurationOverflow);
            }

            ::std::time::Duration::from_secs_f64(($secs_per_interval) * ($count))
        }};
    }

    macro_rules! duration_lt_second {
        ($nanos_per_interval:expr, $count:expr) => {{
            let nanos: f64 = ($nanos_per_interval) * ($count);
            if !nanos.is_finite() {
                return Err(error::Error::DurationOverflow);
            }

            let rounded = nanos.round();
            #[allow(clippy::cast_possible_truncation)]
            let int_nanos = rounded as i64;

            // Ensure the conversion didn't silently overflow or truncate
            #[allow(clippy::cast_precision_loss)]
            if (int_nanos as f64 - rounded).abs() > f64::EPSILON {
                return Err(error::Error::DurationOverflow);
            }

            match u64::try_from(int_nanos) {
                Ok(valid) => ::std::time::Duration::from_nanos(valid),
                Err(_) => return Err(error::Error::DurationOverflow),
            }
        }};
    }

    impl TryFrom<Container> for std::time::Duration {
        type Error = error::Error;

        /// Convert a [`Duration`] into an [`std::time::Duration`]
        fn try_from(durations: Container) -> Result<Self, Self::Error> {
            let mut duration_sum = Self::new(0, 0);

            for duration in &durations.0 {
                duration_sum += match duration {
                    Duration::Year(count) => {
                        duration_ge_second!(Convert::SECS_PER_YEAR, count)
                    }
                    Duration::Month(count) => {
                        duration_ge_second!(Convert::SECS_PER_MONTH, count)
                    }
                    Duration::Week(count) => {
                        duration_ge_second!(Convert::SECS_PER_WEEK, count)
                    }
                    Duration::Day(count) => {
                        duration_ge_second!(Convert::SECS_PER_DAY, count)
                    }
                    Duration::Hour(count) => {
                        duration_ge_second!(Convert::SECS_PER_HOUR, count)
                    }
                    Duration::Minute(count) => {
                        duration_ge_second!(Convert::SECS_PER_MIN, count)
                    }
                    Duration::Second(count) => duration_ge_second!(1.0, count),
                    Duration::Millisecond(count) => {
                        duration_lt_second!(Convert::NANOS_PER_MILLI, count)
                    }
                    Duration::Microsecond(count) => {
                        duration_lt_second!(Convert::NANOS_PER_MICRO, count)
                    }
                    Duration::Nanosecond(count) => {
                        if *count < 0 {
                            return Err(error::Error::DurationOverflow);
                        }

                        // Checked above
                        #[allow(clippy::cast_sign_loss)]
                        Self::from_nanos(*count as u64)
                    }
                }
            }

            Ok(duration_sum)
        }
    }
}

/// Conversions from [`Duration`] into [`chrono::TimeDelta`][::chrono::TimeDelta]
#[cfg(feature = "with-chrono")]
pub mod chrono {
    use super::{error, Container, Convert, Duration, TryFrom};

    macro_rules! duration_ge_second {
        ($secs_per_interval:expr, $count:expr) => {{
            let seconds = ($secs_per_interval) * ($count);
            if seconds.is_infinite() || seconds > i64::MAX as f64 || seconds < i64::MIN as f64 {
                return Err(error::Error::DurationOverflow);
            }
            let (seconds, nanos) = (
                seconds.trunc(),
                (seconds - seconds.trunc()) * Convert::NANOS_PER_SEC,
            );
            ::chrono::TimeDelta::new(seconds as i64, nanos as u32).unwrap()
        }};
    }

    macro_rules! duration_lt_second {
        ($nanos_per_interval:expr, $count:expr) => {{
            let nanos = ($nanos_per_interval) * ($count);
            if nanos.is_infinite() || nanos > i64::MAX as f64 || nanos < i64::MIN as f64 {
                return Err(error::Error::DurationOverflow);
            }
            ::chrono::TimeDelta::nanoseconds(nanos.round() as i64)
        }};
    }

    impl TryFrom<Container> for ::chrono::TimeDelta {
        type Error = error::Error;

        /// Convert a [`Duration`] into a [`::chrono::TimeDelta`]
        fn try_from(durations: Container) -> Result<Self, Self::Error> {
            let mut duration_sum = Self::new(0, 0).unwrap();
            for duration in &durations.0 {
                duration_sum += match duration {
                    Duration::Year(count) => {
                        duration_ge_second!(Convert::SECS_PER_YEAR, count)
                    }
                    Duration::Month(count) => {
                        duration_ge_second!(Convert::SECS_PER_MONTH, count)
                    }
                    Duration::Week(count) => {
                        duration_ge_second!(Convert::SECS_PER_WEEK, count)
                    }
                    Duration::Day(count) => {
                        duration_ge_second!(Convert::SECS_PER_DAY, count)
                    }
                    Duration::Hour(count) => {
                        duration_ge_second!(Convert::SECS_PER_HOUR, count)
                    }
                    Duration::Minute(count) => {
                        duration_ge_second!(Convert::SECS_PER_MIN, count)
                    }
                    Duration::Second(count) => duration_ge_second!(1.0f64, count),
                    Duration::Millisecond(count) => {
                        duration_lt_second!(Convert::NANOS_PER_MILLI, count)
                    }
                    Duration::Microsecond(count) => {
                        duration_lt_second!(Convert::NANOS_PER_MICRO, count)
                    }
                    Duration::Nanosecond(count) => Self::nanoseconds(*count),
                };
            }

            Ok(duration_sum)
        }
    }
}

/// Conversions from [`Duration`] into [`::time::Duration`]
#[cfg(feature = "with-time")]
pub mod time {
    use super::{error, Container, Convert, Duration, TryFrom};

    macro_rules! duration_ge_second {
        ($secs_per_interval:expr, $count:expr) => {{
            ::time::Duration::checked_seconds_f64(($secs_per_interval) * ($count))
                .ok_or(error::Error::DurationOverflow)?
        }};
    }

    macro_rules! duration_lt_second {
        ($nanos_per_interval:expr, $count:expr) => {{
            let nanos = ($nanos_per_interval) * ($count);
            if nanos.is_infinite() || nanos > i64::MAX as f64 || nanos < i64::MIN as f64 {
                return Err(error::Error::DurationOverflow);
            }
            ::time::Duration::nanoseconds(nanos.round() as i64)
        }};
    }

    /// Convert a [`Duration`] into a [`::time::Duration`]
    impl TryFrom<Container> for ::time::Duration {
        type Error = error::Error;

        fn try_from(durations: Container) -> Result<Self, Self::Error> {
            let mut duration_sum = Self::new(0, 0);

            for duration in &durations.0 {
                duration_sum += match duration {
                    Duration::Year(count) => {
                        duration_ge_second!(Convert::SECS_PER_YEAR, count)
                    }
                    Duration::Month(count) => {
                        duration_ge_second!(Convert::SECS_PER_MONTH, count)
                    }
                    Duration::Week(count) => {
                        duration_ge_second!(Convert::SECS_PER_WEEK, count)
                    }
                    Duration::Day(count) => {
                        duration_ge_second!(Convert::SECS_PER_DAY, count)
                    }
                    Duration::Hour(count) => {
                        duration_ge_second!(Convert::SECS_PER_HOUR, count)
                    }
                    Duration::Minute(count) => {
                        duration_ge_second!(Convert::SECS_PER_MIN, count)
                    }
                    Duration::Second(count) => duration_ge_second!(1.0, count),
                    Duration::Millisecond(count) => {
                        duration_lt_second!(Convert::NANOS_PER_MILLI, count)
                    }
                    Duration::Microsecond(count) => {
                        duration_lt_second!(Convert::NANOS_PER_MICRO, count)
                    }
                    Duration::Nanosecond(count) => Self::nanoseconds(*count),
                }
            }

            Ok(duration_sum)
        }
    }
}
