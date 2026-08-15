use chrono_04 as chrono;
#[cfg(feature = "chrono-tz-010")]
use chrono_tz_010 as chrono_tz;
use jiff_02 as jiff;

#[cfg(feature = "chrono-tz-010")]
use crate::TryToJiff;
use crate::{Error, ToJiff, TryToChrono};

#[cfg(feature = "chrono-tz-010")]
impl TryToChrono<chrono_tz::Tz> for jiff::tz::TimeZone {
    /// Convert a `jiff::tz::TimeZone` to a `chrono_tz::Tz`.
    ///
    /// This conversion is fallible because `jiff` also supports time zones without an IANA name
    /// (e.g. fixed-offset and POSIX time zones), and because `chrono-tz`'s database of IANA names
    /// may not exactly match `jiff`'s.
    fn try_to_chrono(&self) -> Result<chrono_tz::Tz, Error> {
        let name = self
            .iana_name()
            .ok_or_else(|| Error::new("time zone has no IANA name"))?;
        Ok(name.parse::<chrono_tz::Tz>()?)
    }
}

#[cfg(feature = "chrono-tz-010")]
impl TryToJiff<jiff::tz::TimeZone> for chrono_tz::Tz {
    /// Convert a `chrono_tz::Tz` to a `jiff::tz::TimeZone`.
    ///
    /// This conversion is fallible because `jiff`'s time zone database may not exactly match
    /// `chrono-tz`'s.
    fn try_to_jiff(&self) -> Result<jiff::tz::TimeZone, Error> {
        Ok(jiff::tz::TimeZone::get(self.name())?)
    }
}

impl TryToChrono<chrono::FixedOffset> for jiff::tz::TimeZone {
    /// Convert a `jiff::tz::TimeZone` to a `chrono::FixedOffset`.
    ///
    /// This conversion is fallible because the `jiff::tz::TimeZone` might not represent a fixed
    /// offset from UTC.
    fn try_to_chrono(&self) -> Result<chrono::FixedOffset, Error> {
        let offset = self
            .to_fixed_offset()
            .map_err(|_| Error::new("time zone is not a fixed offset"))?;
        offset.try_to_chrono()
    }
}

impl ToJiff<jiff::tz::TimeZone> for chrono::FixedOffset {
    /// Convert a `chrono::FixedOffset` to a `jiff::tz::TimeZone`.
    ///
    /// This conversion is infallible because `jiff::tz::TimeZone` can always be created from a
    /// fixed offset and `chrono`'s offset range (±23:59:59) is a subset of `jiff`'s offset range
    /// (±25:59:59).
    fn to_jiff(&self) -> jiff::tz::TimeZone {
        let offset: jiff::tz::Offset = self.to_jiff();
        jiff::tz::TimeZone::fixed(offset)
    }
}

impl TryToChrono<chrono::Utc> for jiff::tz::TimeZone {
    /// Convert a `jiff::tz::TimeZone` to `chrono::Utc`.
    ///
    /// This conversion is fallible because a `jiff::tz::TimeZone` might not represent UTC (e.g.
    /// it could be a different fixed offset, or a non-fixed IANA time zone).
    fn try_to_chrono(&self) -> Result<chrono::Utc, Error> {
        match self.to_fixed_offset() {
            Ok(offset) if offset == jiff::tz::Offset::UTC => Ok(chrono::Utc),
            _ => Err(Error::new("time zone is not UTC")),
        }
    }
}

impl ToJiff<jiff::tz::TimeZone> for chrono::Utc {
    /// Convert `chrono::Utc` to a `jiff::tz::TimeZone`.
    ///
    /// This conversion is infallible: `chrono::Utc` always maps to `jiff::tz::TimeZone::UTC`.
    fn to_jiff(&self) -> jiff::tz::TimeZone {
        jiff::tz::TimeZone::UTC
    }
}
