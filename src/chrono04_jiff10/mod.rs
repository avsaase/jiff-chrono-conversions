use jiff_10 as jiff;

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
