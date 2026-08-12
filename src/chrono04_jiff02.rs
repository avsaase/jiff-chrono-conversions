use chrono_04::{self as chrono, Datelike, Timelike};
use chrono_tz_04 as chrono_tz;
use jiff_02 as jiff;

use crate::{ToChrono, ToJiff, TryToChrono, TryToJiff};

#[derive(Debug)]
pub struct ToChronoError;

#[derive(Debug, thiserror::Error)]
#[error("Value out of range for target type")]
pub struct OutOfRangeError;

/* Date types */

/// Convert a `chrono::NaiveDate` to a `jiff::civil::Date`.
///
/// This conversion is fallible because `chrono`'s date range is larger than `jiff`'s date range.
impl TryToJiff<jiff::civil::Date> for chrono::NaiveDate {
    type Error = jiff::Error;

    fn to_jiff(&self) -> Result<jiff::civil::Date, Self::Error> {
        let year: i16 = self.year().try_into().map_err(|_| {
            // Map to a jiff::Error to avoid having to introduce a new error type.
            jiff::Error::from_args(format_args!(
                "chrono year {} is out of range for jiff's i16 year",
                self.year()
            ))
        })?;
        // The casts here are safe because
        // - `chrono`'s month is a u32 in the range [1, 12], which fits in an i8.
        // - `chrono`'s day is a u32 in the range [1, 31], which fits in an i8.
        jiff::civil::Date::new(year, self.month() as i8, self.day() as i8)
    }
}

/// Convert a `jiff::civil::Date` to a `chrono::NaiveDate`.
///
/// This conversion is infallible because `jiff`'s date range is a subset of `chrono`'s date range.
impl ToChrono<chrono::NaiveDate> for jiff::civil::Date {
    fn to_chrono(&self) -> chrono::NaiveDate {
        // The casts here as safe because
        // - `jiff`'s year is an i16 in the range [-9999, 9999], which fits in an i32.
        // - `jiff`'s month is an i8 in the range [1, 12], which fits in a u32.
        // - `jiff`'s day is an i8 in the range [1, 31], which fits in a u32.
        chrono::NaiveDate::from_ymd_opt(self.year() as i32, self.month() as u32, self.day() as u32)
            .expect("Conversion never fails")
    }
}

/* Time types */

/// Convert a `chrono::NaiveTime` to a `jiff::civil::Time`.
///
/// This conversion is infallible with one caveat: `jiff` does not support leap seconds, so if the
/// `chrono::NaiveTime` is a leap second, it will be converted to the last second of the minute.
/// When parsing times, `jiff` will accept leap seconds, but they will be normalized to the last
/// second of the minute so we do the same here.
impl ToJiff<jiff::civil::Time> for chrono::NaiveTime {
    fn to_jiff(&self) -> jiff::civil::Time {
        // The casts here are safe because
        // - `chrono`'s hour is a u32 in the range [0, 23], which fits in an i8.
        // - `chrono`'s minute is a u32 in the range [0, 59], which fits in an i8.
        // - `chrono`'s second is a u32 in the range [0, 59], which fits in an i8.
        // - we clamp the nanoseconds to the range [0, 999_999_999] to avoid leap seconds, which
        // fits in an i32.
        jiff::civil::Time::new(
            self.hour() as i8,
            self.minute() as i8,
            self.second() as i8,
            self.nanosecond().min(999_999_999) as i32,
        )
        .expect("Conversion never fails")
    }
}

/// Convert a `jiff::civil::Time` to a `chrono::NaiveTime`.
///
/// This conversion is infallible because `jiff`'s time range (including its nanosecond
/// precision) is a subset of `chrono`'s time range, and `jiff` has no leap seconds to worry
/// about.
impl ToChrono<chrono::NaiveTime> for jiff::civil::Time {
    fn to_chrono(&self) -> chrono::NaiveTime {
        chrono::NaiveTime::from_hms_nano_opt(
            self.hour() as u32,
            self.minute() as u32,
            self.second() as u32,
            self.subsec_nanosecond() as u32,
        )
        .expect("Conversion never fails")
    }
}

/* DateTime types */

/// Convert a `chrono::NaiveDateTime` to a `jiff::civil::DateTime`.
///
/// This conversion is fallible because `chrono`'s date range is larger than `jiff`'s date range.
impl TryToJiff<jiff::civil::DateTime> for chrono::NaiveDateTime {
    type Error = jiff::Error;

    fn to_jiff(&self) -> Result<jiff::civil::DateTime, Self::Error> {
        let date = self.date().to_jiff()?;
        let time = self.time().to_jiff();
        let date_time = jiff::civil::DateTime::from_parts(date, time);
        Ok(date_time)
    }
}

/// Convert a `jiff::civil::DateTime` to a `chrono::NaiveDateTime`.
///
/// This conversion is infallible because the underlying conversions from `jiff::civil::Date`
/// to `chrono::NaiveDate` and from `jiff::civil::Time` to `chrono::NaiveTime` are both infallible.
impl ToChrono<chrono::NaiveDateTime> for jiff::civil::DateTime {
    fn to_chrono(&self) -> chrono::NaiveDateTime {
        let date = self.date().to_chrono();
        let time = self.time().to_chrono();
        chrono::NaiveDateTime::new(date, time)
    }
}

/* Timestamp types */

