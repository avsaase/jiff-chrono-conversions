use chrono_04 as chrono;
use jiff_02 as jiff;

use crate::{ToChrono, TryToJiff};

/// Convert a `chrono::DateTime<chrono::Utc>` to a `jiff::Timestamp`.
///
/// This conversion is fallible because `chrono`'s `DateTime` range is larger than `jiff`'s
/// `Timestamp` range. It is infallible with respect to leap seconds: like `jiff::civil::Time`,
/// `jiff::Timestamp` does not support leap seconds, so if the `chrono::DateTime` is a leap
/// second, it is converted to the last nanosecond of the second before it.
impl TryToJiff<jiff::Timestamp> for chrono::DateTime<chrono::Utc> {
    type Error = jiff::Error;

    fn try_to_jiff(&self) -> Result<jiff::Timestamp, Self::Error> {
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
