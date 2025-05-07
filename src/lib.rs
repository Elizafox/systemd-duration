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

//! systemd-duration is a library that parses [systemd-style durations].
//!
//! It can parse durations into the following formats:
//! * [`time::Duration`][::time::Duration] (with the `with-time` feature)
//! * [`chrono::TimeDelta`][::chrono::TimeDelta] (with the `with-chrono` feature)
//! * [`std::time::Duration`]
//!
//! It uses the [`nom`] library to parse durations.
//!
//! # Example
//! ```
//! let td = systemd_duration::stdtime::parse("1d3s").expect("Could not parse duration");
//! assert_eq!(td, std::time::Duration::from_secs(86403));
//! ```
//!
//! [systemd-style durations]: https://www.freedesktop.org/software/systemd/man/latest/systemd.time.html

#![warn(clippy::style)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]

#[cfg(feature = "with-chrono")]
pub mod chrono;
pub mod duration;
pub mod error;
pub mod parser;
pub mod stdtime;
#[cfg(feature = "with-time")]
pub mod time;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stdtime_duration_parse_year() {
        use std::time;

        let duration_compare = time::Duration::from_secs(31556952);

        if let Ok(duration) = parser::stdtime::parse("1 years") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 year") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 yrs") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 yr") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 y") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1y") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("52.1775w") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("365d5h49m12s") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("365.2425d") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }
    }

    #[test]
    fn test_stdtime_duration_parse_month() {
        use std::time;

        let duration_compare = time::Duration::from_secs(2629746);

        if let Ok(duration) = parser::stdtime::parse("1 months") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 month") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 mos") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 mo") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 M") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("30d10h29m6s") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }
    }

    #[test]
    fn test_stdtime_duration_parse_week() {
        use std::time;

        let duration_compare = time::Duration::from_secs(604800);

        if let Ok(duration) = parser::stdtime::parse("1 weeks") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 week") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 w") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("7d") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("168h") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("10080m") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }
    }

    #[test]
    fn test_stdtime_duration_parse_day() {
        use std::time;

        let duration_compare = time::Duration::from_secs(86400);

        if let Ok(duration) = parser::stdtime::parse("1 days") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 day") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 d") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("24h") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1440m") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("86400s") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }
    }

    #[test]
    fn test_stdtime_duration_parse_hour() {
        use std::time;

        let duration_compare = time::Duration::from_secs(3600);

        if let Ok(duration) = parser::stdtime::parse("1 hours") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 hour") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 hrs") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 hr") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 h") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("60m") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("3600s") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("3600000ms") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }
    }

    #[test]
    fn test_stdtime_duration_parse_minute() {
        use std::time;

        let duration_compare = time::Duration::from_secs(60);

        if let Ok(duration) = parser::stdtime::parse("1 minutes") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 minute") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 mins") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 min") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 m") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("60s") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("60000ms") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }
    }

    #[test]
    fn test_stdtime_duration_parse_second() {
        use std::time;

        let duration_compare = time::Duration::from_secs(1);

        if let Ok(duration) = parser::stdtime::parse("1 seconds") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 second") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 s") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 sec") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1000ms") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1000000µs") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1000000000ns") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }
    }

    #[test]
    fn test_stdtime_duration_parse_millisecond() {
        use std::time;

        let duration_compare = time::Duration::from_millis(1);

        if let Ok(duration) = parser::stdtime::parse("1 milliseconds") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 millisecond") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 msecs") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 msec") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 ms") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1000µs") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1000000ns") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }
    }

    #[test]
    fn test_stdtime_duration_parse_microsecond() {
        use std::time;

        let duration_compare = time::Duration::from_micros(1);

        if let Ok(duration) = parser::stdtime::parse("1 microseconds") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 microsecond") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 µsecs") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 µsec") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 µs") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 usecs") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 usec") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 us") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1000ns") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }
    }

    #[test]
    fn test_stdtime_duration_parse_nanosecond() {
        use std::time;

        let duration_compare = time::Duration::from_nanos(1);

        if let Ok(duration) = parser::stdtime::parse("1 nanoseconds") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 nanosecond") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("1 ns") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }
    }

    #[test]
    fn test_stdtime_duration_negative_invalid() {
        assert!(parser::stdtime::parse("-30d").is_err());
    }

    #[test]
    fn test_time_duration_parse_year() {
        let duration_compare = ::time::Duration::weeks(52)
            + ::time::Duration::days(1)
            + ::time::Duration::hours(5)
            + ::time::Duration::minutes(49)
            + ::time::Duration::seconds(12);

        if let Ok(duration) = parser::time::parse("1 years") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 year") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 yrs") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 yr") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 y") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1y") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("52.1775w") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("365d5h49m12s") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("365.2425d") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }
    }

    #[test]
    fn test_time_duration_parse_month() {
        let duration_compare = ::time::Duration::weeks(4)
            + ::time::Duration::days(2)
            + ::time::Duration::hours(10)
            + ::time::Duration::minutes(29)
            + ::time::Duration::seconds(6);

        if let Ok(duration) = parser::time::parse("1 months") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 month") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 mos") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 mo") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 M") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("30d10h29m6s") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }
    }

    #[test]
    fn test_time_duration_parse_week() {
        let duration_compare = ::time::Duration::weeks(1);

        if let Ok(duration) = parser::time::parse("1 weeks") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 week") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 w") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("7d") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("168h") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("10080m") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }
    }

    #[test]
    fn test_time_duration_parse_day() {
        let duration_compare = ::time::Duration::days(1);

        if let Ok(duration) = parser::time::parse("1 days") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 day") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 d") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("24h") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1440m") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("86400s") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }
    }

    #[test]
    fn test_time_duration_parse_hour() {
        let duration_compare = ::time::Duration::hours(1);

        if let Ok(duration) = parser::time::parse("1 hours") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 hour") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 hrs") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 hr") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 h") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("60m") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("3600s") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("3600000ms") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }
    }

    #[test]
    fn test_time_duration_parse_minute() {
        let duration_compare = ::time::Duration::minutes(1);

        if let Ok(duration) = parser::time::parse("1 minutes") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 minute") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 mins") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 min") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 m") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("60s") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("60000ms") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }
    }

    #[test]
    fn test_time_duration_parse_second() {
        let duration_compare = ::time::Duration::seconds(1);

        if let Ok(duration) = parser::time::parse("1 seconds") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 second") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 s") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 sec") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1000ms") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }
    }

    #[test]
    fn test_time_duration_parse_millisecond() {
        let duration_compare = ::time::Duration::milliseconds(1);

        if let Ok(duration) = parser::time::parse("1 milliseconds") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 millisecond") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 msecs") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 msec") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 ms") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1000µs") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1000000ns") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }
    }

    #[test]
    fn test_time_duration_parse_microsecond() {
        let duration_compare = ::time::Duration::microseconds(1);

        if let Ok(duration) = parser::time::parse("1 microseconds") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 microsecond") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 µsecs") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 µsec") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 µs") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 usecs") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 usec") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 us") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1000ns") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }
    }

    #[test]
    fn test_time_duration_parse_nanosecond() {
        let duration_compare = ::time::Duration::nanoseconds(1);

        if let Ok(duration) = parser::time::parse("1 nanoseconds") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 nanosecond") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::time::parse("1 ns") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }
    }

    #[test]
    fn test_time_duration_negative() {
        let duration_compare = ::time::Duration::nanoseconds(-1) + ::time::Duration::seconds(-1);
        if let Ok(duration) = parser::time::parse("-1s-1ns") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }
    }

    #[test]
    fn test_chrono_duration_parse_year() {
        let duration_compare = ::chrono::TimeDelta::weeks(52)
            + ::chrono::TimeDelta::days(1)
            + ::chrono::TimeDelta::hours(5)
            + ::chrono::TimeDelta::minutes(49)
            + ::chrono::TimeDelta::seconds(12);

        if let Ok(duration) = parser::chrono::parse("1 years") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 year") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 yrs") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 yr") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 y") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1y") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("52.1775w") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("365d5h49m12s") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("365.2425d") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }
    }

    #[test]
    fn test_chrono_duration_parse_month() {
        let duration_compare = ::chrono::TimeDelta::weeks(4)
            + ::chrono::TimeDelta::days(2)
            + ::chrono::TimeDelta::hours(10)
            + ::chrono::TimeDelta::minutes(29)
            + ::chrono::TimeDelta::seconds(6);

        if let Ok(duration) = parser::chrono::parse("1 months") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 month") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 mos") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 mo") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 M") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("30d10h29m6s") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }
    }

    #[test]
    fn test_chrono_duration_parse_week() {
        let duration_compare = ::chrono::TimeDelta::weeks(1);

        if let Ok(duration) = parser::chrono::parse("1 weeks") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 week") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 w") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("7d") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("168h") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("10080m") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }
    }

    #[test]
    fn test_chrono_duration_parse_day() {
        let duration_compare = ::chrono::TimeDelta::days(1);

        if let Ok(duration) = parser::chrono::parse("1 days") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 day") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 d") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("24h") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1440m") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("86400s") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }
    }

    #[test]
    fn test_chrono_duration_parse_hour() {
        let duration_compare = ::chrono::TimeDelta::hours(1);

        if let Ok(duration) = parser::chrono::parse("1 hours") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 hour") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 hrs") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 hr") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 h") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("60m") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("3600s") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("3600000ms") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }
    }

    #[test]
    fn test_chrono_duration_parse_minute() {
        let duration_compare = ::chrono::TimeDelta::minutes(1);

        if let Ok(duration) = parser::chrono::parse("1 minutes") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 minute") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 mins") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 min") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 m") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("60s") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("60000ms") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }
    }

    #[test]
    fn test_chrono_duration_parse_second() {
        let duration_compare = ::chrono::TimeDelta::seconds(1);

        if let Ok(duration) = parser::chrono::parse("1 seconds") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 second") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 s") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 sec") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1000ms") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1000000us") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1000000000ns") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }
    }

    #[test]
    fn test_chrono_duration_parse_millisecond() {
        let duration_compare = ::chrono::TimeDelta::milliseconds(1);

        if let Ok(duration) = parser::chrono::parse("1 milliseconds") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 millisecond") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 msecs") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 msec") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 ms") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1000µs") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1000000ns") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }
    }

    #[test]
    fn test_chrono_duration_parse_microsecond() {
        let duration_compare = ::chrono::TimeDelta::microseconds(1);

        if let Ok(duration) = parser::chrono::parse("1 microseconds") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 microsecond") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 µsecs") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 µsec") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 µs") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 usecs") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 usec") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 us") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1000ns") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }
    }

    #[test]
    fn test_chrono_duration_parse_nanosecond() {
        let duration_compare = ::chrono::TimeDelta::nanoseconds(1);

        if let Ok(duration) = parser::chrono::parse("1 nanoseconds") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 nanosecond") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::chrono::parse("1 ns") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }
    }

    #[test]
    fn test_chrono_duration_negative() {
        let duration_compare =
            ::chrono::TimeDelta::nanoseconds(-1) + ::chrono::TimeDelta::seconds(-1);
        if let Ok(duration) = parser::chrono::parse("-1s-1ns") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }
    }

    #[test]
    fn test_duration_fractional() {
        use std::time;

        let duration_compare = time::Duration::from_nanos(30_001);

        if let Ok(duration) = parser::stdtime::parse("30.001µs") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }

        if let Ok(duration) = parser::stdtime::parse("0.000030001s") {
            assert_eq!(duration_compare, duration);
        } else {
            panic!("Parse failure");
        }
    }

    #[test]
    fn test_duration_invalid() {
        assert!(parser::stdtime::parse("30p").is_err());
    }
}