/// Convert a `chrono::DateTime<chrono::Utc>` to a `jiff::Timestamp`.
///
/// This conversion is fallible because `chrono`'s `DateTime` range is larger than `jiff`'s
/// `Timestamp` range. It is infallible with respect to leap seconds: like `jiff::civil::Time`,
/// `jiff::Timestamp` does not support leap seconds, so if the `chrono::DateTime` is a leap
/// second, it is converted to the last nanosecond of the second before it.
impl TryToJiff<jiff::Timestamp> for chrono::DateTime<chrono::Utc> {
    type Error = jiff::Error;

    fn to_jiff(&self) -> Result<jiff::Timestamp, Self::Error> {
        jiff::Timestamp::new(
            self.timestamp(),
            self.timestamp_subsec_nanos().min(999_999_999) as i32,
        )
    }
}

/// Convert a `jiff::Timestamp` to a `chrono::DateTime<chrono::Utc>`.
///
/// This conversion is infallible because `jiff`'s `Timestamp` range is a subset of `chrono`'s
/// `DateTime` range.
impl ToChrono<chrono::DateTime<chrono::Utc>> for jiff::Timestamp {
    fn to_chrono(&self) -> chrono::DateTime<chrono::Utc> {
        // `jiff`'s `second` and `subsec_nanosecond` share the same sign (both negative for
        // timestamps before the Unix epoch), whereas `chrono::DateTime::from_timestamp` expects
        // a non-negative nanosecond count.
        let mut secs = self.as_second();
        let mut nanos = self.subsec_nanosecond();
        if nanos < 0 {
            secs -= 1;
            nanos += 1_000_000_000;
        }
        chrono::DateTime::from_timestamp(secs, nanos as u32).expect("Conversion never fails")
    }
}

/* Offset types */

/// Convert a `chrono::FixedOffset` to a `jiff::tz::Offset`.
///
/// This conversion is infallible because `chrono`'s offset range (±23:59:59) is a subset of
/// `jiff`'s offset range (±25:59:59).
impl ToJiff<jiff::tz::Offset> for chrono::FixedOffset {
    fn to_jiff(&self) -> jiff::tz::Offset {
        jiff::tz::Offset::from_seconds(self.local_minus_utc()).expect("Conversion never fails")
    }
}

/// Convert a `jiff::tz::Offset` to a `chrono::FixedOffset`.
///
/// This conversion is fallible because `jiff`'s offset range (±25:59:59) is larger than
/// `chrono`'s offset range (±23:59:59).
impl TryToChrono<chrono::FixedOffset> for jiff::tz::Offset {
    type Error = OutOfRangeError;

    fn to_chrono(&self) -> Result<chrono::FixedOffset, Self::Error> {
        chrono::FixedOffset::east_opt(self.seconds()).ok_or(OutOfRangeError)
    }
}

/* DateTime with time zone types */

/// Convert a `chrono::DateTime<chrono::FixedOffset>` to a `jiff::Zoned`.
///
/// This conversion is fallible because `chrono`'s `DateTime` range is larger than `jiff`'s
/// `Timestamp` range.
impl TryToJiff<jiff::Zoned> for chrono::DateTime<chrono::FixedOffset> {
    type Error = jiff::Error;

    fn to_jiff(&self) -> Result<jiff::Zoned, Self::Error> {
        let timestamp = self.with_timezone(&chrono::Utc).to_jiff()?;
        let offset = self.offset().to_jiff();
        Ok(timestamp.to_zoned(jiff::tz::TimeZone::fixed(offset)))
    }
}

/// Convert a `jiff::Zoned` to a `chrono::DateTime<chrono::FixedOffset>`.
///
/// This conversion is fallible because `jiff`'s offset range (±25:59:59) is larger than
/// `chrono`'s offset range (±23:59:59).
impl TryToChrono<chrono::DateTime<chrono::FixedOffset>> for jiff::Zoned {
    type Error = OutOfRangeError;

    fn to_chrono(&self) -> Result<chrono::DateTime<chrono::FixedOffset>, Self::Error> {
        let utc = self.timestamp().to_chrono();
        let offset = self.offset().to_chrono()?;
        Ok(utc.with_timezone(&offset))
    }
}

/* Time zone types */

/// Error type for time zone conversion from `jiff::tz::TimeZone` to `chrono_tz::Tz`.
#[derive(Debug, thiserror::Error)]
#[error("Failed to convert time zone")]
pub enum TimeZoneConversionError {
    NoIanaName,
    TimeZoneParse(chrono_tz::ParseError),
}

/// Convert a `jiff::tz::TimeZone` to a `chrono_tz::Tz`.
///
/// This conversion is fallible because `jiff` also supports time zones without an IANA name
/// (e.g. fixed-offset and POSIX time zones), and because `chrono-tz`'s database of IANA names
/// may not exactly match `jiff`'s.
impl TryToChrono<chrono_tz::Tz> for jiff::tz::TimeZone {
    type Error = TimeZoneConversionError;

    fn to_chrono(&self) -> Result<chrono_tz::Tz, Self::Error> {
        self.iana_name()
            .ok_or(TimeZoneConversionError::NoIanaName)?
            .parse::<chrono_tz::Tz>()
            .map_err(|e| TimeZoneConversionError::TimeZoneParse(e))
    }
}

/// Convert a `chrono_tz::Tz` to a `jiff::tz::TimeZone`.
///
/// This conversion is fallible because `jiff`'s time zone database may not exactly match
/// `chrono-tz`'s.
impl TryToJiff<jiff::tz::TimeZone> for chrono_tz::Tz {
    type Error = jiff::Error;

    fn to_jiff(&self) -> Result<jiff::tz::TimeZone, Self::Error> {
        jiff::tz::TimeZone::get(self.name())
    }
}
