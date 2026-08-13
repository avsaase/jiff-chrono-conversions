use chrono_tz_04 as chrono_tz;

mod date;
mod datetime;
mod offset;
mod time;
mod timestamp;
mod timezone;
mod zoned;

#[derive(Debug)]
pub struct ToChronoError;

#[derive(Debug, thiserror::Error)]
#[error("Value out of range for target type")]
pub struct OutOfRangeError;

#[derive(Debug, thiserror::Error)]
#[error("Time zone is not UTC")]
pub struct NotUtcError;

/// Error type for time zone conversion from `jiff::tz::TimeZone` to `chrono_tz::Tz`.
#[derive(Debug, thiserror::Error)]
#[error("Failed to convert time zone")]
pub enum TimeZoneConversionError {
    NoIanaName,
    TimeZoneParse(chrono_tz::ParseError),
}
