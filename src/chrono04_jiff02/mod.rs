use chrono_tz_04 as chrono_tz;
use jiff_02 as jiff;

use crate::Error;

mod date;
mod datetime;
mod offset;
mod time;
mod timestamp;
mod timezone;
mod zoned;

impl From<jiff::Error> for Error {
    fn from(err: jiff::Error) -> Self {
        Error::with_source("failed to convert to jiff type", err)
    }
}

impl From<chrono_tz::ParseError> for Error {
    fn from(err: chrono_tz::ParseError) -> Self {
        Error::with_source("failed to convert time zone", err)
    }
}
