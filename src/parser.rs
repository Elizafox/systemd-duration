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

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit0, digit1, multispace0, one_of},
    combinator::{all_consuming, complete, cut, map, opt, recognize},
    error::{ErrorKind::TooLarge, ParseError},
    multi::many1,
    sequence::delimited,
    Err::Failure,
    Finish, IResult, Parser,
};

use crate::{
    duration::{Container, Duration},
    error,
};

// Dimensionless unit constants
#[derive(Copy, Clone, Debug)]
enum DurationUnit {
    Year,
    Month,
    Week,
    Day,
    Hour,
    Minute,
    Second,
    Millisecond,
    Microsecond,
    Nanosecond,
}

// NOTE: we don't accept full float syntax. Systemd doesn't, so this isn't a problem.
fn float(input: &str) -> IResult<&str, f64> {
    map(
        recognize((opt(one_of("+-")), opt((digit0, char('.'))), digit1)),
        |s: &str| s.parse::<f64>().unwrap(),
    )
    .parse(input)
}

fn timespan_word(input: &str) -> IResult<&str, &str> {
    // XXX - not fantastic but don't have a better way right now
    recognize(many1(one_of("Macdehiklmnorstuwyµ"))).parse(input)
}

// This is used to get the longest possible match for a string
#[must_use]
fn all_consuming_tag<'a, E>(
    t: &'a str,
) -> impl Parser<&'a str, Output = &'a str, Error = E> + 'a
where
    E: ParseError<&'a str> + 'a,
{
    all_consuming(tag(t))
}

fn timespan_period_years(input: &str) -> IResult<&str, DurationUnit> {
    map(
        alt((
            all_consuming_tag("years"),
            all_consuming_tag("year"),
            all_consuming_tag("yrs"),
            all_consuming_tag("yr"),
            all_consuming_tag("y"),
        )),
        |_| DurationUnit::Year,
    )
    .parse(input)
}

fn timespan_period_months(input: &str) -> IResult<&str, DurationUnit> {
    map(
        alt((
            all_consuming_tag("months"),
            all_consuming_tag("month"),
            all_consuming_tag("mos"),
            all_consuming_tag("mo"),
            all_consuming_tag("M"),
        )),
        |_| DurationUnit::Month,
    )
    .parse(input)
}

fn timespan_period_weeks(input: &str) -> IResult<&str, DurationUnit> {
    map(
        alt((
            all_consuming_tag("weeks"),
            all_consuming_tag("week"),
            all_consuming_tag("wks"),
            all_consuming_tag("wk"),
            all_consuming_tag("w"),
        )),
        |_| DurationUnit::Week,
    )
    .parse(input)
}

fn timespan_period_days(input: &str) -> IResult<&str, DurationUnit> {
    map(
        alt((
            all_consuming_tag("days"),
            all_consuming_tag("day"),
            all_consuming_tag("d"),
        )),
        |_| DurationUnit::Day,
    )
    .parse(input)
}

fn timespan_period_hours(input: &str) -> IResult<&str, DurationUnit> {
    map(
        alt((
            all_consuming_tag("hours"),
            all_consuming_tag("hour"),
            all_consuming_tag("hrs"),
            all_consuming_tag("hr"),
            all_consuming_tag("h"),
        )),
        |_| DurationUnit::Hour,
    )
    .parse(input)
}

fn timespan_period_minutes(input: &str) -> IResult<&str, DurationUnit> {
    map(
        alt((
            all_consuming_tag("minutes"),
            all_consuming_tag("minute"),
            all_consuming_tag("mins"),
            all_consuming_tag("min"),
            all_consuming_tag("m"),
        )),
        |_| DurationUnit::Minute,
    )
    .parse(input)
}

fn timespan_period_seconds(input: &str) -> IResult<&str, DurationUnit> {
    map(
        alt((
            all_consuming_tag("seconds"),
            all_consuming_tag("second"),
            all_consuming_tag("secs"),
            all_consuming_tag("sec"),
            all_consuming_tag("s"),
        )),
        |_| DurationUnit::Second,
    )
    .parse(input)
}

fn timespan_period_milliseconds(input: &str) -> IResult<&str, DurationUnit> {
    map(
        alt((
            all_consuming_tag("milliseconds"),
            all_consuming_tag("millisecond"),
            all_consuming_tag("msecs"),
            all_consuming_tag("msec"),
            all_consuming_tag("ms"),
        )),
        |_| DurationUnit::Millisecond,
    )
    .parse(input)
}

