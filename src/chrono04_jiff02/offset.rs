use chrono_04 as chrono;
use jiff_02 as jiff;

use crate::{ToJiff, TryToChrono};

use super::OutOfRangeError;

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

    fn try_to_chrono(&self) -> Result<chrono::FixedOffset, Self::Error> {
        chrono::FixedOffset::east_opt(self.seconds()).ok_or(OutOfRangeError)
    }
}
