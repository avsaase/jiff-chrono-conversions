use chrono_04::{self as chrono};
use chrono_tz_04 as chrono_tz;
use jiff_02 as jiff;

use crate::{Error, ToChrono, ToJiff, TryToChrono, TryToJiff};

/// Convert a `chrono::DateTime<chrono::FixedOffset>` to a `jiff::Zoned`.
///
/// This conversion is fallible because `chrono`'s `DateTime` range is larger than `jiff`'s
/// `Timestamp` range.
impl TryToJiff<jiff::Zoned> for chrono::DateTime<chrono::FixedOffset> {
    fn try_to_jiff(&self) -> Result<jiff::Zoned, Error> {
        let timestamp: jiff::Timestamp = self.with_timezone(&chrono::Utc).try_to_jiff()?;
        let offset = self.offset().to_jiff();
        Ok(timestamp.to_zoned(jiff::tz::TimeZone::fixed(offset)))
    }
}

/// Convert a `jiff::Zoned` to a `chrono::DateTime<chrono::FixedOffset>`.
///
/// This conversion is fallible because `jiff`'s offset range (±25:59:59) is larger than
/// `chrono`'s offset range (±23:59:59).
impl TryToChrono<chrono::DateTime<chrono::FixedOffset>> for jiff::Zoned {
    fn try_to_chrono(&self) -> Result<chrono::DateTime<chrono::FixedOffset>, Error> {
        let utc = self.timestamp().to_chrono();
        let offset = self.offset().try_to_chrono()?;
        Ok(utc.with_timezone(&offset))
    }
}

/// Convert a `chrono::DateTime<chrono::Utc>` to a `jiff::Zoned`.
///
/// This conversion is fallible because `chrono`'s `DateTime` range is larger than `jiff`'s
/// `Timestamp` range.
impl TryToJiff<jiff::Zoned> for chrono::DateTime<chrono::Utc> {
    fn try_to_jiff(&self) -> Result<jiff::Zoned, Error> {
        let timestamp: jiff::Timestamp = self.try_to_jiff()?;
        Ok(timestamp.to_zoned(jiff::tz::TimeZone::UTC))
    }
}

/// Convert a `jiff::Zoned` to a `chrono::DateTime<chrono::Utc>`.
///
/// This conversion is infallible: unlike `chrono::FixedOffset` or `chrono_tz::Tz`, `chrono::Utc`
/// carries no time zone identity of its own, so any instant can always be expressed in it.
impl ToChrono<chrono::DateTime<chrono::Utc>> for jiff::Zoned {
    fn to_chrono(&self) -> chrono::DateTime<chrono::Utc> {
        self.timestamp().to_chrono()
    }
}

/// Convert a `chrono::DateTime<chrono_tz::Tz>` to a `jiff::Zoned`.
///
/// This conversion is fallible because the underlying conversions from
/// `chrono::DateTime<chrono::Utc>` to `jiff::Timestamp` and from `chrono_tz::Tz` to
/// `jiff::tz::TimeZone` are both fallible.
impl TryToJiff<jiff::Zoned> for chrono::DateTime<chrono_tz::Tz> {
    fn try_to_jiff(&self) -> Result<jiff::Zoned, Error> {
        let timestamp: jiff::Timestamp = self.with_timezone(&chrono::Utc).try_to_jiff()?;
        let tz = self.timezone().try_to_jiff()?;
        Ok(timestamp.to_zoned(tz))
    }
}

/// Convert a `jiff::Zoned` to a `chrono::DateTime<chrono_tz::Tz>`.
///
/// This conversion is fallible because the underlying conversion from `jiff::tz::TimeZone` to
/// `chrono_tz::Tz` is fallible.
impl TryToChrono<chrono::DateTime<chrono_tz::Tz>> for jiff::Zoned {
    fn try_to_chrono(&self) -> Result<chrono::DateTime<chrono_tz::Tz>, Error> {
        let utc = self.timestamp().to_chrono();
        let tz = self.time_zone().try_to_chrono()?;
        Ok(utc.with_timezone(&tz))
    }
}
