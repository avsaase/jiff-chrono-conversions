use chrono_04::{self as chrono, Timelike};
use jiff_02 as jiff;

use crate::{ToChrono, ToJiff};

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