fn timespan_period_microseconds(input: &str) -> IResult<&str, DurationUnit> {
    map(
        alt((
            all_consuming_tag("microseconds"),
            all_consuming_tag("microsecond"),
            all_consuming_tag("µsecs"),
            all_consuming_tag("µsec"),
            all_consuming_tag("µs"),
            all_consuming_tag("µ"),
            all_consuming_tag("usecs"),
            all_consuming_tag("usec"),
            all_consuming_tag("us"),
        )),
        |_| DurationUnit::Microsecond,
    )
    .parse(input)
}

fn timespan_period_nanoseconds(input: &str) -> IResult<&str, DurationUnit> {
    map(
        alt((
            all_consuming_tag("nanoseconds"),
            all_consuming_tag("nanosecond"),
            all_consuming_tag("nsecs"),
            all_consuming_tag("nsec"),
            all_consuming_tag("ns"),
        )),
        |_| DurationUnit::Nanosecond,
    )
    .parse(input)
}

// Match a timespan period, consisting of an entire word
// If the string isn't consumed, this fails.
fn timespan_period(input: &str) -> IResult<&str, DurationUnit> {
    let (input, unit) = timespan_word(input)?;
    let (_, result) = all_consuming(alt((
        timespan_period_years,
        timespan_period_months,
        timespan_period_weeks,
        timespan_period_days,
        timespan_period_hours,
        timespan_period_minutes,
        timespan_period_seconds,
        timespan_period_milliseconds,
        timespan_period_microseconds,
        timespan_period_nanoseconds,
    )))
    .parse(unit)?;

    Ok((input, result))
}

// Returns a fragment of the duration
#[inline(never)]
fn duration_fragment(input: &str) -> IResult<&str, Duration> {
    let (input, count) = delimited(multispace0, float, multispace0).parse(input)?;
    let (input, unit) = timespan_period(input)?;
    let val = match unit {
        DurationUnit::Year => Duration::Year(count),
        DurationUnit::Month => Duration::Month(count),
        DurationUnit::Week => Duration::Week(count),
        DurationUnit::Day => Duration::Day(count),
        DurationUnit::Hour => Duration::Hour(count),
        DurationUnit::Minute => Duration::Minute(count),
        DurationUnit::Second => Duration::Second(count),
        DurationUnit::Millisecond => Duration::Millisecond(count),
        DurationUnit::Microsecond => Duration::Microsecond(count),
        DurationUnit::Nanosecond => {
            // All numbers are specified as floats, and a 52-bit mantissa is more than enough for
            // most nanosecond values, so this is fine.
            #[allow(clippy::cast_precision_loss)]
            if count < i64::MIN as f64 || count > i64::MAX as f64 {
                return Err(Failure(ParseError::from_error_kind(input, TooLarge)));
            }
            #[allow(clippy::cast_possible_truncation)]
            Duration::Nanosecond(count as i64)
        }
    };

    Ok((input, val))
}

// If nothing else is input, just interpret it as seconds.
fn raw_seconds(input: &str) -> IResult<&str, Duration> {
    let (input, seconds) =
        all_consuming(delimited(multispace0, float, multispace0)).parse(input)?;
    Ok((input, Duration::Second(seconds)))
}

fn full_duration(input: &str) -> IResult<&str, Vec<Duration>> {
    all_consuming(many1(duration_fragment)).parse(input)
}

// Parse a duration
fn duration(input: &str) -> IResult<&str, Container> {
    complete(cut(alt((
        map(raw_seconds, |v| Container::new(vec![v])),
        map(full_duration, Container::new),
    ))))
    .parse(input)
}

macro_rules! impl_parse {
    ($modname:ident, $typename:ident) => {
        impl_parse!($modname, $typename, ::$modname::$typename);
    };
    ($modname:ident, $typename:ident, $type:ty) => {
        #[doc = concat!(
            "Parsing systemd-style durations into structs used by [`",
            stringify!($typename),
            "`][",
            stringify!($type), "]"
        )]
        pub mod $modname {
            use super::*;

            #[doc = concat!(
                "Parse a duration string into a [`",
                stringify!($typename),
                "`][",
                stringify!($type),
                "].\n\n",
                "# Errors\n\n",
                "Returns [`error::Error`] if the input string is not a valid duration format\n",
                "or cannot be converted into a [`",
                stringify!($typename),
                "`][",
                stringify!($type),
                "]."
            )]
            #[doc = concat!(
                "Parse a duration string into a [`",
                stringify!($typename),
                "`][",
                stringify!($type),
                "]"
            )]
            pub fn parse(input: &str) -> Result<$type, error::Error> {
                let dur = duration(input).map_err(|e| e.to_owned()).finish()?;
                let ret = dur.1.try_into()?;
                Ok(ret)
            }
        }
    };
}

impl_parse!(stdtime, Duration, std::time::Duration);

#[cfg(feature = "with-chrono")]
impl_parse!(chrono, TimeDelta);

#[cfg(feature = "with-time")]
impl_parse!(time, Duration);
