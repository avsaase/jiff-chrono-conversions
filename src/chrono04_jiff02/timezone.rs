use chrono_04 as chrono;
use chrono_tz_04 as chrono_tz;
use jiff_02 as jiff;

use crate::{ToJiff, TryToChrono, TryToJiff};

use super::{NotUtcError, OutOfRangeError, TimeZoneConversionError};

/// Convert a `jiff::tz::TimeZone` to a `chrono_tz::Tz`.
///
/// This conversion is fallible because `jiff` also supports time zones without an IANA name
/// (e.g. fixed-offset and POSIX time zones), and because `chrono-tz`'s database of IANA names
/// may not exactly match `jiff`'s.
impl TryToChrono<chrono_tz::Tz> for jiff::tz::TimeZone {
    type Error = TimeZoneConversionError;

    fn try_to_chrono(&self) -> Result<chrono_tz::Tz, Self::Error> {
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

    fn try_to_jiff(&self) -> Result<jiff::tz::TimeZone, Self::Error> {
        jiff::tz::TimeZone::get(self.name())
    }
}

/// Convert a `jiff::tz::TimeZone` to a `chrono::FixedOffset`.
///
/// This conversion is fallible because the `jiff::tz::TimeZone` might not represent a fixed offset
/// from UTC.
impl TryToChrono<chrono::FixedOffset> for jiff::tz::TimeZone {
    type Error = OutOfRangeError;

    fn try_to_chrono(&self) -> Result<chrono::FixedOffset, Self::Error> {
        let offset = self.to_fixed_offset().map_err(|_| OutOfRangeError)?;
        offset.try_to_chrono()
    }
}

/// Convert a `chrono::FixedOffset` to a `jiff::tz::TimeZone`.
///
/// This conversion is infallible because `jiff::tz::TimeZone` can always be created from a fixed
/// offset and `chrono`'s offset range (±23:59:59) is a subset of `jiff`'s offset range
/// (±25:59:59).
impl ToJiff<jiff::tz::TimeZone> for chrono::FixedOffset {
    fn to_jiff(&self) -> jiff::tz::TimeZone {
        let offset: jiff::tz::Offset = self.to_jiff();
        jiff::tz::TimeZone::fixed(offset)
    }
}

/// Convert a `jiff::tz::TimeZone` to `chrono::Utc`.
///
/// This conversion is fallible because a `jiff::tz::TimeZone` might not represent UTC (e.g. it
/// could be a different fixed offset, or a non-fixed IANA time zone).
impl TryToChrono<chrono::Utc> for jiff::tz::TimeZone {
    type Error = NotUtcError;

    fn try_to_chrono(&self) -> Result<chrono::Utc, Self::Error> {
        match self.to_fixed_offset() {
            Ok(offset) if offset == jiff::tz::Offset::UTC => Ok(chrono::Utc),
            _ => Err(NotUtcError),
        }
    }
}

/// Convert `chrono::Utc` to a `jiff::tz::TimeZone`.
///
/// This conversion is infallible: `chrono::Utc` always maps to `jiff::tz::TimeZone::UTC`.
impl ToJiff<jiff::tz::TimeZone> for chrono::Utc {
    fn to_jiff(&self) -> jiff::tz::TimeZone {
        jiff::tz::TimeZone::UTC
    }
}
