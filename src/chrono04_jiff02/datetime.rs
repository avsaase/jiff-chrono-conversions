use chrono_04 as chrono;
use jiff_02 as jiff;

use crate::{ToChrono, ToJiff, TryToJiff};

/// Convert a `chrono::NaiveDateTime` to a `jiff::civil::DateTime`.
///
/// This conversion is fallible because `chrono`'s date range is larger than `jiff`'s date range.
impl TryToJiff<jiff::civil::DateTime> for chrono::NaiveDateTime {
    type Error = jiff::Error;

    fn try_to_jiff(&self) -> Result<jiff::civil::DateTime, Self::Error> {
        let date = self.date().try_to_jiff()?;
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
