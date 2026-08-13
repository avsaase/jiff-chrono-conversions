use chrono_04::{self as chrono, Datelike};
use jiff_02 as jiff;

use crate::{Error, ToChrono, TryToJiff};

/// Convert a `chrono::NaiveDate` to a `jiff::civil::Date`.
///
/// This conversion is fallible because `chrono`'s date range is larger than `jiff`'s date range.
impl TryToJiff<jiff::civil::Date> for chrono::NaiveDate {
    fn try_to_jiff(&self) -> Result<jiff::civil::Date, Error> {
        let year: i16 = self.year().try_into().map_err(|_| {
            Error::new(format!(
                "chrono year {} is out of range for jiff's i16 year",
                self.year()
            ))
        })?;
        // The casts here are safe because
        // - `chrono`'s month is a u32 in the range [1, 12], which fits in an i8.
        // - `chrono`'s day is a u32 in the range [1, 31], which fits in an i8.
        Ok(jiff::civil::Date::new(
            year,
            self.month() as i8,
            self.day() as i8,
        )?)
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
